/// L2 — semantic vector index.
/// Embeddings stored as raw f32 BLOBs; cosine similarity computed in Rust.
/// ADR-0002 D-3: pure-Rust approach for Phase 1; sqlite-vec drop-in planned in ADR-0003.
use chrono::Utc;
use ulid::Ulid;

use crate::db::Db;

#[derive(Debug, Clone)]
pub struct EmbeddingEntry {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub chunk_index: u32,
    pub content: String,
    pub embedding: Vec<f32>,
    pub model: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct SimilarityResult {
    pub entry: EmbeddingEntry,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct EntityFilter {
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
}

pub struct L2Store {
    db: Db,
}

impl L2Store {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn insert_embedding(
        &self,
        entity_type: &str,
        entity_id: &str,
        chunk_index: u32,
        content: &str,
        embedding: Vec<f32>,
        model: &str,
    ) -> crate::error::Result<EmbeddingEntry> {
        let id = Ulid::new().to_string();
        let now = Utc::now().timestamp_millis();
        let blob = f32_slice_to_bytes(&embedding);
        let dims = embedding.len() as i64;
        let entry = EmbeddingEntry {
            id: id.clone(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            chunk_index,
            content: content.to_string(),
            embedding,
            model: model.to_string(),
            created_at: now,
        };

        let et = entity_type.to_string();
        let eid = entity_id.to_string();
        let c = content.to_string();
        let m = model.to_string();

        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO embeddings
                        (id, entity_type, entity_id, chunk_index, content, embedding, dimensions, model, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    rusqlite::params![id, et, eid, chunk_index, c, blob, dims, m, now],
                )?;
                Ok(())
            })
            .await?;

        Ok(entry)
    }

    /// Linear scan cosine similarity search. Sufficient for Phase 1 data volumes.
    pub async fn search_similar(
        &self,
        query: Vec<f32>,
        top_k: usize,
        filter: Option<EntityFilter>,
    ) -> crate::error::Result<Vec<SimilarityResult>> {
        let rows = self
            .db
            .conn
            .call(move |conn| {
                let (where_clause, params_vec) = build_filter_clause(&filter);
                let sql = format!(
                    "SELECT id, entity_type, entity_id, chunk_index, content, embedding, model, created_at
                     FROM embeddings{where_clause}"
                );
                let mut stmt = conn.prepare(&sql)?;

                let mut results: Vec<(EmbeddingEntry, f32)> = {
                    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
                        let blob: Vec<u8> = row.get(5)?;
                        let embedding = bytes_to_f32_vec(&blob);
                        Ok(EmbeddingEntry {
                            id: row.get(0)?,
                            entity_type: row.get(1)?,
                            entity_id: row.get(2)?,
                            chunk_index: row.get::<_, i64>(3)? as u32,
                            content: row.get(4)?,
                            embedding,
                            model: row.get(6)?,
                            created_at: row.get(7)?,
                        })
                    })?;
                    rows.filter_map(|r| r.ok())
                        .map(|entry| {
                            let score = cosine_similarity(&query, &entry.embedding);
                            (entry, score)
                        })
                        .collect()
                };

                results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                results.truncate(top_k);
                Ok(results)
            })
            .await?;

        Ok(rows
            .into_iter()
            .map(|(entry, score)| SimilarityResult { entry, score })
            .collect())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn f32_slice_to_bytes(v: &[f32]) -> Vec<u8> {
    v.iter()
        .flat_map(|f| f.to_le_bytes())
        .collect()
}

fn bytes_to_f32_vec(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

fn build_filter_clause(filter: &Option<EntityFilter>) -> (String, Vec<String>) {
    match filter {
        None => (String::new(), vec![]),
        Some(f) => {
            let mut clauses = vec![];
            let mut params = vec![];
            if let Some(et) = &f.entity_type {
                clauses.push(format!("entity_type = ?{}", clauses.len() + 1));
                params.push(et.clone());
            }
            if let Some(eid) = &f.entity_id {
                clauses.push(format!("entity_id = ?{}", clauses.len() + 1));
                params.push(eid.clone());
            }
            if clauses.is_empty() {
                (String::new(), vec![])
            } else {
                (format!(" WHERE {}", clauses.join(" AND ")), params)
            }
        }
    }
}
