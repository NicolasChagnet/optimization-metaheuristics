use std::fmt::Debug;

use rand::Rng;

use crate::{
    algorithms::{AlgorithmError, genetic_algorithm::config::GeneticAlgorithmConfig},
    problems::ProblemError,
};

/// Main trait for compatible solutions
pub trait GeneticCompatible: Clone + Debug + PartialOrd {
    fn mutate(&mut self, mutation_rate: f64, rng: &mut impl Rng) -> Result<(), ProblemError>;

    fn generate_children_with(
        &self,
        other_parent: &Self,
        rng: &mut impl Rng,
    ) -> Result<Vec<Self>, ProblemError>;
}

/// Population wrapper used by the Genetic Algorithm
struct Population<T: GeneticCompatible> {
    /// Elements of the population, in sorted order
    elements: Vec<T>,
}

impl<T: GeneticCompatible> Population<T> {
    pub fn new(capacity: usize) -> Self {
        Population {
            elements: Vec::with_capacity(capacity),
        }
    }
    /// Add elements
    pub fn add_individuals(&mut self, individuals: Vec<T>) {
        self.elements.extend(individuals);
    }
    /// Truncate elements
    pub fn truncate(&mut self, size: usize) {
        self.elements.truncate(size);
    }
    /// Sort the elements with minimal fitness first
    pub fn sort(&mut self) {
        self.elements
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }
    /// Simple top-k selection
    pub fn generate_offspring(
        &mut self,
        number_pairs_parents: usize,
        rng: &mut impl Rng,
    ) -> Result<Vec<T>, AlgorithmError> {
        // First generate the offsprings
        let offsprings_nested = (0..number_pairs_parents)
            .map(|idx| {
                self.elements[2 * idx]
                    .generate_children_with(&self.elements[2 * idx + 1], rng)
                    .map_err(|_| AlgorithmError::ExecutionError("could not generate offsprings"))
            })
            .collect::<Result<Vec<Vec<T>>, AlgorithmError>>()?;
        let offsprings: Vec<T> = offsprings_nested.into_iter().flatten().collect();
        Ok(offsprings)
    }
    /// Return the best solution
    pub fn best_individual(&self) -> Result<T, AlgorithmError> {
        if self.elements.is_empty() {
            return Err(AlgorithmError::ExecutionError("empty population"));
        }
        Ok(self.elements[0].clone())
    }
}

/// Main genetic algorithm to solve optimization problems
pub struct GeneticAlgorithm {
    pub config: GeneticAlgorithmConfig,
}

impl GeneticAlgorithm {
    pub fn new(config: GeneticAlgorithmConfig) -> Self {
        Self { config }
    }

    pub fn execute<T>(
        &self,
        initial_elements: Vec<T>,
        rng: &mut impl Rng,
    ) -> Result<T, AlgorithmError>
    where
        T: GeneticCompatible,
    {
        // Create the initial population
        let capacity = self.config.population_size + 2 * self.config.number_pairs_parents;
        let mut population: Population<T> = Population::new(capacity);
        population.add_individuals(initial_elements);
        population.sort();

        // Iterate over generations
        for _ in 1..=self.config.number_generations {
            // Generate offsprings
            let mut offsprings =
                population.generate_offspring(self.config.number_pairs_parents, rng)?;
            // Mutate offsprings with a probability
            for offspring in offsprings.iter_mut() {
                offspring
                    .mutate(self.config.mutation_rate, rng)
                    .map_err(|_| AlgorithmError::ExecutionError("could not mutate offspring"))?;
            }
            // Add the offsprings to the population, sort and truncate
            population.add_individuals(offsprings);
            population.sort();
            population.truncate(self.config.population_size);
        }
        population.best_individual()
    }
}
