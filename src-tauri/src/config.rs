use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub daemon: DaemonConfig,
    pub ai: AiConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// HTTP port for the IPC server. 0 = OS-assigned (useful in tests).
    pub http_port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub default_routing: RoutingPreference,
    pub ollama_base_url: String,
    pub claude_api_key: Option<String>,
    pub default_local_model: String,
    pub default_embed_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoutingPreference {
    #[default]
    Local,
    Cloud,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
}

impl Config {
    pub fn load() -> crate::error::Result<Self> {
        if std::env::var("DAEMON_TEST_MODE").is_ok() {
            return Ok(Self::test_default());
        }
        Ok(Self {
            daemon: DaemonConfig {
                http_port: std::env::var("DAEMON_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(45678),
                host: "127.0.0.1".to_string(),
            },
            ai: AiConfig {
                default_routing: RoutingPreference::Local,
                ollama_base_url: std::env::var("OLLAMA_BASE_URL")
                    .unwrap_or_else(|_| "http://localhost:11434".to_string()),
                claude_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
                default_local_model: std::env::var("DEFAULT_LOCAL_MODEL")
                    .unwrap_or_else(|_| "llama3.2".to_string()),
                default_embed_model: "nomic-embed-text".to_string(),
            },
            storage: StorageConfig {
                data_dir: Self::default_data_dir(),
            },
        })
    }

    pub fn test_default() -> Self {
        Self {
            daemon: DaemonConfig {
                http_port: 0,
                host: "127.0.0.1".to_string(),
            },
            ai: AiConfig {
                default_routing: RoutingPreference::Local,
                ollama_base_url: "http://localhost:11434".to_string(),
                claude_api_key: None,
                default_local_model: "mock-model".to_string(),
                default_embed_model: "mock-embed".to_string(),
            },
            storage: StorageConfig {
                data_dir: std::env::temp_dir()
                    .join(format!("cognition-test-{}", std::process::id())),
            },
        }
    }

    pub fn db_path(&self) -> PathBuf {
        self.storage.data_dir.join("cognition.db")
    }

    fn default_data_dir() -> PathBuf {
        ProjectDirs::from("com", "cognition-os", "CognitionDaemon")
            .map(|d| d.data_local_dir().to_path_buf())
            .unwrap_or_else(|| {
                std::env::var("HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(".cognition-daemon")
            })
    }
}
