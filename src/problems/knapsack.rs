use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use rand::Rng;
use rand::seq::SliceRandom;
use rand::seq::index::sample;

use crate::algorithms::{GeneticCompatible, SimulatedAnnealing};
use crate::problems::ProblemSolution;
use crate::problems::errors::ProblemError;

/// Generic Knapsack problem struct
#[derive(Debug, Clone, PartialEq)]
pub struct KnapsackProblem {
    /// Number of items possible in the knapsack
    pub number_items: usize,
    /// Maximum weight allowed
    pub max_weight: f64,
    /// All values of items in the knapsack
    pub all_values: Vec<f64>,
    /// All weights of items in the knapsack
    pub all_weights: Vec<f64>,
    /// Optimal value if known
    pub optimal_value: Option<f64>,
}

impl KnapsackProblem {
    /// Constructor
    pub fn new(
        all_values: &[f64],
        all_weights: &[f64],
        max_weight: f64,
        optimal_value: Option<f64>,
    ) -> Result<Self, ProblemError> {
        let number_items = all_values.len();
        if all_weights.len() != number_items {
            return Err(ProblemError::InitializationError(
                "the weights and values should have the same length.",
            ));
        }

        Ok(KnapsackProblem {
            number_items,
            max_weight,
            all_values: Vec::from(all_values),
            all_weights: Vec::from(all_weights),
            optimal_value,
        })
    }

    /// Easily load it from file
    pub fn load_from_file(file_path: &PathBuf) -> Result<Vec<Self>, ProblemError> {
        let contents = fs::read_to_string(file_path)
            .map_err(|_| ProblemError::InitializationError("file was not found"))?;
        let chunks = contents.split("\n---\n");
        let problems: Vec<Self> = chunks
            .map(|chunk| {
                let mut lines = chunk.lines();
                let max_weight: f64 = lines
                    .next()
                    .ok_or(ProblemError::InitializationError(
                        "line for maximal weight not found",
                    ))?
                    .parse()
                    .map_err(|_| {
                        ProblemError::InitializationError("could not parse maximum weight")
                    })?;
                let all_weights: Vec<f64> = lines
                    .next()
                    .ok_or(ProblemError::InitializationError(
                        "line for weights not found",
                    ))?
                    .split(",")
                    .map(|x| {
                        x.parse::<f64>().map_err(|_| {
                            ProblemError::InitializationError("could not parse weight value")
                        })
                    })
                    .collect::<Result<Vec<f64>, ProblemError>>()?;
                let all_values: Vec<f64> = lines
                    .next()
                    .ok_or(ProblemError::InitializationError(
                        "line for values not found",
                    ))?
                    .split(",")
                    .map(|x| {
                        x.parse::<f64>().map_err(|_| {
                            ProblemError::InitializationError("could not parse weight value")
                        })
                    })
                    .collect::<Result<Vec<f64>, ProblemError>>()?;
                let optimal_value: f64 = lines
                    .next()
                    .ok_or(ProblemError::InitializationError(
                        "line for optimal value not found",
                    ))?
                    .parse()
                    .map_err(|_| {
                        ProblemError::InitializationError("could not parse optimal value")
                    })?;
                KnapsackProblem::new(&all_values, &all_weights, max_weight, Some(optimal_value))
            })
            .collect::<Result<Vec<Self>, ProblemError>>()?;
        Ok(problems)
    }
}

/// Knapsack problem solution
#[derive(Debug, Clone, PartialEq)]
pub struct KnapsackSolution<'a> {
    /// Current items selected
    pub items: HashSet<usize>,
    /// Current value
    pub value: f64,
    /// Current weight
    pub weight: f64,
    /// Reference to the problem
    problem: &'a KnapsackProblem,
}

