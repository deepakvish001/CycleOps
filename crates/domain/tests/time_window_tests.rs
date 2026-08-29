#[derive(Debug, Clone, Copy)]
struct Scenario { observed: f64, baseline: f64, confidence: f64 }

impl Scenario { fn score(self) -> f64 { (self.observed - self.baseline).abs() * self.confidence } }

#[test]
fn time_window_tests_is_deterministic() {
    let scenario = Scenario { observed: 18.0, baseline: 12.0, confidence: 0.8 };
    assert!((scenario.score() - 4.8).abs() < f64::EPSILON);
    assert_eq!(scenario.score(), scenario.score());
}
