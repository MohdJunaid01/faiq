use serde::{Deserialize, Serialize};
use crate::model::FaiqState;
use crate::storage::{Database, Episode, Lesson};
use super::llm::LlmClient;

/// The synthesized result of a reflection cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionSummary {
    pub episodes_analyzed: usize,
    pub distilled_principle: String,
    pub strategic_insight: String,
    pub xp_awarded: f64,
    pub leveled_up: bool,
    pub new_level: u32,
}

pub struct ReflectionEngine {
    llm: LlmClient,
}

impl Default for ReflectionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ReflectionEngine {
    pub fn new() -> Self {
        Self {
            llm: LlmClient::new(),
        }
    }

    /// Performs an autonomous reflection cycle over recent episodic memory.
    pub async fn reflect(
        &self,
        state: &mut FaiqState,
        db: &Database,
        window_size: usize,
    ) -> Result<ReflectionSummary, String> {
        let episodes = db
            .get_recent_episodes(window_size)
            .map_err(|e| format!("Failed to load episodes: {}", e))?;

        if episodes.is_empty() {
            return Ok(ReflectionSummary {
                episodes_analyzed: 0,
                distilled_principle: "No recent episodes to reflect on.".to_string(),
                strategic_insight: "Engage in actions or conversations to provide experience data.".to_string(),
                xp_awarded: 0.0,
                leveled_up: false,
                new_level: state.level_engine.level,
            });
        }

        // Aggregate episodes context
        let mut ep_digest = String::new();
        let mut failure_count = 0;
        let mut success_count = 0;

        for ep in &episodes {
            ep_digest.push_str(&format!(
                "- [{}] Source: {}, Outcome: {}, Content: {}\n",
                ep.timestamp.format("%Y-%m-%d %H:%M"),
                ep.source,
                ep.outcome,
                ep.content
            ));
            if ep.outcome.to_lowercase().contains("fail") {
                failure_count += 1;
            } else {
                success_count += 1;
            }
        }

        let system_prompt = state.identity.build_system_prompt();
        let user_prompt = format!(
            "Review our recent events:\n{}\n\n\
            As Faiq, perform a cold, calculative synthesis. State:\n\
            1. The single distilled tactical principle/rule learned.\n\
            2. Strategic directive for the owner moving forward.\n\
            Keep it concise, direct, and unsparing.",
            ep_digest
        );

        let insight = self
            .llm
            .complete(&system_prompt, &user_prompt)
            .await
            .unwrap_or_else(|_| "Execute with total calculation. Suppress emotional noise.".to_string());

        let principle = if failure_count > 0 {
            format!(
                "Adversity distillation: {} failures converted into calculated tactical resilience",
                failure_count
            )
        } else {
            format!(
                "Operational momentum: {} successes executed with flawless precision",
                success_count
            )
        };

        // Record distilled lesson to SQLite database
        let lesson = Lesson::new(&principle, &format!("Distilled from {} recent events", episodes.len()));
        db.record_lesson(&lesson)
            .map_err(|e| format!("Failed to record lesson: {}", e))?;

        // Calculate and award experience based on reflection complexity
        let reflection_difficulty = ((episodes.len() as f64) * 0.5 + (failure_count as f64) * 1.0).clamp(2.0, 8.0);
        let learning_outcome = state.experience(
            "autonomous_reflection",
            reflection_difficulty,
            failure_count == 0,
        );

        // Record reflection event as an episode
        let ref_episode = Episode::new(
            "reflection_engine",
            &format!("Completed reflection cycle over {} events. Distilled: {}", episodes.len(), principle),
            "distilled",
            0.8,
        );
        db.record_episode(&ref_episode).ok();

        // Persist updated FaiqState to database
        db.save_state(state)
            .map_err(|e| format!("Failed to persist state: {}", e))?;

        Ok(ReflectionSummary {
            episodes_analyzed: episodes.len(),
            distilled_principle: principle,
            strategic_insight: insight,
            xp_awarded: learning_outcome.xp_gained,
            leveled_up: learning_outcome.leveled_up,
            new_level: state.level_engine.level,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reflection_cycle_with_episodes() {
        let db = Database::open_in_memory().unwrap();
        let mut state = FaiqState::default();

        let ep1 = Episode::new("cli", "Attempted complex algorithm", "failure", 0.8);
        let ep2 = Episode::new("cli", "Refactored algorithm", "success", 0.9);
        db.record_episode(&ep1).unwrap();
        db.record_episode(&ep2).unwrap();

        let engine = ReflectionEngine::default();
        let summary = engine.reflect(&mut state, &db, 5).await.unwrap();

        assert_eq!(summary.episodes_analyzed, 2);
        assert!(summary.distilled_principle.contains("Adversity distillation"));
        assert!(summary.xp_awarded > 0.0);

        let lessons = db.get_lessons(5).unwrap();
        assert_eq!(lessons.len(), 1);
    }
}

