use crate::error::{HermesError, Result};
use crate::julian_date::JulianDate;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Simulation time tracking.
///
/// Decouples simulated time from wall clock time to allow time acceleration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationTime {
    /// Current simulation time
    pub current: JulianDate,
    /// Simulation start time
    pub epoch: JulianDate,
    /// Time acceleration multiplier
    pub acceleration: TimeAcceleration,
}

impl SimulationTime {
    /// Create new simulation starting at given epoch.
    pub fn new(epoch: JulianDate, acceleration: TimeAcceleration) -> Self {
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

        let sim_seconds = sim_nanos as f64 / 1_000_000_000.0;
        self.current = self.current.add_seconds(sim_seconds);
    }

    /// Get elapsed simulation time in seconds since epoch.
    pub fn elapsed_seconds(&self) -> f64 {
        self.current.elapsed_seconds_since(self.epoch)
    }

    /// Current time as UTC (for logging purposes only).
    pub fn to_utc(&self) -> DateTime<Utc> {
        self.current.to_utc()
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use chrono::{Duration, Timelike};

    #[test]
    fn test_new_starts_at_epoch() {
        let sim = SimulationTime::new(JulianDate::J2000, TimeAcceleration::Realtime);
        assert_eq!(sim.elapsed_seconds(), 0.0);
    }

    #[test]
    fn test_realtime_advance() {
        let mut sim = SimulationTime::new(JulianDate::J2000, TimeAcceleration::Realtime);
        sim.advance(Duration::seconds(60));
        assert_relative_eq!(sim.elapsed_seconds(), 60.0, epsilon = 1e-6);
    }

    #[test]
    fn test_fast_acceleration() {
        let mut sim = SimulationTime::new(JulianDate::J2000, TimeAcceleration::Fast);
        sim.advance(Duration::seconds(1));
        // 1 wall second -> 100x = 100 sim seconds
        assert_relative_eq!(sim.elapsed_seconds(), 100.0, epsilon = 1e-6);
    }

    #[test]
    fn test_ludicrous_acceleration() {
        let mut sim = SimulationTime::new(JulianDate::J2000, TimeAcceleration::Ludicrous);
        sim.advance(Duration::seconds(1));
        // 1 wall second -> 10000x = 10000 sim seconds
        assert_relative_eq!(sim.elapsed_seconds(), 10_000.0, epsilon = 1e-6);
    }

    #[test]
    fn test_27_minute_dtn_delay_at_ludicrous() {
        let mut sim = SimulationTime::new(JulianDate::J2000, TimeAcceleration::Ludicrous);
        let twenty_seven_minutes_sim = 27.0 * 60.0; // 27 minutes in seconds
        let wall_seconds_needed = twenty_seven_minutes_sim / 10_000.0;
        let wall_millis = (wall_seconds_needed * 1000.0) as i64;

        sim.advance(Duration::milliseconds(wall_millis));

        assert_relative_eq!(
            sim.elapsed_seconds(),
            twenty_seven_minutes_sim,
            epsilon = 1.0 // within 1 second given millisecond wall resolution
        )
    }

    #[test]
    fn test_to_utc_advances_correctly() {
        let mut sim = SimulationTime::new(JulianDate::J2000, TimeAcceleration::Realtime);
        sim.advance(Duration::hours(1));
        let utc = sim.to_utc();
        // should be ~12:58:55 UTC
        assert_eq!(utc.time().hour(), 12);
    }
}
