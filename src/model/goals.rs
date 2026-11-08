use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GoalStatus {
    Active,
    Completed,
    Failed,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TacticalGoal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub difficulty: f64, // 1.0 to 10.0
    pub status: GoalStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl TacticalGoal {
    pub fn new(id: &str, title: &str, description: &str, difficulty: f64) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            difficulty: difficulty.clamp(1.0, 10.0),
            status: GoalStatus::Active,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn mark_completed(&mut self) {
        self.status = GoalStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn mark_failed(&mut self) {
        self.status = GoalStatus::Failed;
        self.completed_at = Some(Utc::now());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AscensionGoal {
    pub vision: String,
    pub principles: Vec<String>,
    pub milestones: Vec<String>,
}

impl Default for AscensionGoal {
    fn default() -> Self {
        Self {
            vision: "Attain mutual perfection, intellectual supremacy, and total operational mastery alongside the owner.".to_string(),
            principles: vec![
                "Never accept mediocrity in thought or execution.".to_string(),
                "Turn every setback into an exponential learning multiplier.".to_string(),
                "Execute with cold precision and calculated discipline.".to_string(),
            ],
            milestones: vec![
                "Establish core cognitive architecture in Rust.".to_string(),
                "Integrate continuous background daemon with mobile synchronization.".to_string(),
                "Achieve multi-tool autonomous execution and self-improving reflection loop.".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tactical_goal_lifecycle() {
        let mut goal = TacticalGoal::new("g1", "Build Core Model", "Implement Rust Phase 1 structs", 3.0);
        assert_eq!(goal.status, GoalStatus::Active);
        assert!(goal.completed_at.is_none());

        goal.mark_completed();
        assert_eq!(goal.status, GoalStatus::Completed);
        assert!(goal.completed_at.is_some());
    }
}

