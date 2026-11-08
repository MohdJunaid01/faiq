use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tokio::net::TcpListener;
use crate::engine::LlmClient;
use crate::model::FaiqState;
use crate::storage::{Database, Episode, SyncEvent};
use super::sync::{ApiChatRequest, ApiChatResponse, ApiStatusResponse, SyncPushRequest, SyncPushResponse};

/// Daemon application state shared across HTTP requests.
#[derive(Clone)]
pub struct DaemonState {
    pub faiq: Arc<Mutex<FaiqState>>,
    pub db: Database,
    pub llm: LlmClient,
}

pub struct DaemonServer {
    state: DaemonState,
    port: u16,
}

impl DaemonServer {
    pub fn new(faiq: FaiqState, db: Database, port: u16) -> Self {
        Self {
            state: DaemonState {
                faiq: Arc::new(Mutex::new(faiq)),
                db,
                llm: LlmClient::new(),
            },
            port,
        }
    }

    /// Builds the Axum router for the daemon.
    pub fn build_router(state: DaemonState) -> Router {
        Router::new()
            .route("/health", get(health_handler))
            .route("/api/status", get(status_handler))
            .route("/api/chat", post(chat_handler))
            .route("/api/sync/push", post(sync_push_handler))
            .with_state(state)
    }

    /// Starts the daemon HTTP server listening on all network interfaces (for local Wi-Fi sync).
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await?;
        let router = Self::build_router(self.state.clone());

        println!("==================================================");
        println!("  FAIQ DAEMON | Background Service Active");
        println!("==================================================");
        println!("  Listening on:     http://0.0.0.0:{}", self.port);
        println!("  Local Wi-Fi Sync: Ready for Android companion");
        println!("  Status Endpoint:  http://localhost:{}/api/status", self.port);
        println!("==================================================");

        // Spawn periodic background sync/reflection worker
        let background_state = self.state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
            loop {
                interval.tick().await;
                let ep = Episode::new("daemon_heartbeat", "Daemon active and monitoring local network", "ok", 0.1);
                background_state.db.record_episode(&ep).ok();
            }
        });

        axum::serve(listener, router).await?;
        Ok(())
    }
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "alive",
        "entity": "Faiq",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn status_handler(State(state): State<DaemonState>) -> impl IntoResponse {
    let faiq = state.faiq.lock().unwrap().clone();
    let active_goals = state.db.get_active_goals().unwrap_or_default();
    let top_lessons = state.db.get_lessons(5).unwrap_or_default();
    let recent_episodes = state.db.get_recent_episodes(10).unwrap_or_default();

    Json(ApiStatusResponse {
        state: faiq,
        active_goals,
        top_lessons,
        recent_episodes,
    })
}

async fn chat_handler(
    State(state): State<DaemonState>,
    Json(payload): Json<ApiChatRequest>,
) -> impl IntoResponse {
    // 1. Prepare system prompt and regulation without holding the lock across .await
    let (system_prompt, reg) = {
        let faiq = state.faiq.lock().unwrap();
        let reg = faiq.regulate();

        let lessons = state.db.get_lessons(3).unwrap_or_default();
        let mut lesson_context = String::new();
        for l in lessons {
            lesson_context.push_str(&format!("- Principle: {}\n", l.principle));
        }

        let sys = format!(
            "{}\n\n\
            Current Composure: {:.1}%\n\
            Execution Clarity: {:.1}%\n\
            Top Distilled Rules:\n{}\n\
            Directive: {}",
            faiq.identity.build_system_prompt(),
            reg.composure * 100.0,
            reg.execution_clarity * 100.0,
            lesson_context,
            reg.directive
        );
        (sys, reg)
    };

    // 2. Await LLM response without holding the mutex lock
    let reply = state
        .llm
        .complete(&system_prompt, &payload.message)
        .await
        .unwrap_or_else(|_| "Understood. We evaluate the next step through strict calculation.".to_string());

    // 3. Acquire lock to update state and experience
    let (level, xp) = {
        let mut faiq = state.faiq.lock().unwrap();
        let _outcome = faiq.experience("strategic_dialogue", 2.0, true);
        state.db.save_state(&faiq).ok();
        (faiq.level_engine.level, faiq.level_engine.xp)
    };

    let episode = Episode::new("chat_interaction", &payload.message, "dialogue_completed", 0.5);
    state.db.record_episode(&episode).ok();

    Json(ApiChatResponse {
        reply,
        level,
        xp,
        composure: reg.composure,
        execution_clarity: reg.execution_clarity,
        directive: reg.directive,
    })
}

