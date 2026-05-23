pub mod l0;
pub mod l1;
pub mod l2;

use std::sync::Arc;

use crate::db::Db;

pub use l0::L0Store;
pub use l1::L1Store;
pub use l2::L2Store;

/// Unified memory facade — engines access all layers through this struct.
#[derive(Clone)]
pub struct MemoryStore {
    pub l0: Arc<L0Store>,
    pub l1: Arc<L1Store>,
    pub l2: Arc<L2Store>,
}

impl MemoryStore {
    pub fn new(db: Db) -> Self {
        Self {
            l0: Arc::new(L0Store::new(db.clone())),
            l1: Arc::new(L1Store::new(db.clone())),
            l2: Arc::new(L2Store::new(db)),
        }
    }
}
