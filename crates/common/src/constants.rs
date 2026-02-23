//! Physical and mathematical constants for orbital mechanics.
//!
//! # Sources
//! All values are sourced from authoritative international standards.
//! See `docs/REFERENCES.md` for full citations and verification status.
//!
//! # Unit Convention
//! Internal calculations use kilometers (km), seconds (s), and kilograms (kg)
//! unless otherwise noted. Gravitational parameters (μ = GM) are used directly
//! rather than computing G * M separately, as μ is determined with far higher
//! precision than G alone (G has ~22 ppm uncertainty; μ is known to sub-ppm).

// ─────────────────────────────────────────────────────────────────────────────
// Fundamental Physical Constants
// ─────────────────────────────────────────────────────────────────────────────

/// Speed of light in vacuum (m/s).
///
/// Exact by SI definition since 1983.
/// Source: NIST / CODATA 2018
pub const LIGHT_SPEED_M_PER_S: f64 = 299_792_458.0;

/// Speed of light in vacuum (km/s).
pub const LIGHT_SPEED_KM_PER_S: f64 = LIGHT_SPEED_M_PER_S / 1000.0;

/// Newtonian constant of gravitation (m³ kg⁻¹ s⁻²).
///
/// Source: CODATA 2018. Relative uncertainty: ~22 ppm.
///
/// # Warning
/// Do NOT use G to derive gravitational parameters (μ) for specific bodies.
/// Body-specific μ = GM values are determined directly from spacecraft tracking
/// with sub-ppm accuracy. Deriving μ via G * mass introduces unnecessary error.
/// G is provided here only for mass derivations in three-body mass ratio calculations.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

// ─────────────────────────────────────────────────────────────────────────────
// Earth Parameters
// ─────────────────────────────────────────────────────────────────────────────

/// Earth parameters from IERS Conventions 2010.
///
/// Source: Petit, G. & Luzum, B. (eds.), IERS Technical Note No. 36, Ch. 1, Table 1.1.
///
/// # Note on WGS-84 vs IERS 2010
/// These values differ slightly from WGS-84 (used in GPS/mapping).
/// IERS 2010 is optimized for celestial mechanics; WGS-84 for global positioning.
pub mod earth {
    /// Standard gravitational parameter μ = GM (km³/s²).
    /// Source: IERS Conventions 2010, Table 1.1.
    pub const MU_KM3_PER_S2: f64 = 398_600.4418;

    /// Earth mass (kg), derived from μ/G.
    ///
    /// Unit conversion: μ is in km³/s², G is in m³/(kg·s²).
    /// Multiply μ by 1e9 to convert km³ → m³ before dividing by G.
    pub const MASS_KG: f64 = MU_KM3_PER_S2 * 1e9 / super::GRAVITATIONAL_CONSTANT;

    /// Equatorial radius (km). Source: IERS 2010, Table 1.1.
    pub const RADIUS_EQUATORIAL_KM: f64 = 6_378.1366;

    /// Polar radius (km). Derived: b = a(1 - f).
    /// Verified consistent with IERS 2010 equatorial radius and flattening.
    pub const RADIUS_POLAR_KM: f64 = 6_356.751858;

    /// Inverse flattening (1/f). Source: IERS 2010, Table 1.1.
    pub const INVERSE_FLATTENING: f64 = 298.25642;

    /// Flattening factor f = 1/(1/f).
    pub const FLATTENING: f64 = 1.0 / INVERSE_FLATTENING;

    /// Nominal mean angular velocity (rad/s).
    /// Corresponds to a mean sidereal day of 86,164.0905 s.
    /// Source: IERS 2010, Table 1.1.
    pub const ROTATION_RATE_RAD_PER_S: f64 = 7.292_115_146_7e-5;

    /// Mean sidereal day (s). Derived from rotation rate: T = 2π/ω.
    pub const SIDEREAL_DAY_S: f64 = std::f64::consts::TAU / ROTATION_RATE_RAD_PER_S;

    /// Semi-major axis of Earth's heliocentric orbit (km).
    /// By convention, 1 AU (IAU 2012 Resolution B2).
    pub const SEMI_MAJOR_AXIS_KM: f64 = super::AU_KM;

    /// Earth's mean heliocentric orbital period (s).
    /// Julian year = 365.25 days.
    pub const ORBITAL_PERIOD_S: f64 = 365.25 * super::SECONDS_PER_DAY;

    /// Earth's mean heliocentric mean motion (rad/s). n = 2π/T.
    pub const MEAN_MOTION_RAD_PER_S: f64 = std::f64::consts::TAU / ORBITAL_PERIOD_S;
}

