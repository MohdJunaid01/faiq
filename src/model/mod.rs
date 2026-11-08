pub mod goals;
pub mod identity;
pub mod inner_state;
pub mod level;
pub mod traits;

pub use goals::{AscensionGoal, GoalStatus, TacticalGoal};
pub use identity::Identity;
pub use inner_state::{InnerState, RegulatedOutput};
pub use level::{LearningOutcome, LevelEngine, SkillProgress};
pub use traits::NumericTraits;

use serde::{Deserialize, Serialize};

/// The complete encapsulated state of Faiq.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FaiqState {
    pub identity: Identity,
    pub traits: NumericTraits,
    pub inner_state: InnerState,
    pub level_engine: LevelEngine,
    pub ascension_goal: AscensionGoal,
    pub active_goals: Vec<TacticalGoal>,
}

impl Default for FaiqState {
    fn default() -> Self {
        Self {
            identity: Identity::default(),
            traits: NumericTraits::default(),
            inner_state: InnerState::default(),
            level_engine: LevelEngine::default(),
            ascension_goal: AscensionGoal::default(),
            active_goals: Vec::new(),
        }
    }
}

impl FaiqState {
    /// Returns the regulated output for current inner state and traits.
    pub fn regulate(&self) -> RegulatedOutput {
        self.inner_state.regulate(&self.traits)
    }

    /// Records an experience and updates skills, traits, and level.
    pub fn experience(&mut self, skill: &str, difficulty: f64, success: bool) -> LearningOutcome {
        self.level_engine
            .record_experience(skill, difficulty, success, &mut self.traits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faiq_state_integration() {
        let mut faiq = FaiqState::default();
        assert_eq!(faiq.identity.name, "Faiq");
        assert_eq!(faiq.level_engine.level, 1);

        // Experience a challenging failure
        let outcome = faiq.experience("strategic_calculation", 4.0, false);
        // 4.0 * 20 * 1.5 = 120 XP -> crosses 100 XP threshold!
        assert_eq!(outcome.old_level, 1);
        assert_eq!(outcome.new_level, 2);
        assert!(outcome.leveled_up);

        let regulation = faiq.regulate();
        assert!(regulation.composure >= 0.8);
    }
}

