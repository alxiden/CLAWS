use rand_distr::{Normal, Distribution};

/// Calculates a yearly decision rate from weekly, then samples from a normal distribution (bell curve) with std deviation 5.
pub fn sample_yearly_decision_rate(weekly_rate: f64) -> f64 {
	let yearly_rate = weekly_rate * 52.0;
	let normal = Normal::new(yearly_rate, 5.0).unwrap();
	let mut rng = rand::rng();
	normal.sample(&mut rng)
}
