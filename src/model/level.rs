use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::traits::NumericTraits;

/// Specific skill progress and mastery tracking.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillProgress {
    pub name: String,
    pub level: u32,
    pub mastery: f64, // 0.0 to 100.0
    pub success_count: u64,
    pub failure_count: u64,
}

impl SkillProgress {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            level: 1,
            mastery: 10.0,
            success_count: 0,
            failure_count: 0,
        }
    }
}

/// The result of an experience or learning event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearningOutcome {
    pub xp_gained: f64,
    pub old_level: u32,
    pub new_level: u32,
    pub leveled_up: bool,
    pub unlocked_capabilities: Vec<String>,
    pub skill_mastery_gain: f64,
    pub trait_adjustments: Vec<(String, f64)>,
}

/// Adaptive Level-Up Engine.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LevelEngine {
    pub level: u32,
    pub xp: f64,
    pub skills: HashMap<String, SkillProgress>,
    pub total_experiences: u64,
}

impl Default for LevelEngine {
    fn default() -> Self {
        Self {
            level: 1,
            xp: 0.0,
            skills: HashMap::new(),
            total_experiences: 0,
        }
    }
}

impl LevelEngine {
    /// Computes the required XP to advance from the current level to the next.
    pub fn xp_threshold_for_level(level: u32) -> f64 {
        level as f64 * 100.0
    }

    /// Returns the active capability tier description for a given level.
    pub fn capabilities_for_level(level: u32) -> Vec<String> {
        let mut caps = vec![
            "Tier 1: Analytical sparring, razor-sharp dialogue, and single-step CLI commands".to_string(),
        ];

        if level >= 2 {
            caps.push("Tier 2: Structured goal planning, file inspections, and automated script execution".to_string());
        }
        if level >= 3 {
            caps.push("Tier 3: Autonomous failure analysis, resilient self-recovery, and codebase decomposition".to_string());
        }
        if level >= 4 {
            caps.push("Tier 4: Multi-step strategy synthesis, cross-session project orchestration, and deep reflection".to_string());
        }
        if level >= 5 {
            caps.push("Tier 5: High-autonomy alter ego, predictive execution, and uninhibited tool orchestration".to_string());
        }

        caps
    }

    /// Records an experience, updates skills, applies the failure multiplier (1.5x),
    /// checks level-up thresholds, and adjusts numeric traits.
    pub fn record_experience(
        &mut self,
        skill_name: &str,
        difficulty: f64,
        success: bool,
        traits: &mut NumericTraits,
    ) -> LearningOutcome {
        self.total_experiences += 1;
        let diff = difficulty.clamp(1.0, 10.0);

        // Failure grants a 1.5x experience multiplier because overcoming adversity accelerates mastery
        let multiplier = if success { 1.0 } else { 1.5 };
        let base_xp = 20.0;
        let xp_gain = diff * base_xp * multiplier;

        // Update skill entry
        let skill = self
            .skills
            .entry(skill_name.to_string())
            .or_insert_with(|| SkillProgress::new(skill_name));

        if success {
            skill.success_count += 1;
        } else {
            skill.failure_count += 1;
        }

        let mastery_gain = (diff * if success { 1.2 } else { 1.8 }).clamp(0.1, 15.0);
        skill.mastery = (skill.mastery + mastery_gain).clamp(0.0, 100.0);
        while skill.mastery >= (skill.level as f64 * 20.0) && skill.level < 10 {
            skill.level += 1;
        }

        // Apply trait adaptations based on success or failure
        let mut trait_adjustments = Vec::new();
        if success {
            traits.adjust("confidence", 0.5).ok();
            traits.adjust("calculation", 0.4).ok();
            trait_adjustments.push(("confidence".to_string(), 0.5));
            trait_adjustments.push(("calculation".to_string(), 0.4));
        } else {
            // Failure tempers resilience and courage
            traits.adjust("resilience", 0.8).ok();
            traits.adjust("courage", 0.6).ok();
            traits.adjust("calculation", 0.3).ok();
            trait_adjustments.push(("resilience".to_string(), 0.8));
            trait_adjustments.push(("courage".to_string(), 0.6));
            trait_adjustments.push(("calculation".to_string(), 0.3));
        }

        // Accumulate XP and handle level up
        let old_level = self.level;
        self.xp += xp_gain;

        while self.xp >= Self::xp_threshold_for_level(self.level) {
            self.xp -= Self::xp_threshold_for_level(self.level);
            self.level += 1;
        }

        let leveled_up = self.level > old_level;
        let unlocked_capabilities = if leveled_up {
            Self::capabilities_for_level(self.level)
        } else {
            Vec::new()
        };

        LearningOutcome {
            xp_gained: xp_gain,
            old_level,
            new_level: self.level,
            leveled_up,
            unlocked_capabilities,
            skill_mastery_gain: mastery_gain,
            trait_adjustments,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_yields_higher_xp_and_resilience() {
        let mut engine = LevelEngine::default();
        let mut traits = NumericTraits::default();
        let initial_resilience = traits.resilience;

        let outcome = engine.record_experience("rust_debugging", 2.0, false, &mut traits);

        // 2.0 * 20.0 * 1.5 = 60.0 XP
        assert_eq!(outcome.xp_gained, 60.0);
        assert!(!outcome.leveled_up);
        assert!(traits.resilience > initial_resilience);
    }

    #[test]
    fn test_level_up_threshold_and_capabilities_unlock() {
        let mut engine = LevelEngine::default();
        let mut traits = NumericTraits::default();

        // Level 1 threshold is 100.0. A high-difficulty failure (diff 5.0) yields 5 * 20 * 1.5 = 150.0 XP
        let outcome = engine.record_experience("systems_architecture", 5.0, false, &mut traits);

        assert!(outcome.leveled_up);
        assert_eq!(outcome.old_level, 1);
        assert_eq!(outcome.new_level, 2);
        assert_eq!(engine.xp, 50.0); // 150 - 100
        assert!(outcome.unlocked_capabilities.iter().any(|c| c.contains("Tier 2")));
    }
}

