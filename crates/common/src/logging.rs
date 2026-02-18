use tracing::Level;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub type LogLevel = Level;

/// Initialize mission logging subsystem.
///
/// # Mission Context
/// All subsystems use structured logging with targets for filtering:
/// - `MISSION_CONTROL`: high-level mission events
/// - `TELEMETRY`: satellite health and status
/// - `COMMS`: radio link activity
/// - `ROUTING`: bundle path decisions
/// - `ORBITAL`: position and trajectory
pub fn init_logging(level: LogLevel) {
    let filter = EnvFilter::new(format!("{}", level));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .init();
}

/// Mission control logging macro.
///
/// # Example
/// ```
/// mission_log!(info, satellite = "HSRN-1", event = "ORBIT_INSERTION", "main engine cutoff");
/// ```
#[macro_export]
macro_rules! mission_log {
    ($level:ident, $($arg:tt)*) => {
        tracing::$level!(
            target: "MISSION_CONTROL",
            $($arg)*
        )
    };
}

/// Telemetry logging macro.
#[macro_export]
macro_rules! telemetry_log {
    ($level:ident, $($arg:tt)*) => {
        tracing::$level!(
            target: "TELEMETRY",
            $($arg)*
        )
    };
}

/// Communications logging macro.
#[macro_export]
macro_rules! comms_log {
    ($level:ident, $($arg:tt)*) => {
        tracing::$level!(
            target: "COMMS",
            $($arg)*
        )
    };
}

/// Routing logging macro.
#[macro_export]
macro_rules! routing_log {
    ($level:ident, $($arg:tt)*) => {
        tracing::$level!(
            target: "ROUTING",
            $($arg)*
        )
    };
}

/// Routing logging macro.
#[macro_export]
macro_rules! orbital_log {
    ($level:ident, $($arg:tt)*) => {
        tracing::$level!(
            target: "ORBITAL",
            $($arg)*
        )
    };
}