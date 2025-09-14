#[cfg(test)]
mod tests {
    use optimization_metaheuristics::algorithms::{GeneticAlgorithm, GeneticAlgorithmConfig};
    use optimization_metaheuristics::problems::{KnapsackProblem, KnapsackSolution};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;
    use std::fs;
    use std::io;

    #[test]
    fn test_knapsack_genetic_algorithm() {
        let files = fs::read_dir("./tests/knapsack")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();
        let problems: Vec<KnapsackProblem> = files
            .iter()
            .map(|file| KnapsackProblem::load_from_file(file).unwrap())
            .flatten()
            .collect();
        for problem in problems {
            let mut rng = SmallRng::seed_from_u64(654321);
            let config = GeneticAlgorithmConfig::new(1000, 100, 0.2, 4).unwrap();
            let ga = GeneticAlgorithm::new(config);
            let initial_solutions = (1..ga.config.population_size)
                .map(|_| KnapsackSolution::new_random(None, &problem, &mut rng).unwrap())
                .collect();
            let solution = ga.execute(initial_solutions, &mut rng).unwrap();
            assert!(
                solution.value == problem.optimal_value.unwrap(),
                "Expected {}, found {}.",
                problem.optimal_value.unwrap(),
                solution.value
            )
        }
    }
}
