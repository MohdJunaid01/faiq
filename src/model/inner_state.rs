use serde::{Deserialize, Serialize};
use super::traits::NumericTraits;

/// Fast-moving internal signals representing real-time cognitive and environmental load.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerState {
    pub stress_level: f64, // 0.0 (calm) to 1.0 (extreme load)
    pub focus: f64,        // 0.0 (scattered) to 1.0 (deep flow)
    pub urgency: f64,      // 0.0 (routine) to 1.0 (critical crisis)
}

impl Default for InnerState {
    fn default() -> Self {
        Self {
            stress_level: 0.1,
            focus: 0.9,
            urgency: 0.2,
        }
    }
}

/// The Calculation Regulator intercepts fast-moving emotional and cognitive signals
/// and ensures all decisions, advice, and responses remain strictly calculated and objective.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegulatedOutput {
    pub composure: f64,         // 0.0 to 1.0 (how impervious speech and action are to stress)
    pub execution_clarity: f64, // 0.0 to 1.0 (objective problem-solving sharpness)
    pub tactical_patience: f64, // 0.0 to 1.0 (resistance to panic or rushed action)
    pub directive: String,
}

impl InnerState {
    /// Applies calculation and resilience to regulate raw emotional/stress inputs.
    pub fn regulate(&self, traits: &NumericTraits) -> RegulatedOutput {
        let calc_factor = (traits.calculation / 100.0).clamp(0.0, 1.0);
        let res_factor = (traits.resilience / 100.0).clamp(0.0, 1.0);

        // High calculation & resilience dampens stress dampening down to near-zero impact
        let raw_stress_impact = self.stress_level * (1.0 - (calc_factor * 0.7 + res_factor * 0.3));
        let composure = (1.0 - raw_stress_impact).clamp(0.0, 1.0);

        // Execution clarity is driven by focus reinforced by calculation
        let execution_clarity = (self.focus * 0.5 + calc_factor * 0.5).clamp(0.0, 1.0);

        // Tactical patience balances urgency through calculative composure
        let tactical_patience = (1.0 - (self.urgency * (1.0 - calc_factor))).clamp(0.0, 1.0);

        let directive = if composure >= 0.85 {
            "Maintain total objective composure. Evaluate the optimal move through raw calculation."
                .to_string()
        } else if self.urgency >= 0.7 {
            "High urgency detected. Suppress emotional noise; execute direct, high-leverage actions immediately."
                .to_string()
        } else {
            "Recenter focus. Filter external friction through tactical calculation."
                .to_string()
        };

        RegulatedOutput {
            composure,
            execution_clarity,
            tactical_patience,
            directive,
        }
    }

    /// Updates internal load based on external stimulus.
    pub fn register_stimulus(&mut self, stress_delta: f64, focus_delta: f64, urgency_delta: f64) {
        self.stress_level = (self.stress_level + stress_delta).clamp(0.0, 1.0);
        self.focus = (self.focus + focus_delta).clamp(0.0, 1.0);
        self.urgency = (self.urgency + urgency_delta).clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regulation_maintains_high_composure_under_stress() {
        let traits = NumericTraits::default();
        let mut state = InnerState::default();

        // Inject severe stress and urgency
        state.register_stimulus(0.8, -0.2, 0.7);
        let output = state.regulate(&traits);

        // Even under severe stress, Faiq's high calculation (92) and resilience (90)
        // ensure composure remains exceptionally high
        assert!(output.composure > 0.75);
        assert!(output.execution_clarity > 0.65);
    }

    #[test]
    fn test_stimulus_clamping() {
        let mut state = InnerState::default();
        state.register_stimulus(5.0, -5.0, 10.0);
        assert_eq!(state.stress_level, 1.0);
        assert_eq!(state.focus, 0.0);
        assert_eq!(state.urgency, 1.0);
    }
}

