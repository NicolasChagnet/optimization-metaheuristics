use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProblemError {
    #[error("Error initializing the problem")]
    InitializationError,
    #[error("Error when creating a new solution to the problem")]
    NewSolutionError,
}
