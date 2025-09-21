use std::time::{Duration, Instant};

use crate::{
    algorithms::{errors::AlgorithmError, simulated_annealing::config::SimulatedAnnealingConfig},
    problems::{ProblemError, ProblemSolution},
};
use rand::Rng;

/// Trait for solutions with local variations
pub trait SimulatedAnnealing: Clone + std::fmt::Debug + ProblemSolution {
    /// Generate a new solution
    fn new_solution(&self, rng: &mut impl Rng) -> Result<Self, ProblemError>;
}

/// Main algorithm implementation for the simulated annealing algorithm
pub struct SimulatedAnnealingAlgorithm {
    /// Configuration object for the algorithm
    pub config: SimulatedAnnealingConfig,
}

/// Simulation result and statistics
pub struct SimulationResult<T> {
    /// Solution from the metaheuristics
    pub solution: T,
    /// Run time of the algorithm
    pub runtime: Duration,
    /// Number of iterations
    pub number_iterations: usize,
}

impl<T> SimulationResult<T> {
    pub fn new(solution: T, initial_time: Instant, number_iterations: usize) -> Self {
        Self {
            solution,
            runtime: Instant::now() - initial_time,
            number_iterations,
        }
    }
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
    pub fn execute<T>(
        &self,
        initial_solution: T,
        rng: &mut impl Rng,
    ) -> Result<SimulationResult<T>, AlgorithmError>
    where
        T: SimulatedAnnealing,
    {
        let initial_time = Instant::now();

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
            }

            // Update temperature and iteration counter
            temperature = self
                .cooldown(temperature)
                .max(self.config.minimal_temperature);
            iteration += 1;

            // Early stopping check
            if self.config.stop_threshold.is_some()
                && (best_solution.objective() < self.config.stop_threshold.unwrap())
            {
                break;
            }
        }

        // Return the solution
        let result = SimulationResult::new(best_solution, initial_time, iteration);
        Ok(result)
    }
}
