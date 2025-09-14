use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AlgorithmError {
    #[error("Error generating the configuration object: {0}")]
    ConfigurationError(&'static str),
    #[error("Error when running the algorithm: {0}")]
    ExecutionError(&'static str),
}
