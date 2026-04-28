//! Simple two-body Keplerian ephemeris for solar system bodies.
//!
//! Provides ECI positions for the Sun and Mars by propagating from
//! J2000 orbital elements using two-body Keplerian dynamics.
//!
//! # Fidelity
//! This is a Keplerian (two-body) ephemeris. No perturbations, no
//! oblateness, no relativistic corrections. Positional errors grow over
//! time but are acceptable for DTN contact scheduling at interplanetary
//! distances where timing precision of minutes is sufficient.
//!
//! It'll be replaced with JPL DE440 lookup via JPL Horizons API later down the road.
//!
//! # Coordinate Frame
//! All positions are in Earth-Centered Inertial (ECI) J2000 frame (km).
//! Sun position is derived by negating Earth's heliocentric position.

use crate::contact::EphemerisProvider;
use crate::coordinates::Vector3;
use crate::propagator::OrbitalElements;
use hsrn_common::constants::{mars, sun, AU_KM};
use hsrn_common::julian_date::JulianDate;

/// Earth's heliocentric orbital elements at J2000
mod earth_j2000 {
    use hsrn_common::constants::DEG_TO_RAD;

    pub const SEMI_MAJOR_AXIS_KM: f64 = 149_598_023.0;
    pub const ECCENTRICITY: f64 = 0.016_708_617;
    pub const INCLINATION_RAD: f64 = 0.0;
    pub const RAAN_RAD: f64 = 0.0;
    pub const ARG_PERIAPSIS_RAD: f64 = 102.937_348_08 * DEG_TO_RAD;
    pub const TRUE_ANOMALY_RAD: f64 = (100.466_448_51 - 102.937_348_08) * DEG_TO_RAD;
}

/// Mars heliocentric orbital elements at J2000
mod mars_j2000 {
    use hsrn_common::constants::DEG_TO_RAD;

    pub const SEMI_MAJOR_AXIS_KM: f64 = 227_939_186.0;
    pub const ECCENTRICITY: f64 = 0.093_400_620;
    pub const INCLINATION_RAD: f64 = 1.849_726_48 * DEG_TO_RAD;
    pub const RAAN_RAD: f64 = 49.558_093_21 * DEG_TO_RAD;
    pub const ARG_PERIAPSIS_RAD: f64 = 286.502_140_77 * DEG_TO_RAD;
    pub const TRUE_ANOMALY_RAD: f64 = (355.433_274_63 - 336.060_233_98) * DEG_TO_RAD;
}

/// Keplerian ephemeris for Sun and Mars in ECI
///
/// Constructed once and queried for any simulation time
pub struct KeplerianEphemeris {
    earth_elements: OrbitalElements,
    mars_elements: OrbitalElements,
}

impl KeplerianEphemeris {
    /// Construct ephemeris from J2000 elements.
    pub fn new() -> Self {
        Self {
            earth_elements: OrbitalElements {
                semi_major_axis_km: earth_j2000::SEMI_MAJOR_AXIS_KM,
                eccentricity: earth_j2000::ECCENTRICITY,
                inclination_rad: earth_j2000::INCLINATION_RAD,
                raan_rad: earth_j2000::RAAN_RAD,
                argument_of_periapsis_rad: earth_j2000::ARG_PERIAPSIS_RAD,
                true_anomaly_rad: earth_j2000::TRUE_ANOMALY_RAD,
                epoch: JulianDate::J2000,
                mu_km3_s2: sun::MU_KM3_PER_S2,
            },
            mars_elements: OrbitalElements {
                semi_major_axis_km: mars_j2000::SEMI_MAJOR_AXIS_KM,
                eccentricity: mars_j2000::ECCENTRICITY,
                inclination_rad: mars_j2000::INCLINATION_RAD,
                raan_rad: mars_j2000::RAAN_RAD,
                argument_of_periapsis_rad: mars_j2000::ARG_PERIAPSIS_RAD,
                true_anomaly_rad: mars_j2000::TRUE_ANOMALY_RAD,
                epoch: JulianDate::J2000,
                mu_km3_s2: sun::MU_KM3_PER_S2,
            },
        }
    }

