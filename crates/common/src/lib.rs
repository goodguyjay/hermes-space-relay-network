//! HERMES SPACE RELAY NETWORK - COMMON SUBSYSTEMS
//!```text
//!        ___---___
//!    ___/         \___
//!   /               \
//!  |   Mission Core  |
//!   \___         ___/
//!       ---___---
//!```
//! Shared types, logging infrastructure, and utilities.

pub mod error;
pub mod logging;
pub mod time;
pub mod julian_date;
pub mod constants;

pub use error::{HermesError, Result};
pub use logging::{init_logging, LogLevel};
pub use time::{SimulationTime, TimeAcceleration};
pub use constants::*;