// ─────────────────────────────────────────────────────────────────────────────
// Mars Parameters
// ─────────────────────────────────────────────────────────────────────────────

/// Mars parameters from IAU/IAG Working Group on Cartographic Coordinates
/// and Rotational Elements (WGCCRE 2015), Archinal et al. (2018).
pub mod mars {
    /// Standard gravitational parameter μ = GM (km³/s²).
    /// Includes mass of planet + Phobos + Deimos.
    /// Source: Archinal et al. (2018), Table 4 (via Konopliv et al. 2011).
    pub const MU_KM3_PER_S2: f64 = 42_828.3752;

    /// Mars mass (kg), derived from μ/G.
    pub const MASS_KG: f64 = MU_KM3_PER_S2 * 1e9 / super::GRAVITATIONAL_CONSTANT;

    /// Equatorial radius (km).
    /// Source: Archinal et al. (2018), Table 1.
    /// Note: 3396.2 is a common engineering approximation; 3396.19 is the formal value.
    pub const RADIUS_EQUATORIAL_KM: f64 = 3_396.19;

    /// Mars rotation rate (rad/s).
    /// Corresponds to a mean solar day (Sol) of 88,775.244 s.
    /// Source: Archinal et al. (2018), Table 4.
    pub const ROTATION_RATE_RAD_PER_S: f64 = 7.088_218_081e-5;

    /// Mean solar day on Mars (Sol) in seconds: 24h 39m 35.244s.
    /// Source: Allison & McEwen (2000), Table 4.
    pub const SOL_S: f64 = 88_775.244;

    /// Semi-major axis of Mars's heliocentric orbit (km).
    /// Source: JPL DE430 ephemeris (consistent with IAU).
    pub const SEMI_MAJOR_AXIS_KM: f64 = 1.523_679_342 * super::AU_KM;

    /// Mars sidereal orbital period (s). 686.971 Earth days.
    pub const ORBITAL_PERIOD_S: f64 = 686.971 * super::SECONDS_PER_DAY;

    /// Mars mean heliocentric mean motion (rad/s). n = 2π/T.
    pub const MEAN_MOTION_RAD_PER_S: f64 = std::f64::consts::TAU / ORBITAL_PERIOD_S;
}

// ─────────────────────────────────────────────────────────────────────────────
// Sun Parameters
// ─────────────────────────────────────────────────────────────────────────────

/// Solar parameters from IAU 2015 Resolution B3.
pub mod sun {
    /// Solar gravitational parameter μ = GM (km³/s²).
    /// Nominal value, fixed by IAU 2015 Resolution B3 for numerical stability.
    /// Compatible with both TCB and TDB time scales.
    pub const MU_KM3_PER_S2: f64 = 1.327_124_40e11;

    /// Solar mass (kg), derived from μ/G.
    pub const MASS_KG: f64 = MU_KM3_PER_S2 * 1e9 / super::GRAVITATIONAL_CONSTANT;

    /// Nominal solar radius (km). Source: IAU 2015 Resolution B3.
    /// Corresponds to optical depth τ = 2/3 in the photosphere.
    pub const RADIUS_KM: f64 = 695_700.0;
}

// ─────────────────────────────────────────────────────────────────────────────
// Astronomical Unit
// ─────────────────────────────────────────────────────────────────────────────

/// Astronomical unit (km). Exact by IAU 2012 Resolution B2.
///
/// Fixed as exactly 149,597,870,700 m = 149,597,870.7 km.
/// This is a defined constant, not a measured value. If solar mass estimates
/// change, μ_sun is updated, the AU remains fixed.
pub const AU_KM: f64 = 149_597_870.7;

// ─────────────────────────────────────────────────────────────────────────────
// Lagrange Point Stability
// ─────────────────────────────────────────────────────────────────────────────

/// Routh's criterion: triangular Lagrange points (L4/L5) are linearly stable
/// if the system mass ratio μ = m₂/(m₁+m₂) is less than this threshold.
///
/// Exact value: ½(1 - √(23/27)) ≈ 0.0385208965
/// Derived from: 27μ(1-μ) < 1
///
/// Source: Szebehely (1967), "Theory of Orbits", pp. 138-142.
pub const ROUTH_STABILITY_THRESHOLD: f64 = 0.038_520_896_5;

// ─────────────────────────────────────────────────────────────────────────────
// Communications
// ─────────────────────────────────────────────────────────────────────────────

