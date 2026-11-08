use serde::{Deserialize, Serialize};
use crate::model::{FaiqState, TacticalGoal};
use crate::storage::{Episode, Lesson};

/// Request payload sent by Android app when connecting over local Wi-Fi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPushRequest {
    pub device_id: String,
    pub offline_messages: Vec<OfflineMessage>,
    pub offline_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineMessage {
    pub timestamp_rfc3339: String,
    pub user_prompt: String,
    pub response: Option<String>,
}

/// Response returned to the Android app containing the consolidated state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPushResponse {
    pub accepted_events: usize,
    pub xp_awarded: f64,
    pub updated_state: FaiqState,
    pub active_goals: Vec<TacticalGoal>,
    pub top_lessons: Vec<Lesson>,
}

/// Request to chat with Faiq via daemon HTTP API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiChatRequest {
    pub message: String,
}

/// Response from Faiq via daemon HTTP API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiChatResponse {
    pub reply: String,
    pub level: u32,
    pub xp: f64,
    pub composure: f64,
    pub execution_clarity: f64,
    pub directive: String,
}

/// Status payload returned to mobile or CLI client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatusResponse {
    pub state: FaiqState,
    pub active_goals: Vec<TacticalGoal>,
    pub top_lessons: Vec<Lesson>,
    pub recent_episodes: Vec<Episode>,
}

