use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use rand::Rng;

use crate::algorithms::SimulatedAnnealing;
use crate::problems::errors::ProblemError;

/// Generic Knapsack problem struct
#[derive(Debug, Clone)]
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
            return Err(ProblemError::InitializationError);
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
        let contents = fs::read_to_string(file_path).expect("Error opening file");
        let chunks = contents.split("\n---\n");
        let problems: Vec<Result<Self, ProblemError>> = chunks
            .map(|chunk| {
                let mut lines = chunk.lines();
                let max_weight: f64 = lines.next().unwrap().parse().unwrap();
                let all_weights: Vec<f64> = lines
                    .next()
                    .unwrap()
                    .split(",")
                    .into_iter()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect();
                let all_values: Vec<f64> = lines
                    .next()
                    .unwrap()
                    .split(",")
                    .into_iter()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect();
                let optimal_value: f64 = lines.next().unwrap().parse().unwrap();
                KnapsackProblem::new(&all_values, &all_weights, max_weight, Some(optimal_value))
            })
            .collect();
        problems.into_iter().collect()
    }
}

/// Knapsack problem solution
#[derive(Debug, Clone)]
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
        let items = HashSet::from_iter(current_items.into_iter());
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
        number_items_in_set: usize,
        problem: &'a KnapsackProblem,
        rng: &mut impl Rng,
    ) -> Result<Self, ProblemError> {
        let mut current_items: HashSet<usize> = HashSet::new();
        while current_items.len() != number_items_in_set {
            let random_index = rng.random_range(1..problem.number_items);
            if !current_items.contains(&random_index) {
                current_items.insert(random_index);
            }
        }
        Self::new(current_items, problem)
    }
}

/// Implement the Simulated annealing methods for the knapsack problem
impl<'a> SimulatedAnnealing for KnapsackSolution<'a> {
    type NewSolutionError = ProblemError;

    fn objective(&self) -> f64 {
        if self.weight > self.problem.max_weight {
            return 0.0; // Worse possible objective
        }
        -self.value
    }

    fn new_solution(&self, rng: &mut impl Rng) -> Result<Self, Self::NewSolutionError> {
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
