use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An episodic event in Faiq's permanent memory log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub source: String, // "conversation", "tool_execution", "mobile_sync", "manual"
    pub content: String,
    pub outcome: String,
    pub importance: f64, // 0.0 to 1.0
    pub synced: bool,
}

impl Episode {
    pub fn new(source: &str, content: &str, outcome: &str, importance: f64) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            source: source.to_string(),
            content: content.to_string(),
            outcome: outcome.to_string(),
            importance: importance.clamp(0.0, 1.0),
            synced: false,
        }
    }
}

/// A distilled lesson or tactical rule synthesized from experience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lesson {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub principle: String,
    pub context: String,
    pub relevance_count: u64,
    pub last_applied: Option<DateTime<Utc>>,
}

impl Lesson {
    pub fn new(principle: &str, context: &str) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            principle: principle.to_string(),
            context: context.to_string(),
            relevance_count: 1,
            last_applied: Some(Utc::now()),
        }
    }
}

/// Event model for synchronization between laptop daemon and mobile app.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncEvent {
    pub id: String,
    pub device_id: String,
    pub event_type: String,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    pub processed: bool,
}

impl SyncEvent {
    pub fn new(id: &str, device_id: &str, event_type: &str, payload: &str) -> Self {
        Self {
            id: id.to_string(),
            device_id: device_id.to_string(),
            event_type: event_type.to_string(),
            payload: payload.to_string(),
            created_at: Utc::now(),
            processed: false,
        }
    }
}

