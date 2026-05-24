pub mod ai;
pub mod config;
pub mod crypto;
pub mod db;
pub mod engines;
pub mod error;
pub mod ipc;
pub mod memory;
pub mod observability;
pub mod sync;

use std::sync::Arc;

use config::Config;
use db::Db;
use engines::{ArtifactEngine, CapabilityEngine, MentorEngine, TrustEngine};
use sync::SyncCoordinator;
use memory::MemoryStore;
use observability::AuditLog;
use tracing::info;

use ai::{
    providers::{
        claude::ClaudeProvider, mock::MockProvider, ollama::OllamaProvider, ProviderRegistry,
    },
    Orchestrator,
};

/// Shared application state injected into every Axum handler via `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Arc<Db>,
    pub memory: Arc<MemoryStore>,
    pub orchestrator: Arc<Orchestrator>,
    pub audit: Arc<AuditLog>,
    pub mentor: Arc<MentorEngine>,
    pub capability: Arc<CapabilityEngine>,
    pub artifact: Arc<ArtifactEngine>,
    pub trust: Arc<TrustEngine>,
    pub sync_coordinator: Arc<SyncCoordinator>,
}

impl AppState {
    pub async fn new(config: Config) -> error::Result<Self> {
        let db = Arc::new(Db::open(&config.db_path()).await?);
        let memory = Arc::new(MemoryStore::new((*db).clone()));
        let audit = Arc::new(AuditLog::new((*db).clone()));

        let registry = build_provider_registry(&config);
        let orchestrator = Arc::new(Orchestrator::new(
            Arc::new(registry),
            Arc::clone(&audit),
            config.ai.default_local_model.clone(),
        ));

        let mentor = Arc::new(MentorEngine::new(
            Arc::clone(&memory),
            Arc::clone(&orchestrator),
            Arc::clone(&audit),
        ));
        let capability = Arc::new(CapabilityEngine::new(
            Arc::clone(&memory),
            Arc::clone(&orchestrator),
            Arc::clone(&audit),
        ));
        let artifact = Arc::new(ArtifactEngine::new(
            Arc::clone(&memory),
            Arc::clone(&orchestrator),
            Arc::clone(&audit),
        ));
        let trust = Arc::new(TrustEngine::new(Arc::clone(&db)));
        let sync_coordinator = Arc::new(SyncCoordinator::new(Arc::clone(&db)));

        Ok(Self {
            config: Arc::new(config),
            db,
            memory,
            orchestrator,
            audit,
            mentor,
            capability,
            artifact,
            trust,
            sync_coordinator,
        })
    }
}

fn build_provider_registry(config: &Config) -> ProviderRegistry {
    let test_mode = std::env::var("DAEMON_TEST_MODE").is_ok();

    if test_mode {
        let local: Vec<Box<dyn ai::providers::Provider>> =
            vec![Box::new(MockProvider::default())];
        return ProviderRegistry::new(local, vec![]);
    }

    let local: Vec<Box<dyn ai::providers::Provider>> =
        vec![Box::new(OllamaProvider::new(config.ai.ollama_base_url.clone()))];

    let cloud: Vec<Box<dyn ai::providers::Provider>> = config
        .ai
        .claude_api_key
        .as_ref()
        .map(|key| -> Vec<Box<dyn ai::providers::Provider>> {
            vec![Box::new(ClaudeProvider::new(key.clone()))]
        })
        .unwrap_or_default();

    ProviderRegistry::new(local, cloud)
}

/// Start the daemon: open DB, bind HTTP server, return the bound address.
/// Factored out for testability — tests call this instead of `run()`.
pub async fn start(config: Config) -> error::Result<std::net::SocketAddr> {
    let port = config.daemon.http_port;
    let host = config.daemon.host.clone();

    let state = AppState::new(config).await?;
    let router = ipc::create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    let addr = listener.local_addr()?;
    info!(addr = %addr, "daemon HTTP server listening");

    tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("HTTP server terminated unexpectedly");
    });

    Ok(addr)
}

/// Full Tauri application entry point.
pub fn run() {
    init_tracing();

    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                let config = match Config::load() {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(err = %e, "failed to load config — daemon not started");
                        return;
                    }
                };
                if let Err(e) = start(config).await {
                    tracing::error!(err = %e, "daemon failed to start");
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

fn init_tracing() {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,cognition_daemon=debug"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true))
        .init();
}
