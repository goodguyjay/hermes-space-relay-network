//! Coordinate system representation and transformations.

use hsrn_common::constants::{earth, DEG_TO_RAD};
use hsrn_common::julian_date::JulianDate;
use nalgebra::Vector3 as NalVector3;
use serde::{Deserialize, Serialize};

/// 3D vector (generic, units specified by context).
pub type Vector3 = NalVector3<f64>;

/// Earth's ecliptic longitude at J2000 (radians).
/// (todo) fact check this.
const EARTH_LONGITUDE_J2000_RAD: f64 = 100.46 * DEG_TO_RAD;

/// Compute Earth's orbital phase angle at a given time.
///
/// θ(t) = θ₀ + n·Δt
/// where `n = earth::MEAN_MOTION_RAD_PER_S`, Δt = seconds since J2000 TT.
pub fn earth_orbital_phase(time: JulianDate) -> f64 {
    let dt = time.elapsed_seconds_since(JulianDate::J2000);
    let theta = EARTH_LONGITUDE_J2000_RAD + earth::MEAN_MOTION_RAD_PER_S * dt;
    // normalize
    theta.rem_euclid(std::f64::consts::TAU)
}

/// Convert a position from the synodic (rotating) frame to ECI
///
/// The synodic frame rotates with the secondary body (earth) around the primary (sun)
/// x-axis = Sun -> Earth direction
/// z-axis = ecliptic north (shared with ECI)
///
/// # Arguments
/// - `pos_rotating`: position in synodic frame (km)
/// - `time`: current simulation time (used to compute orbital phase)
pub fn rotating_to_eci(pos_rotating: Vector3, time: JulianDate) -> Vector3 {
    let theta = earth_orbital_phase(time);
    let cos_t = theta.cos();
    let sin_t = theta.sin();

    Vector3::new(
        pos_rotating.x * cos_t - pos_rotating.y * sin_t,
        pos_rotating.x * sin_t + pos_rotating.y * cos_t,
        pos_rotating.z,
    )
}

/// Inverse transform from ECI to rotating frame
pub fn eci_to_rotating(pos_eci: Vector3, time: JulianDate) -> Vector3 {
    let theta = earth_orbital_phase(time);
    let cos_t = theta.cos();
    let sin_t = theta.sin();

    Vector3::new(
        pos_eci.x * cos_t + pos_eci.y * sin_t,
        -pos_eci.x * sin_t + pos_eci.y * cos_t,
        pos_eci.z,
    )
}

/// Position in Earth-Centered Inertial (ECI) coordinates (km).
///
/// Origin: Earth center
/// X-axis: Vernal equinox direction (fixed stars)
/// Z-axis: Earth's rotation axis (North Pole)
/// Units: kilometers (km)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EciPosition {
    pub position_km: Vector3,
    pub velocity_km_per_s: Vector3,
}

impl EciPosition {
    pub fn new(position_km: Vector3, velocity_km_per_s: Vector3) -> Self {
        Self {
            position_km,
            velocity_km_per_s,
        }
    }

    /// Compute distance from origin (magnitude of position vector)
    pub fn distance_from_origin_km(&self) -> f64 {
        self.position_km.norm()
    }

    /// Compute speed (magnitude of velocity vector)
    pub fn speed_km_per_s(&self) -> f64 {
        self.velocity_km_per_s.norm()
    }
}

/// Position in Earth-Centered Earth-Fixed (ECEF) coordinates (km).
///
/// Origin: Earth center
/// Rotates with Earth (one rotation per sidereal day)
/// Units: kilometers (km)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EcefPosition {
    pub position_km: Vector3,
}

impl EcefPosition {
    pub fn new(position_km: Vector3) -> Self {
        Self { position_km }
    }

    /// Convert from geodetic coordinates (latitude, longitude, altitude).
    ///
    /// # Arguments
    /// - `lat_deg`: Latitude in degrees (north positive)
    /// - `lon_deg`: Longitude in degrees (east positive)
    /// - `alt_km`: Altitude above sea level in kilometers
    pub fn from_geodetic(lat_deg: f64, lon_deg: f64, alt_km: f64) -> Self {
        let lat_rad = lat_deg * DEG_TO_RAD;
        let lon_rad = lon_deg * DEG_TO_RAD;

        // Earth ellipsoid parameters
        let a = earth::RADIUS_EQUATORIAL_KM;
        let f = earth::FLATTENING;
        let e_sq = 2.0 * f - f * f; // Square of eccentricity

        // Radius of curvature in prime vertical
        let sin_lat = lat_rad.sin();
        let n = a / (1.0 - e_sq * sin_lat * sin_lat).sqrt();

        let cos_lat = lat_rad.cos();
        let cos_lon = lon_rad.cos();
        let sin_lon = lon_rad.sin();

        let x = (n + alt_km) * cos_lat * cos_lon;
        let y = (n + alt_km) * cos_lat * sin_lon;
        let z = (n * (1.0 - e_sq) + alt_km) * sin_lat;

        Self::new(Vector3::new(x, y, z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use hsrn_common::AU_KM;

    #[test]
    fn test_geodetic_to_ecef_equator() {
        // Point on the equator at prime meridian
        let ecef = EcefPosition::from_geodetic(0.0, 0.0, 0.0);

        // Should be at Earth's equatorial radius along X-axis
        assert_relative_eq!(
            ecef.position_km.x,
            earth::RADIUS_EQUATORIAL_KM,
            epsilon = 1e-6
        );
        assert_relative_eq!(ecef.position_km.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(ecef.position_km.z, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_geodetic_to_ecef_north_pole() {
        // North Pole
        let ecef = EcefPosition::from_geodetic(90.0, 0.0, 0.0);

        // Should be at Earth's polar radius along Z-axis
        assert_relative_eq!(ecef.position_km.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(ecef.position_km.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(
            ecef.position_km.z,
            earth::RADIUS_POLAR_KM,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_rotating_to_eci_at_j2000_theta_zero() {
        let pos = Vector3::new(AU_KM, 0.0, 0.0);
        let time = JulianDate::J2000;
        let eci = rotating_to_eci(pos, time);
        let back = eci_to_rotating(eci, time);
        assert_relative_eq!(back.x, pos.x, epsilon = 1e-6);
        assert_relative_eq!(back.y, pos.y, epsilon = 1e-6);
        assert_relative_eq!(back.z, pos.z, epsilon = 1e-6);
    }

    #[test]
    fn test_roundtrip_arbitrary_time() {
        let pos = Vector3::new(1.0e8, 5.0e7, 1.0e6);
        let time = JulianDate::J2000.add_days(365.0);
        let eci = rotating_to_eci(pos, time);
        let back = eci_to_rotating(eci, time);
        assert_relative_eq!(back.x, pos.x, epsilon = 1e-6);
        assert_relative_eq!(back.y, pos.y, epsilon = 1e-6);
        assert_relative_eq!(back.z, pos.z, epsilon = 1e-6);
    }

    #[test]
    fn test_one_full_orbit_returns_to_start() {
        let pos = Vector3::new(AU_KM, 0.0, 0.0);
        let t0 = JulianDate::J2000;
        let t1 = JulianDate::J2000.add_seconds(earth::ORBITAL_PERIOD_S);
        let eci_t0 = rotating_to_eci(pos, t0);
        let eci_t1 = rotating_to_eci(pos, t1);
        assert_relative_eq!(eci_t0.x, eci_t1.x, epsilon = 1e-3);
        assert_relative_eq!(eci_t0.y, eci_t1.y, epsilon = 1e-3);
    }
}
