use crate::constants::{J2000_JD_TT, SECONDS_PER_DAY};
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Julian Date in Terrestrial Time (TT).
///
/// A continuous count of days since noon, January 1, 4713 BC (Julian calendar).
/// This is the standard epoch-independent time representation for orbital mechanics.
///
/// # TT vs UTC
/// This type always represents TT, never UTC. The offset at J2000:
///   TT - UTC = 64.184 s (TAI + 32.184 s offset, plus 32 leap seconds at J2000)
/// This offset grows by 1 s each time a leap second is added.
///
/// Conversions to/from `DateTime<Utc>` apply the J2000-era offset (64.184 s).
/// This is sufficient for current simulation purposes.
/// For sub-second precision across decades `hifitime` will be implemented
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct JulianDate(f64);

impl JulianDate {
    /// J2000.0 epoch: 2000-01-01 12:00:00 TT = JD 2451545.0 TT.
    pub const J2000: Self = Self(J2000_JD_TT);

    pub fn new(jd: f64) -> Self {
        Self(jd)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    /// Elapsed seconds from `other` to `self`
    pub fn elapsed_seconds_since(&self, other: JulianDate) -> f64 {
        (self.0 - other.0) * SECONDS_PER_DAY
    }

    /// Advance by seconds
    ///
    /// # Returns
    /// **New** `JulianDate` advanced by given seconds.
    /// 
    /// # Note
    /// Precision degrades slightly for large values due to seconds->days->seconds roundtrip.
    /// The fix is storing a separate sub-day remainder field when hifitime is implemented
    pub fn add_seconds(&self, seconds: f64) -> Self {
        Self(self.0 + seconds / SECONDS_PER_DAY)
    }

    /// Advance by days
    /// # Returns
    /// **New** `JulianDate` advanced by given days.
    pub fn add_days(&self, days: f64) -> Self {
        Self(self.0 + days)
    }

    /// Convert to `DateTime<Utc>` by applying J2000-era TT-UTC offset.
    pub fn to_utc(&self) -> DateTime<Utc> {
        let seconds_from_j2000_tt = self.elapsed_seconds_since(Self::J2000);

        let j2000_utc = Utc
            .with_ymd_and_hms(2000, 1, 1, 11, 58, 55)
            .single()
            .expect("J2000 UTC is a valid fixed datetime")
            .checked_add_signed(Duration::milliseconds(816))
            .expect("J2000 UTC millisecond adjustment valid");

        let millis = (seconds_from_j2000_tt * 1000.0) as i64;
        j2000_utc
            .checked_add_signed(Duration::milliseconds(millis))
            .expect("JulianDate to UTC conversion overflow")
    }

    /// Convert from `DateTime<Utc>`
    pub fn from_utc(dt: DateTime<Utc>) -> Self {
        let j2000_utc = Self::J2000.to_utc();
        let delta_s = (dt - j2000_utc).num_milliseconds() as f64 / 1000.0;
        Self::J2000.add_seconds(delta_s)
    }
}

impl std::fmt::Display for JulianDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JD {:.6} TT", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use chrono::{Timelike};

    #[test]
    fn test_j2000_value() {
        assert_eq!(JulianDate::J2000.value(), 2451545.0);
    }

    #[test]
    fn test_elapsed_seconds_zero_at_same_epoch() {
        let elapsed = JulianDate::J2000.elapsed_seconds_since(JulianDate::J2000);
        assert_eq!(elapsed, 0.0);
    }

    #[test]
    fn test_add_and_elapsed_roundtrip() {
        let jd = JulianDate::J2000.add_seconds(3600.0);
        let elapsed = jd.elapsed_seconds_since(JulianDate::J2000);
        assert_relative_eq!(elapsed, 3600.0, epsilon = 1e-3);
    }

    #[test]
    fn test_add_days() {
        let jd = JulianDate::J2000.add_days(1.0);
        assert_relative_eq!(
            jd.elapsed_seconds_since(JulianDate::J2000),
            86_400.0,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_j2000_to_utc_date() {
        let utc = JulianDate::J2000.to_utc();
        // J2000 TT = 2000-01-01 11:58:55.816 UTC
        assert_eq!(utc.date_naive().to_string(), "2000-01-01");
        assert_eq!(utc.time().hour(), 11);
        assert_eq!(utc.time().minute(), 58);
    }

    #[test]
    fn test_from_utc_roundtrip() {
        let original = JulianDate::J2000.add_seconds(123_456.789);
        let utc = original.to_utc();
        let recovered = JulianDate::from_utc(utc);
        // millisecond precision expected
        assert_relative_eq!(original.value(), recovered.value(), epsilon = 1e-3);
    }

    #[test]
    fn test_display() {
        let s = format!("{}", JulianDate::J2000);
        assert!(s.contains("2451545.000000"));
        assert!(s.contains("TT"));
    }
}