impl<'a> KnapsackSolution<'a> {
    pub fn new<T>(current_items: T, problem: &'a KnapsackProblem) -> Result<Self, ProblemError>
    where
        T: IntoIterator<Item = usize>,
    {
        let items = HashSet::from_iter(current_items);
        let value = items
            .iter()
            .fold(0.0, |acc, &i| acc + problem.all_values[i]);
        let weight = items
            .iter()
            .fold(0.0, |acc, &i| acc + problem.all_weights[i]);

        Ok(KnapsackSolution {
            items,
            value,
            weight,
            problem,
        })
    }

    pub fn new_random(
        number_items_in_set: Option<usize>,
        problem: &'a KnapsackProblem,
        rng: &mut impl Rng,
    ) -> Result<Self, ProblemError> {
        let number_items_in_set =
            number_items_in_set.unwrap_or(rng.random_range(1..=problem.number_items));
        let mut current_items: HashSet<usize> = HashSet::new();
        while current_items.len() != number_items_in_set {
            let random_index = rng.random_range(0..problem.number_items);
            if !current_items.contains(&random_index) {
                current_items.insert(random_index);
            }
        }
        Self::new(current_items, problem)
    }
}

/// Partial ordering implementation for this solution
impl<'a> PartialOrd for KnapsackSolution<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.objective().partial_cmp(&other.objective())
    }
}

///
impl<'a> ProblemSolution for KnapsackSolution<'a> {
    fn objective(&self) -> f64 {
        if self.weight > self.problem.max_weight {
            return 0.0; // Worst possible objective
        }
        -self.value
    }
}

/// Implement the Simulated annealing methods for the knapsack problem
impl<'a> SimulatedAnnealing for KnapsackSolution<'a> {
    fn new_solution(&self, rng: &mut impl Rng) -> Result<Self, ProblemError> {
        let mut new_items = self.items.clone();
        let random_index = rng.random_range(0..self.problem.number_items);
        let (new_value, new_weight) = match new_items.contains(&random_index) {
            // If the item is already in the set, remove it
            true => {
                new_items.remove(&random_index);
                (
                    self.value - self.problem.all_values[random_index],
                    self.weight - self.problem.all_weights[random_index],
                )
            }
            // Otherwise add it
            false => {
                new_items.insert(random_index);
                (
                    self.value + self.problem.all_values[random_index],
                    self.weight + self.problem.all_weights[random_index],
                )
            }
        };
        Ok(KnapsackSolution {
            items: new_items,
            value: new_value,
            weight: new_weight,
            problem: self.problem,
        })
    }
}

/// Implementation of the genetic algorithm
impl<'a> GeneticCompatible for KnapsackSolution<'a> {
    fn mutate(&mut self, mutation_rate: f64, rng: &mut impl Rng) -> Result<(), ProblemError> {
        let total_number_items: f64 = self.problem.number_items as f64;
        let expected_number_flips = usize::try_from(
            (mutation_rate * total_number_items)
                .clamp(0.0, total_number_items)
                .floor() as u64,
        )
        .map_err(|_| ProblemError::NewSolutionError("couldn't mutate the knapsack solution."))?;
        let indices = sample(rng, self.problem.number_items, expected_number_flips);
        for n in indices {
            if self.items.contains(&n) {
                self.items.remove(&n);
            } else {
                self.items.insert(n);
            }
        }
        Ok(())
    }
    fn generate_children_with(
        &self,
        other_parent: &Self,
        mut rng: &mut impl Rng,
    ) -> Result<Vec<Self>, ProblemError> {
        // Combine all elements from both sets
        let mut all_elements: Vec<usize> = self.items.union(&other_parent.items).cloned().collect();

        // Shuffle the combined elements
        all_elements.shuffle(&mut rng);

        // Split into two halves
        let split_at = all_elements.len() / 2;
        let (first_half, second_half) = all_elements.split_at(split_at);

        // Convert back to HashSets
        let items_child_1: HashSet<usize> = first_half.iter().cloned().collect();
        let items_child_2: HashSet<usize> = second_half.iter().cloned().collect();

        // Convert to children
        let children = vec![
            Self::new(items_child_1, self.problem)?,
            Self::new(items_child_2, self.problem)?,
        ];

        Ok(children)
    }
}
