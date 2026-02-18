use crate::error::{HermesError, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Simulation time tracking.
///
/// Decouples simulated time from wall clock time to allow time acceleration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationTime {
    /// Current simulated timestamp (UTC)
    pub current: DateTime<Utc>,
    /// Simulation start time
    pub epoch: DateTime<Utc>,
    /// Time acceleration multiplier
    pub acceleration: TimeAcceleration,
}

impl SimulationTime {
    /// Create new simulation starting at given epoch.
    pub fn new(epoch: DateTime<Utc>, acceleration: TimeAcceleration) -> Self {
        Self {
            current: epoch,
            epoch,
            acceleration,
        }
    }

    /// Advance time simulation by wall clock duration.
    ///
    /// # Panics
    /// Panics on overflow. Call `TimeAcceleration::validate_for_duration()`
    /// during initialization to ensure safety
    pub fn advance(&mut self, wall_duration: Duration) {
        let wall_nanos = wall_duration
            .num_nanoseconds()
            .expect("Wall duration exceeds representable range.");

        let multiplier = i64::from(self.acceleration.multiplier());

        let sim_nanos = wall_nanos
            .checked_mul(multiplier)
            .expect("Time acceleration will overflow with given wall duration.");

        let sim_duration = Duration::nanoseconds(sim_nanos);

        self.current = self
            .current
            .checked_add_signed(sim_duration)
            .expect("Simulation time overflow.")
    }

    /// Get elapsed simulation time since epoch.
    pub fn elapsed(&self) -> Duration {
        self.current.signed_duration_since(self.epoch)
    }
}

/// Time acceleration configuration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeAcceleration {
    /// Real-time (1:1)
    Realtime,
    /// Fast (100x)
    Fast,
    /// Ludicrous (10000x)
    Ludicrous,
    /// Custom multiplier
    Custom(u32),
}

impl TimeAcceleration {
    pub fn multiplier(&self) -> u32 {
        match self {
            Self::Realtime => 1,
            Self::Fast => 100,
            Self::Ludicrous => 10_000,
            Self::Custom(x) => *x,
        }
    }

    /// Validate that acceleration won't overflow for given duration.
    ///
    /// # Mission Context
    /// Call during initialization to verify simulation parameters
    /// are safe before starting mission timeline.
    pub fn validate_for_duration(&self, max_duration: Duration) -> Result<()> {
        let max_nanos = max_duration.num_nanoseconds().ok_or_else(|| {
            HermesError::Configuration("Duration exceeds representable range".into())
        })?;

        let multiplier = i64::from(self.multiplier());

        max_nanos.checked_mul(multiplier).ok_or_else(|| {
            HermesError::Configuration(format!(
                "Time acceleration {}x will overflow with duration {:?}. \
                Maximum safe duration: {:?}",
                multiplier,
                max_duration,
                Duration::nanoseconds(i64::MAX / multiplier)
            ))
        })?;

        Ok(())
    }
}
