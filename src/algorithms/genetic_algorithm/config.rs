use crate::algorithms::errors::AlgorithmError;

/// Configuration struct for the genetic algorithm
#[derive(Debug, Clone)]
pub struct GeneticAlgorithmConfig {
    /// Generations
    pub number_generations: usize,
    /// Population size
    pub population_size: usize,
    /// Mutation rate
    pub mutation_rate: f64,
    /// Number of pairs of parents to select per generation
    pub number_pairs_parents: usize,
    /// Possible stop criterion
    pub stop_threshold: Option<f64>,
}

impl Default for GeneticAlgorithmConfig {
    fn default() -> Self {
        Self {
            number_generations: 100,
            population_size: 100,
            mutation_rate: 0.1,
            number_pairs_parents: 2,
            stop_threshold: None,
        }
    }
}

impl GeneticAlgorithmConfig {
    pub fn new(
        number_generations: usize,
        population_size: usize,
        mutation_rate: f64,
        number_pairs_parents: usize,
        stop_threshold: Option<f64>,
    ) -> Result<Self, AlgorithmError> {
        // Validate the data
        if !(0.0..=1.0).contains(&mutation_rate) {
            return Err(AlgorithmError::ConfigurationError(
                "the mutation rate should be between 0 and 1.",
            ));
        }
        if 2 * number_pairs_parents > population_size {
            return Err(AlgorithmError::ConfigurationError(
                "the population of size should be higher than the number of parents selected at each generation",
            ));
        }
        Ok(GeneticAlgorithmConfig {
            number_generations,
            population_size,
            mutation_rate,
            number_pairs_parents,
            stop_threshold,
        })
    }
}