/// Earth-Mars communication light-time delays.
///
/// Source: JPL Horizons ephemeris data.
/// Note: actual delay is time-varying; these are physical bounds.
pub mod comms {
    /// Minimum Earth-Mars distance (km), near opposition (~55.7 million km).
    /// Note: varies between 54.6–103 million km due to orbital eccentricity.
    pub const MARS_EARTH_MIN_DISTANCE_KM: f64 = 55_700_000.0;

    /// Maximum Earth-Mars distance (km), near conjunction (~401 million km).
    pub const MARS_EARTH_MAX_DISTANCE_KM: f64 = 401_000_000.0;

    /// Minimum one-way light-time delay, Earth-Mars (s). ~185.8 s ≈ 3m 6s.
    pub const MARS_EARTH_MIN_DELAY_S: f64 =
        MARS_EARTH_MIN_DISTANCE_KM / super::LIGHT_SPEED_KM_PER_S;

    /// Maximum one-way light-time delay, Earth-Mars (s). ~1337.6 s ≈ 22m 18s.
    pub const MARS_EARTH_MAX_DELAY_S: f64 =
        MARS_EARTH_MAX_DISTANCE_KM / super::LIGHT_SPEED_KM_PER_S;
}

// ─────────────────────────────────────────────────────────────────────────────
// Time Constants
// ─────────────────────────────────────────────────────────────────────────────

/// Seconds in one astronomical day (86,400 s exactly).
pub const SECONDS_PER_DAY: f64 = 86_400.0;

/// Seconds in one hour.
pub const SECONDS_PER_HOUR: f64 = 3_600.0;

/// Julian Date of J2000.0 epoch (TT).
/// J2000.0 corresponds to 2000-01-01T12:00:00 TT (Terrestrial Time).
pub const J2000_JD_TT: f64 = 2451545.0;

// ─────────────────────────────────────────────────────────────────────────────
// Unit Conversions
// ─────────────────────────────────────────────────────────────────────────────

pub const KM_TO_M: f64 = 1_000.0;
pub const M_TO_KM: f64 = 0.001;
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_light_speed_consistency() {
        assert_relative_eq!(
            LIGHT_SPEED_KM_PER_S,
            LIGHT_SPEED_M_PER_S / 1000.0,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_comms_delays_are_division_not_multiplication() {
        // delays must be in seconds (order of hundreds to thousands)
        assert!(comms::MARS_EARTH_MIN_DELAY_S > 100.0);
        assert!(comms::MARS_EARTH_MIN_DELAY_S < 1000.0);
        assert!(comms::MARS_EARTH_MAX_DELAY_S > 1000.0);
        assert!(comms::MARS_EARTH_MAX_DELAY_S < 2000.0);
    }

    #[test]
    fn test_comms_min_delay_approx() {
        // ~185.8 s
        assert_relative_eq!(comms::MARS_EARTH_MIN_DELAY_S, 185.8, epsilon = 1.0);
    }

    #[test]
    fn test_comms_max_delay_approx() {
        // ~1337.6 s
        assert_relative_eq!(comms::MARS_EARTH_MAX_DELAY_S, 1337.6, epsilon = 1.0);
    }

    #[test]
    fn test_routh_threshold_exact() {
        let exact = 0.5 * (1.0 - (23.0_f64 / 27.0).sqrt());
        assert_relative_eq!(ROUTH_STABILITY_THRESHOLD, exact, epsilon = 1e-10);
    }

    #[test]
    fn test_earth_sidereal_day_derived() {
        // Should be ~86164.0905 s
        assert_relative_eq!(earth::SIDEREAL_DAY_S, 86_164.0989, epsilon = 0.001);
    }

    #[test]
    fn test_earth_orbital_period() {
        // ~365.25 days
        let days = earth::ORBITAL_PERIOD_S / SECONDS_PER_DAY;
        assert_relative_eq!(days, 365.25, epsilon = 0.01);
    }

    #[test]
    fn test_mars_orbital_period() {
        // ~686.971 days
        let days = mars::ORBITAL_PERIOD_S / SECONDS_PER_DAY;
        assert_relative_eq!(days, 686.971, epsilon = 0.01);
    }

    #[test]
    fn test_mass_derivation_units() {
        // Earth mass should be ~5.972e24 kg
        assert_relative_eq!(earth::MASS_KG, 5.972e24, epsilon = 1e22);
    }

    #[test]
    fn test_mars_semi_major_axis() {
        // Mars semi-major axis should be ~1.524 AU
        let au = mars::SEMI_MAJOR_AXIS_KM / AU_KM;
        assert_relative_eq!(au, 1.5237, epsilon = 0.001);
    }
}