//! Deterministic pickup priority policy.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Evaluation { pub score: f64, pub triggered: bool }

pub fn evaluate(observed: f64, baseline: f64, confidence: f64, threshold: f64) -> Result<Evaluation, &'static str> {
    if !(0.0..=1.0).contains(&confidence) || threshold < 0.0 { return Err("invalid policy input"); }
    let score = (observed - baseline).abs() * confidence;
    Ok(Evaluation { score, triggered: score >= threshold })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evaluates_pickup_priority() { assert!(evaluate(12.0, 10.0, 0.75, 1.0).unwrap().triggered); }
}
