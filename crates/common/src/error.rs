use thiserror::Error;

pub type Result<T> = std::result::Result<T, HermesError>;

/// Mission-critical error types.
#[derive(Error, Debug)]
pub enum HermesError {
    #[error("Orbital mechanics failure: {0}")]
    OrbitalMechanics(String),

    #[error("Communications failure: {0}")]
    Communications(String),

    #[error("Bundle protocol error: {0}")]
    BundleProtocol(String),

    #[error("Routing failure: {0}")]
    Routing(String),

    #[error("Telemetry subsystem error: {0}")]
    Telemetry(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
