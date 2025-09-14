use crate::{
    algorithms::{errors::AlgorithmError, simulated_annealing::config::SimulatedAnnealingConfig},
    problems::ProblemError,
};
use rand::Rng;

/// Trait for solutions with local variations
pub trait SimulatedAnnealing: Clone + std::fmt::Debug {
    /// Get the objective function of a given solution
    fn objective(&self) -> f64;

    /// Generate a new solution
    fn new_solution(&self, rng: &mut impl Rng) -> Result<Self, ProblemError>;
}

/// Main algorithm implementation for the simulated annealing algorithm
pub struct SimulatedAnnealingAlgorithm {
    /// Configuration object for the algorithm
    pub config: SimulatedAnnealingConfig,
}

impl SimulatedAnnealingAlgorithm {
    /// Constructor to create a new SimulatedAnnealingAlgorithm struct
    pub fn new(config: SimulatedAnnealingConfig) -> Self {
        SimulatedAnnealingAlgorithm { config }
    }

    fn cooldown(&self, temperature: f64) -> f64 {
        temperature * self.config.cooling_rate
    }

    /// Find a solution with minimal objective function
    pub fn execute<T>(&self, initial_solution: T, rng: &mut impl Rng) -> Result<T, AlgorithmError>
    where
        T: SimulatedAnnealing,
    {
        // Initialize useful mutable variables
        let mut current_solution = initial_solution;
        let mut best_solution = current_solution.clone();
        let mut iteration = 0;
        let mut temperature = self.config.initial_temperature;

        // Loop until the final criterion is reached
        while iteration < self.config.max_iterations {
            let new_solution = current_solution
                .new_solution(rng)
                .map_err(|_| AlgorithmError::ExecutionError("could not generate new solution."))?;

            // If the new solution's value is higher than the current one, always accepts it
            // Otherwise, acccept with a probability dependent on the temperature
            let delta_objective = new_solution.objective() - current_solution.objective();
            if (-delta_objective / temperature).exp() > rng.random() {
                current_solution = new_solution;

                if current_solution.objective() < best_solution.objective() {
                    best_solution = current_solution.clone();
                }

                if self.config.stop_threshold.is_some()
                    && (best_solution.objective() < self.config.stop_threshold.unwrap())
                {
                    break;
                }
            }

            // Update temperature and iteration counter
            temperature = self
                .cooldown(temperature)
                .max(self.config.minimal_temperature);
            iteration += 1;
        }

        // Return the solution
        Ok(best_solution)
    }
}
