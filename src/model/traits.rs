use serde::{Deserialize, Serialize};

/// Slow-moving numeric traits that shift gradually from experience.
/// All traits are bounded in the range [0.0, 100.0].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NumericTraits {
    pub confidence: f64,
    pub resilience: f64,
    pub intelligence: f64,
    pub calculation: f64,
    pub courage: f64,
}

impl Default for NumericTraits {
    fn default() -> Self {
        Self {
            confidence: 80.0,
            resilience: 90.0, // Forged through surviving the past
            intelligence: 85.0,
            calculation: 92.0, // High calculative rigor
            courage: 88.0,     // Lionhearted
        }
    }
}

impl NumericTraits {
    /// Adjusts a trait by a delta, clamping the result strictly between 0.0 and 100.0.
    pub fn adjust(&mut self, trait_name: &str, delta: f64) -> Result<f64, String> {
        let val = match trait_name.to_lowercase().as_str() {
            "confidence" => &mut self.confidence,
            "resilience" => &mut self.resilience,
            "intelligence" => &mut self.intelligence,
            "calculation" => &mut self.calculation,
            "courage" => &mut self.courage,
            other => return Err(format!("Unknown trait: {}", other)),
        };

        *val = (*val + delta).clamp(0.0, 100.0);
        Ok(*val)
    }

    /// Calculates an aggregate composite mastery score (0.0 to 100.0).
    pub fn composite_score(&self) -> f64 {
        (self.confidence + self.resilience + self.intelligence + self.calculation + self.courage) / 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_defaults_and_clamping() {
        let mut traits = NumericTraits::default();
        assert_eq!(traits.resilience, 90.0);
        assert_eq!(traits.calculation, 92.0);

        // Adjust upwards with clamping
        traits.adjust("confidence", 25.0).unwrap();
        assert_eq!(traits.confidence, 100.0);

        // Adjust downwards with clamping
        traits.adjust("courage", -150.0).unwrap();
        assert_eq!(traits.courage, 0.0);
    }

    #[test]
    fn test_unknown_trait() {
        let mut traits = NumericTraits::default();
        assert!(traits.adjust("nonexistent", 5.0).is_err());
    }

    #[test]
    fn test_composite_score() {
        let traits = NumericTraits::default();
        let score = traits.composite_score();
        assert!(score > 80.0 && score < 100.0);
    }
}

