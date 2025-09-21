/// Trait describing a generic solution
pub trait ProblemSolution {
    /// All solutions are expected to have an objective value which must be minimized.
    fn objective(&self) -> f64;
}
