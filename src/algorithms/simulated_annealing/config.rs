/// Configuration for the simulated annealing algorithm
pub struct SimulatedAnnealingConfig {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Initial temperature for the algorithm
    pub initial_temperature: f64,
    /// Minimal temperature below which the temperature gets clipped
    pub minimal_temperature: f64,
    /// Cooling rate
    pub cooling_rate: f64,
    /// Threshold under which the objective function should stop (if the target value is zero)
    pub stop_threshold: Option<f64>,
}

impl Default for SimulatedAnnealingConfig {
    fn default() -> Self {
        SimulatedAnnealingConfig {
            max_iterations: 1_000,
            initial_temperature: 1.0,
            minimal_temperature: 0.0,
            cooling_rate: 0.99,
            stop_threshold: None,
        }
    }
}

/// Enum containing the possible status of the algorithm
pub enum SimulatedAnnealingStatus {
    Ready,
    Success,
    Failed,
}
