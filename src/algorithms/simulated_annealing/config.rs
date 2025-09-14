use crate::algorithms::errors::AlgorithmError;

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

impl SimulatedAnnealingConfig {
    pub fn new(
        max_iterations: usize,
        initial_temperature: f64,
        minimal_temperature: f64,
        cooling_rate: f64,
        stop_threshold: Option<f64>,
    ) -> Result<Self, AlgorithmError> {
        if initial_temperature < 0.0
            || minimal_temperature < 0.0
            || initial_temperature < minimal_temperature
        {
            return Err(AlgorithmError::ConfigurationError(
                "the initial temperature should be above the minimal temperature, and both should be larger than 0.0.",
            ));
        }
        if !(0.0..=1.0).contains(&cooling_rate) {
            return Err(AlgorithmError::ConfigurationError(
                "the cooling rate should be between 0 and 1.",
            ));
        }
        Ok(SimulatedAnnealingConfig {
            max_iterations,
            initial_temperature,
            minimal_temperature,
            cooling_rate,
            stop_threshold,
        })
    }
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