async fn sync_push_handler(
    State(state): State<DaemonState>,
    Json(payload): Json<SyncPushRequest>,
) -> impl IntoResponse {
    let mut faiq = state.faiq.lock().unwrap();
    let mut total_xp = 0.0;
    let events_count = payload.offline_messages.len() + payload.offline_notes.len();

    // Process queued offline messages
    for msg in &payload.offline_messages {
        let ep = Episode::new(
            &format!("mobile_sync:{}", payload.device_id),
            &msg.user_prompt,
            msg.response.as_deref().unwrap_or("queued"),
            0.5,
        );
        state.db.record_episode(&ep).ok();

        let outcome = faiq.experience("mobile_interaction", 1.5, true);
        total_xp += outcome.xp_gained;
    }

    // Process queued offline notes/thoughts
    for note in &payload.offline_notes {
        let ep = Episode::new(
            &format!("mobile_note:{}", payload.device_id),
            note,
            "recorded",
            0.6,
        );
        state.db.record_episode(&ep).ok();

        let outcome = faiq.experience("tactical_logging", 1.0, true);
        total_xp += outcome.xp_gained;
    }

    // Enqueue a sync completion event
    let sync_evt = SyncEvent::new(
        &format!("sync-{}", chrono::Utc::now().timestamp_millis()),
        &payload.device_id,
        "sync_completed",
        &format!("Processed {} events", events_count),
    );
    state.db.enqueue_sync_event(&sync_evt).ok();
    state.db.save_state(&faiq).ok();

    let active_goals = state.db.get_active_goals().unwrap_or_default();
    let top_lessons = state.db.get_lessons(5).unwrap_or_default();

    Json(SyncPushResponse {
        accepted_events: events_count,
        xp_awarded: total_xp,
        updated_state: faiq.clone(),
        active_goals,
        top_lessons,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daemon_health_and_sync_flow() {
        let db = Database::open_in_memory().unwrap();
        let faiq = FaiqState::default();
        let daemon_state = DaemonState {
            faiq: Arc::new(Mutex::new(faiq)),
            db,
            llm: LlmClient::new(),
        };

        let app = DaemonServer::build_router(daemon_state);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = reqwest::Client::new();

        // 1. Test /health
        let health_res = client
            .get(format!("http://{}/health", addr))
            .send()
            .await
            .unwrap();
        assert!(health_res.status().is_success());

        // 2. Test /api/status
        let status_res = client
            .get(format!("http://{}/api/status", addr))
            .send()
            .await
            .unwrap();
        assert!(status_res.status().is_success());

        // 3. Test /api/chat
        let chat_res = client
            .post(format!("http://{}/api/chat", addr))
            .json(&ApiChatRequest {
                message: "Status update".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert!(chat_res.status().is_success());

        // 4. Test /api/sync/push
        let sync_push = SyncPushRequest {
            device_id: "android-phone-01".to_string(),
            offline_messages: vec![super::super::sync::OfflineMessage {
                timestamp_rfc3339: chrono::Utc::now().to_rfc3339(),
                user_prompt: "Log offline reflection".to_string(),
                response: Some("Recorded.".to_string()),
            }],
            offline_notes: vec!["Deep work session completed".to_string()],
        };

        let push_res = client
            .post(format!("http://{}/api/sync/push", addr))
            .json(&sync_push)
            .send()
            .await
            .unwrap();
        assert!(push_res.status().is_success());
    }
}
