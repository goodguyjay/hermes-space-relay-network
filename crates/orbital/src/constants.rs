//! Physical and mathematical constants for orbital mechanics.

/// Speed of light in vacuum (m/s)
pub const LIGHT_SPEED_M_PER_S: f64 = 299_792_458.0;

/// Gravitational constant (m³/kg/s²)
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Earth parameters
pub mod earth {
    /// Standard gravitational parameter μ = GM (km³/s²)
    /// Source: IERS Conventions 2010
    pub const MU_KM3_PER_S2: f64 = 398_600.4418;

    /// Equatorial radius (km)
    pub const RADIUS_EQUATORIAL_KM: f64 = 6_378.1366;

    /// Polar radius (km)
    pub const RADIUS_POLAR_KM: f64 = 6_356.751858;

    /// Flattening factor
    pub const FLATTENING: f64 = 1.0 / 298.25642;

    /// Earth rotation rate (rad/s)
    /// One sidereal day = 86164.0905 seconds
    pub const ROTATION_RATE_RAD_PER_S: f64 = 7.292_115_146_7e-5;
}

/// Mars parameters
pub mod mars {
    /// Standard gravitational parameter μ = GM (km³/s²)
    pub const MU_KM3_PER_S2: f64 = 42_828.3752;

    /// Equatorial radius (km)
    pub const RADIUS_EQUATORIAL_KM: f64 = 3_396.2;

    /// Mars rotation rate (rad/s)
    /// One sol (Martian day) = 88775.244 seconds
    pub const ROTATION_RATE_RAD_PER_S: f64 = 7.088_218_081e-5;
}

/// Sun parameters
pub mod sun {
    /// Standard gravitational parameter μ = GM (km³/s²)
    pub const MU_KM3_PER_S2: f64 = 1.327_124_40e11;

    /// Solar radius (km)
    pub const RADIUS_KM: f64 = 695_700.0;
}

/// Astronomical unit (km)
/// Mean Earth-Sun distance
pub const AU_KM: f64 = 149_597_870.7;

/// Conversion factors
pub const KM_TO_M: f64 = 1000.0;
pub const M_TO_KM: f64 = 0.001;
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;