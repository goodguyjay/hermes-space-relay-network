//! HERMES SPACE RELAY NETWORK - COMMON SUBSYSTEMS
//!
//!        ___---___
//!    ___/         \___
//!   /               \
//!  |   Mission Core  |
//!   \___         ___/
//!       ---___---
//!
//! Shared types, logging infrastructure, and utilities.

pub mod error;
pub mod logging;
pub mod time;

pub use error::{HermesError, Result};
pub use logging::{LogLevel, init_logging};
pub use time::{SimulationTime, TimeAcceleration};
