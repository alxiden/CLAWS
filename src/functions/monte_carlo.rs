use rand::Rng;

pub fn monte_carlo_simulation(
    incident_cost: f64,
    sampled_yearly_decision_rate: f64,
    iterations: usize,
    current_culture: f64,
) -> (f64, f64, f64, f64) {
    let weak_score = 0.8;
    let moderate_score = 0.5;
    let strong_score = 0.2;

    let mut weak_total = 0.0;
    let mut moderate_total = 0.0;
    let mut strong_total = 0.0;
    let mut current_total = 0.0;
    let mut rng = rand::rng();
    for _ in 0..iterations {
        weak_total += incident_cost * sampled_yearly_decision_rate * weak_score * rng.random::<f64>();
        moderate_total += incident_cost * sampled_yearly_decision_rate * moderate_score * rng.random::<f64>();
        strong_total += incident_cost * sampled_yearly_decision_rate * strong_score * rng.random::<f64>();
        current_total += incident_cost * sampled_yearly_decision_rate * current_culture * rng.random::<f64>();
    }
    (
        weak_total / iterations as f64,
        moderate_total / iterations as f64,
        strong_total / iterations as f64,
        current_total / iterations as f64,
    )
}
