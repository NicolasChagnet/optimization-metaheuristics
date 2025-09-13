#[cfg(test)]
mod tests {
    use optimization_metaheuristics::algorithms::{
        SimulatedAnnealingAlgorithm, SimulatedAnnealingConfig,
    };
    use optimization_metaheuristics::problems::{KnapsackProblem, KnapsackSolution};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;
    use std::fs;
    use std::io;

    #[test]
    fn test_knapsack_simulated_annealing() {
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
            let config = SimulatedAnnealingConfig {
                max_iterations: 1_000,
                cooling_rate: 0.999,
                initial_temperature: 10.0,
                ..Default::default()
            };
            let mut sa = SimulatedAnnealingAlgorithm::new(config);
            let initial_solution = KnapsackSolution::new(vec![], &problem).unwrap();
            let solution = sa.execute(initial_solution, &mut rng).unwrap();
            assert!(
                solution.value == problem.optimal_value.unwrap(),
                "Expected {}, found {}.",
                problem.optimal_value.unwrap(),
                solution.value
            )
        }
    }
}
