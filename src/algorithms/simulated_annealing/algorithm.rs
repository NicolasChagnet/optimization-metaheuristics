use crate::algorithms::simulated_annealing::{
    config::SimulatedAnnealingConfig, config::SimulatedAnnealingStatus,
};
use rand::Rng;
use thiserror::Error;

/// Simulated annealing errors
#[derive(Debug, Error)]
pub enum SimulatedAnnealingError {
    #[error("Error generating a new solution during the iteration process")]
    NewSolutionError,
}

/// Trait for solutions with local variations
pub trait SimulatedAnnealing: Clone + std::fmt::Debug {
    type NewSolutionError;

    /// Get the objective function of a given solution
    fn objective(&self) -> f64;

    /// Generate a new solution
    fn new_solution(&self, rng: &mut impl Rng) -> Result<Self, Self::NewSolutionError>;
}

/// Main algorithm implementation for the simulated annealing algorithm
pub struct SimulatedAnnealingAlgorithm {
    /// Configuration object for the algorithm
    config: SimulatedAnnealingConfig,
    /// The status of the algorithm
    status: SimulatedAnnealingStatus,
}

impl SimulatedAnnealingAlgorithm {
    /// Constructor to create a new SimulatedAnnealingAlgorithm struct
    pub fn new(config: SimulatedAnnealingConfig) -> Self {
        SimulatedAnnealingAlgorithm {
            config,
            status: SimulatedAnnealingStatus::Ready,
        }
    }

    fn cooldown(&self, temperature: f64) -> f64 {
        temperature * self.config.cooling_rate
    }

    /// Find a solution with minimal objective function
    pub fn execute<T>(
        &mut self,
        initial_solution: T,
        rng: &mut impl Rng,
    ) -> Result<T, SimulatedAnnealingError>
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
            let new_solution = match current_solution.new_solution(rng) {
                Ok(solution) => solution,
                Err(_) => {
                    self.status = SimulatedAnnealingStatus::Failed;
                    return Err(SimulatedAnnealingError::NewSolutionError);
                }
            };

            // If the new solution's value is lower than the current one, always accepts it
            // Otherwise, acccept with a probability dependent on the temperature
            if (-(new_solution.objective() - current_solution.objective()) / temperature).exp()
                > rng.random()
            {
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
        self.status = SimulatedAnnealingStatus::Success;
        Ok(best_solution)
    }
}