    /// Earth's heliocentric ECI position at time (km)
    ///
    /// "Heliocentric ECI" here means origin at Sun, x-axis toward vernal equinox
    /// This is used internally to derive the Sun's geocentric position
    pub fn earth_heliocentric_eci(&self, time: JulianDate) -> Vector3 {
        self.earth_elements.propagate_to(time).to_eci().position_km
    }

    /// Mars heliocentric ECI position at time (km)
    pub fn mars_heliocentric_eci(&self, time: JulianDate) -> Vector3 {
        self.mars_elements.propagate_to(time).to_eci().position_km
    }
}

impl Default for KeplerianEphemeris {
    fn default() -> Self {
        Self::new()
    }
}

impl EphemerisProvider for KeplerianEphemeris {
    /// Sun's geocentric ECI position (km)
    fn sun_position_eci(&self, time: JulianDate) -> Vector3 {
        -self.earth_heliocentric_eci(time)
    }

    /// Mars geocentric ECI position (km)
    fn mars_position_eci(&self, time: JulianDate) -> Vector3 {
        self.mars_heliocentric_eci(time) - self.earth_heliocentric_eci(time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_earth_sun_distance_at_j2000() {
        let eph = KeplerianEphemeris::new();
        let earth_pos = eph.earth_heliocentric_eci(JulianDate::J2000);
        let distance = earth_pos.norm();

        // should be ~1 au +- 2% (near perihelion in January)
        assert_relative_eq!(distance, AU_KM, epsilon = AU_KM * 0.02);
    }

    #[test]
    fn test_sun_position_is_negated_earth() {
        let eph = KeplerianEphemeris::new();
        let sun = eph.sun_position_eci(JulianDate::J2000);
        let earth = eph.earth_heliocentric_eci(JulianDate::J2000);
        assert_relative_eq!(sun.x, -earth.x, epsilon = 1.0);
        assert_relative_eq!(sun.y, -earth.y, epsilon = 1.0);
        assert_relative_eq!(sun.z, -earth.z, epsilon = 1.0);
    }

    #[test]
    fn test_mars_sun_distance_at_j2000() {
        let eph = KeplerianEphemeris::new();
        let mars_helio = eph.mars_heliocentric_eci(JulianDate::J2000);
        let distance = mars_helio.norm();
        // Mars is ~1.524 AU from Sun, +- 10% for eccentricity
        assert_relative_eq!(distance, mars::SEMI_MAJOR_AXIS_KM, epsilon = mars::SEMI_MAJOR_AXIS_KM * 0.10);
    }

    #[test]
    fn test_earth_completes_orbit_in_one_year() {
        let eph = KeplerianEphemeris::new();
        let p0 = eph.earth_heliocentric_eci(JulianDate::J2000);
        let p1 = eph.earth_heliocentric_eci(
            JulianDate::J2000.add_seconds(hsrn_common::constants::earth::ORBITAL_PERIOD_S)
        );
        // should return close to starting position
        assert_relative_eq!(p0.x, p1.x, epsilon = AU_KM * 0.01);
        assert_relative_eq!(p0.y, p1.y, epsilon = AU_KM * 0.01);
    }

    #[test]
    fn test_mars_earth_distance_in_bounds() {
        let eph = KeplerianEphemeris::new();
        let mars_geo = eph.mars_position_eci(JulianDate::J2000);
        let distance = mars_geo.norm();
        // Mars-Earth distance always between ~55M and ~401M km
        assert!(distance > 55_000_000.0, "Mars too close: {} km", distance);
        assert!(distance < 401_000_000.0, "Mars too far: {} km", distance);
    }
}
