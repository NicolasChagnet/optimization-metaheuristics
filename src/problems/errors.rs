use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ProblemError {
    #[error("Error initializing the problem: {0}")]
    InitializationError(&'static str),
    #[error("Error when creating a new solution to the problem: {0}")]
    NewSolutionError(&'static str),
}
