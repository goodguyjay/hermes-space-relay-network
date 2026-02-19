//! Coordinate system representation and transformations.

use nalgebra::Vector3 as NalVector3;
use serde::{Deserialize, Serialize};
use hsrn_common::constants::{earth};

/// 3D vector (generic, units specified by context).
pub type Vector3 = NalVector3<f64>;

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
    ///
    /// # References
    /// Vallado (4th ed), Algorithm 12, pp. 172-173 (needs fact checking)
    pub fn from_geodetic(lat_deg: f64, lon_deg: f64, alt_km: f64) -> Self {
        use hsrn_common::constants::{DEG_TO_RAD, earth};

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
}
