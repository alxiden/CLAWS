mod functions;
use crate::functions::incident_rate_calc::sample_yearly_decision_rate;
use crate::functions::inputs::get_user_input;
use crate::functions::monte_carlo::monte_carlo_simulation;

fn main() {
    let incident_cost = get_user_input("Enter the incident cost (do not include currency symbols):");
    let incident_cost: f64 = incident_cost.trim().parse().expect("Invalid input");
    let decision_rate = get_user_input("Enter the decision rate (Number of security decisions made each week): ");
    let decision_rate: f64 = decision_rate.trim().parse().expect("Invalid input");

    // Calculate sampled yearly decision rate using bell curve
    let sampled_yearly_decision_rate = sample_yearly_decision_rate(decision_rate);

    let iterations = 100_000;
    let (weak_cost, moderate_cost, strong_cost) = monte_carlo_simulation(
        incident_cost,
        sampled_yearly_decision_rate,
        iterations,
    );
    println!("\nYearly Cyber Incident Cost Estimates:");
    println!("Weak Culture:    £{:.2}", weak_cost);
    println!("Moderate Culture: £{:.2}", moderate_cost);
    println!("Strong Culture:   £{:.2}", strong_cost);
}
