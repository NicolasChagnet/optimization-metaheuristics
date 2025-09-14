mod errors;
mod genetic_algorithm;
mod simulated_annealing;

pub use crate::algorithms::errors::AlgorithmError;
pub use crate::algorithms::genetic_algorithm::{
    algorithm::GeneticAlgorithm, algorithm::GeneticCompatible, config::GeneticAlgorithmConfig,
};
pub use crate::algorithms::simulated_annealing::{
    algorithm::SimulatedAnnealing, algorithm::SimulatedAnnealingAlgorithm,
    config::SimulatedAnnealingConfig,
};
