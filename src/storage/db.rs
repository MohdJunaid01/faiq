use std::path::Path;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use crate::model::{FaiqState, GoalStatus, TacticalGoal};
use super::models::{Episode, Lesson, SyncEvent};

/// SQLite database manager for Faiq's persistent episodic and distilled memory.
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Opens or creates a SQLite database at the specified path and initializes the schema.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Creates an in-memory SQLite database (primarily for testing).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Initializes all tables and indices.
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS faiq_state (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                state_json TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS episodes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                source TEXT NOT NULL,
                content TEXT NOT NULL,
                outcome TEXT NOT NULL,
                importance REAL NOT NULL,
                synced INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS lessons (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                principle TEXT NOT NULL,
                context TEXT NOT NULL,
                relevance_count INTEGER NOT NULL DEFAULT 1,
                last_applied TEXT
            );

            CREATE TABLE IF NOT EXISTS tactical_goals (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                difficulty REAL NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                completed_at TEXT
            );

            CREATE TABLE IF NOT EXISTS sync_events (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at TEXT NOT NULL,
                processed INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_episodes_time ON episodes(timestamp DESC);
            CREATE INDEX IF NOT EXISTS idx_lessons_relevance ON lessons(relevance_count DESC);
            CREATE INDEX IF NOT EXISTS idx_sync_processed ON sync_events(processed, created_at ASC);
            "#,
        )?;

        Ok(())
    }

    /// Persists the active FaiqState to the database.
    pub fn save_state(&self, state: &FaiqState) -> Result<()> {
        let json = serde_json::to_string(state)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let now = Utc::now().to_rfc3339();

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO faiq_state (id, state_json, updated_at) VALUES (1, ?1, ?2)
             ON CONFLICT(id) DO UPDATE SET state_json = excluded.state_json, updated_at = excluded.updated_at",
            params![json, now],
        )?;

        Ok(())
    }

    /// Loads the persisted FaiqState if one exists.
    pub fn load_state(&self) -> Result<Option<FaiqState>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT state_json FROM faiq_state WHERE id = 1")?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            let json: String = row.get(0)?;
            let state: FaiqState = serde_json::from_str(&json)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Records an episodic event.
    pub fn record_episode(&self, episode: &Episode) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO episodes (timestamp, source, content, outcome, importance, synced)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                episode.timestamp.to_rfc3339(),
                episode.source,
                episode.content,
                episode.outcome,
                episode.importance,
                if episode.synced { 1 } else { 0 },
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Retrieves recent episodic memories.
    pub fn get_recent_episodes(&self, limit: usize) -> Result<Vec<Episode>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, source, content, outcome, importance, synced
             FROM episodes ORDER BY timestamp DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            let ts_str: String = row.get(1)?;
            let timestamp = DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let synced_int: i32 = row.get(6)?;

            Ok(Episode {
                id: Some(row.get(0)?),
                timestamp,
                source: row.get(2)?,
                content: row.get(3)?,
                outcome: row.get(4)?,
                importance: row.get(5)?,
                synced: synced_int == 1,
            })
        })?;

        let mut episodes = Vec::new();
        for ep in rows {
            episodes.push(ep?);
        }
        Ok(episodes)
    }

    /// Stores a new distilled lesson.
    pub fn record_lesson(&self, lesson: &Lesson) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let last_applied_str = lesson.last_applied.map(|dt| dt.to_rfc3339());

        conn.execute(
            "INSERT INTO lessons (timestamp, principle, context, relevance_count, last_applied)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                lesson.timestamp.to_rfc3339(),
                lesson.principle,
                lesson.context,
                lesson.relevance_count as i64,
                last_applied_str,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Retrieves distilled lessons ordered by relevance count.
    pub fn get_lessons(&self, limit: usize) -> Result<Vec<Lesson>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, principle, context, relevance_count, last_applied
             FROM lessons ORDER BY relevance_count DESC, id DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            let ts_str: String = row.get(1)?;
            let timestamp = DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let last_applied_str: Option<String> = row.get(5)?;
            let last_applied = last_applied_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .ok()
            });

            Ok(Lesson {
                id: Some(row.get(0)?),
                timestamp,
                principle: row.get(2)?,
                context: row.get(3)?,
                relevance_count: row.get::<_, i64>(4)? as u64,
                last_applied,
            })
        })?;

        let mut lessons = Vec::new();
        for l in rows {
            lessons.push(l?);
        }
        Ok(lessons)
    }

    /// Increments the relevance counter of a lesson and updates its last_applied timestamp.
    pub fn reinforce_lesson(&self, lesson_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE lessons SET relevance_count = relevance_count + 1, last_applied = ?1 WHERE id = ?2",
            params![now, lesson_id],
        )?;
        Ok(())
    }

    /// Saves or updates a tactical goal.
    pub fn save_goal(&self, goal: &TacticalGoal) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let status_str = match goal.status {
            GoalStatus::Active => "Active",
            GoalStatus::Completed => "Completed",
            GoalStatus::Failed => "Failed",
            GoalStatus::Suspended => "Suspended",
        };
        let completed_str = goal.completed_at.map(|dt| dt.to_rfc3339());

        conn.execute(
            "INSERT INTO tactical_goals (id, title, description, difficulty, status, created_at, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                description = excluded.description,
                difficulty = excluded.difficulty,
                status = excluded.status,
                completed_at = excluded.completed_at",
            params![
                goal.id,
                goal.title,
                goal.description,
                goal.difficulty,
                status_str,
                goal.created_at.to_rfc3339(),
                completed_str,
            ],
        )?;

        Ok(())
    }

    /// Retrieves all active tactical goals.
    pub fn get_active_goals(&self) -> Result<Vec<TacticalGoal>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, difficulty, status, created_at, completed_at
             FROM tactical_goals WHERE status = 'Active' ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let created_str: String = row.get(5)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let completed_str: Option<String> = row.get(6)?;
            let completed_at = completed_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .ok()
            });

            Ok(TacticalGoal {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                difficulty: row.get(3)?,
                status: GoalStatus::Active,
                created_at,
                completed_at,
            })
        })?;

        let mut goals = Vec::new();
        for g in rows {
            goals.push(g?);
        }
        Ok(goals)
    }

    /// Enqueues a sync event for synchronization with mobile/daemon.
    pub fn enqueue_sync_event(&self, event: &SyncEvent) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO sync_events (id, device_id, event_type, payload, created_at, processed)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO NOTHING",
            params![
                event.id,
                event.device_id,
                event.event_type,
                event.payload,
                event.created_at.to_rfc3339(),
                if event.processed { 1 } else { 0 },
            ],
        )?;

        Ok(())
    }

    /// Fetches all unprocessed sync events.
    pub fn get_pending_sync_events(&self) -> Result<Vec<SyncEvent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, device_id, event_type, payload, created_at, processed
             FROM sync_events WHERE processed = 0 ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let created_str: String = row.get(4)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let processed_int: i32 = row.get(5)?;

            Ok(SyncEvent {
                id: row.get(0)?,
                device_id: row.get(1)?,
                event_type: row.get(2)?,
                payload: row.get(3)?,
                created_at,
                processed: processed_int == 1,
            })
        })?;

        let mut events = Vec::new();
        for e in rows {
            events.push(e?);
        }
        Ok(events)
    }

    /// Marks a sync event as processed.
    pub fn mark_sync_event_processed(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sync_events SET processed = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Exports a markdown summary of episodic memory, lessons, and goals.
    pub fn export_summary_markdown(&self) -> Result<String> {
        let episodes = self.get_recent_episodes(10)?;
        let lessons = self.get_lessons(10)?;
        let goals = self.get_active_goals()?;

        let mut md = String::from("# FAIQ: Memory & Intelligence Digest\n\n");

        md.push_str("## Active Tactical Goals\n");
        if goals.is_empty() {
            md.push_str("*No active goals.*\n\n");
        } else {
            for g in goals {
                md.push_str(&format!("- **{}** (Diff {:.1}): {}\n", g.title, g.difficulty, g.description));
            }
            md.push('\n');
        }

        md.push_str("## Distilled Lessons\n");
        if lessons.is_empty() {
            md.push_str("*No distilled lessons logged yet.*\n\n");
        } else {
            for l in lessons {
                md.push_str(&format!("- **{}** (Applied {}x): {}\n", l.principle, l.relevance_count, l.context));
            }
            md.push('\n');
        }

        md.push_str("## Recent Episodes\n");
        if episodes.is_empty() {
            md.push_str("*No episodic events logged yet.*\n\n");
        } else {
            for ep in episodes {
                md.push_str(&format!(
                    "- `[{}]` **{}** (Outcome: {}): {}\n",
                    ep.timestamp.format("%Y-%m-%d %H:%M"),
                    ep.source,
                    ep.outcome,
                    ep.content
                ));
            }
            md.push('\n');
        }

        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_init_and_state_roundtrip() {
        let db = Database::open_in_memory().unwrap();
        assert!(db.load_state().unwrap().is_none());

        let mut state = FaiqState::default();
        state.level_engine.level = 3;
        state.level_engine.xp = 75.0;
        state.traits.calculation = 95.0;

        db.save_state(&state).unwrap();
        let loaded = db.load_state().unwrap().expect("State should exist");
        assert_eq!(loaded.level_engine.level, 3);
        assert_eq!(loaded.level_engine.xp, 75.0);
        assert_eq!(loaded.traits.calculation, 95.0);
    }

    #[test]
    fn test_episodes_recording_and_retrieval() {
        let db = Database::open_in_memory().unwrap();
        let ep1 = Episode::new("cli", "Executed cargo check", "success", 0.7);
        let ep2 = Episode::new("cli", "Ran test suite", "11 passed", 0.9);

        db.record_episode(&ep1).unwrap();
        db.record_episode(&ep2).unwrap();

        let recents = db.get_recent_episodes(10).unwrap();
        assert_eq!(recents.len(), 2);
        assert_eq!(recents[0].content, "Ran test suite");
    }

    #[test]
    fn test_lessons_recording_and_reinforcement() {
        let db = Database::open_in_memory().unwrap();
        let lesson = Lesson::new("Adversity compounds mastery", "Failure yields 1.5x XP bonus");
        let id = db.record_lesson(&lesson).unwrap();

        db.reinforce_lesson(id).unwrap();

        let lessons = db.get_lessons(5).unwrap();
        assert_eq!(lessons.len(), 1);
        assert_eq!(lessons[0].relevance_count, 2);
    }

    #[test]
    fn test_tactical_goals_storage() {
        let db = Database::open_in_memory().unwrap();
        let mut goal = TacticalGoal::new("g1", "Implement SQLite Storage", "Phase 2 persistence", 4.0);
        db.save_goal(&goal).unwrap();

        let active = db.get_active_goals().unwrap();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].title, "Implement SQLite Storage");

        goal.mark_completed();
        db.save_goal(&goal).unwrap();

        let active_after = db.get_active_goals().unwrap();
        assert_eq!(active_after.len(), 0);
    }

    #[test]
    fn test_sync_events_queue() {
        let db = Database::open_in_memory().unwrap();
        let event = SyncEvent::new("evt-1", "android-phone", "chat_message", r#"{"msg":"Push yourself"}"#);
        db.enqueue_sync_event(&event).unwrap();

        let pending = db.get_pending_sync_events().unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "evt-1");

        db.mark_sync_event_processed("evt-1").unwrap();
        let pending_after = db.get_pending_sync_events().unwrap();
        assert_eq!(pending_after.len(), 0);
    }
}

