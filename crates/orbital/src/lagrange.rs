//! Lagrange point calculations for three-body systems.

use hsrn_common::constants::{earth, mars, sun, AU_KM, ROUTH_STABILITY_THRESHOLD};
use crate::coordinates::Vector3;
use serde::{Deserialize, Serialize};
use hsrn_common::julian_date::JulianDate;

/// Lagrange point designation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LagrangePoint {
    /// L1: Between primary and secondary masses
    L1,
    /// L2: Beyond secondary mass
    L2,
    /// L3: Beyond primary mass (opposite side)
    L3,
    /// L4: Leading triangular point (60° ahead of secondary)
    L4,
    /// L5: Trailing triangular point (60° behind secondary)
    L5,
}

/// Three-body system parameters
#[derive(Debug, Clone, Copy)]
pub struct ThreeBodySystem {
    /// Mass of the primary body (kg)
    pub m1: f64,
    /// Mass of the secondary body (kg)
    pub m2: f64,
    /// Separation between bodies (in km)
    pub separation_km: f64,
}

impl ThreeBodySystem {
    /// Sun-Earth system
    pub fn sun_earth() -> Self {
        let m_sun = sun::MASS_KG / hsrn_common::constants::GRAVITATIONAL_CONSTANT * 1e9; // kg
        let m_earth = earth::MASS_KG / hsrn_common::constants::GRAVITATIONAL_CONSTANT * 1e9; // kg

        Self {
            m1: m_sun,
            m2: m_earth,
            separation_km: AU_KM,
        }
    }

    /// Sun-Mars system
    pub fn sun_mars() -> Self {
        let m_sun = sun::MASS_KG / hsrn_common::constants::GRAVITATIONAL_CONSTANT * 1e9;
        let m_mars = mars::MASS_KG / hsrn_common::constants::GRAVITATIONAL_CONSTANT * 1e9;

        Self {
            m1: m_sun,
            m2: m_mars,
            separation_km: mars::SEMI_MAJOR_AXIS_KM * AU_KM, // Mars semi-major axis
        }
    }

    /// Mass ratio μ = m2 / (m1 + m2)
    pub fn mass_ratio(&self) -> f64 {
        self.m2 / (self.m1 + self.m2)
    }

    /// Compute position of Lagrange point in rotating frame
    ///
    /// # Coordinate System
    /// Origin at system barycenter, x-axis points from m1 to m2
    ///
    /// # Returns
    /// Position vector (km) in rotating frame
    pub fn lagrange_position(&self, point: LagrangePoint) -> Vector3 {
        let mu = self.mass_ratio();
        let r = self.separation_km;

        match point {
            LagrangePoint::L1 => self.compute_l1(mu, r),
            LagrangePoint::L2 => self.compute_l2(mu, r),
            LagrangePoint::L3 => self.compute_l3(mu, r),
            LagrangePoint::L4 => self.compute_l4(mu, r),
            LagrangePoint::L5 => self.compute_l5(mu, r),
        }
    }

    /// L1: Between primary and secondary masses
    ///
    /// Approximate solution using fifth-order series expansion
    /// Accurate to ~0.01% for μ < 0.01
    fn compute_l1(&self, mu: f64, r: f64) -> Vector3 {
        // Distance from m2 to L1 normalized by separation
        let alpha = (mu / 3.0).powf(1.0 / 3.0);

        // Fifth order correction
        let gamma = alpha
            - alpha.powi(2) / 3.0
            + alpha.powi(3) / 9.0
            - 23.0 * alpha.powi(4) / 81.0
            + 31.0 * alpha.powi(5) / 81.0;

        // pos relative to m2
        let x = r * (1.0 - mu - gamma);

        Vector3::new(x, 0.0, 0.0)
    }

    /// L2: Beyond secondary mass.
    fn compute_l2(&self, mu: f64, r: f64) -> Vector3 {
        let alpha = (mu / 3.0).powf(1.0 / 3.0);

        let gamma = alpha
            + alpha.powi(2) / 3.0
            - alpha.powi(3) / 9.0
            - 31.0 * alpha.powi(4) / 81.0
            - 119.0 * alpha.powi(5) / 243.0;

        let x = r * (1.0 - mu + gamma);

        Vector3::new(x, 0.0, 0.0)
    }

    /// L3: Opposite side of primary
    fn compute_l3(&self, mu: f64, r: f64) -> Vector3 {
        // Fifth-order approximation
        let gamma = mu * (7.0 / 12.0 - 1127.0 * mu / 20736.0);

        let x = -r * (1.0 + mu / (1.0 - mu) - gamma);

        Vector3::new(x, 0.0, 0.0)
    }

    /// L4: Leading triangular point (60° ahead)
    ///
    /// Forms equilateral triangle with m1 and m2
    fn compute_l4(&self, mu: f64, r: f64) -> Vector3 {
        let x = r * (0.5 - mu);
        let y = r * (3.0_f64.sqrt() / 2.0);

        Vector3::new(x, y, 0.0)
    }

    /// L5: Trailing triangular point (60° behind)
    fn compute_l5(&self, mu: f64, r: f64) -> Vector3 {
        let x = r * (0.5 - mu);
        let y = -r * (3.0_f64.sqrt() / 2.0);

        Vector3::new(x, y, 0.0)
    }

    /// Check if L4/L5 are linearly stable.
    ///
    /// Stable if mass ratio μ < Routh's criterion
    pub fn triangular_points_stable(&self) -> bool {
        self.mass_ratio() < ROUTH_STABILITY_THRESHOLD
    }
    
    pub fn lagrange_position_eci(&self, point: LagrangePoint, time: JulianDate) -> Vector3 {
        let rotating = self.lagrange_position(point);
        crate::coordinates::rotating_to_eci(rotating, time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_sun_earth_mass_ratio() {
        let system = ThreeBodySystem::sun_earth();
        let mu = system.mass_ratio();

        // Earth-Sun mass ratio is about 3.003e-6
        assert_relative_eq!(mu, 3.003e-6, epsilon = 1e-8);
    }

    #[test]
    fn test_sun_earth_l4_l5_stable() {
        let system = ThreeBodySystem::sun_earth();

        assert!(system.triangular_points_stable());
    }

    #[test]
    fn test_l4_l5_equilateral_triangle() {
        let system = ThreeBodySystem::sun_earth();
        let l4 = system.lagrange_position(LagrangePoint::L4);
        let l5 = system.lagrange_position(LagrangePoint::L5);

        // L4 and L5 should be equidistant from both sun and earth
        let r = system.separation_km;

        // Distance from sun (at barycenter ≈ origin for sun-earth)
        let d_sun_l4 = l4.norm();
        let d_sun_l5 = l5.norm();

        assert_relative_eq!(d_sun_l4, r, epsilon = r * 0.01);
        assert_relative_eq!(d_sun_l5, r, epsilon = r * 0.01);

        // L4 should be at +60 degrees, L5 at -60 degrees
        assert!(l4.y > 0.0);
        assert!(l5.y < 0.0);
    }

    #[test]
    fn test_l1_between_bodies() {
        let system = ThreeBodySystem::sun_earth();
        let l1 = system.lagrange_position(LagrangePoint::L1);

        let earth_x = system.separation_km * (1.0 - system.mass_ratio());

        // L1 should be between sun (x≈0) and earth
        assert!(l1.x > 0.0);
        assert!(l1.x < earth_x);
    }

    #[test]
    fn test_l2_beyond_earth() {
        let system = ThreeBodySystem::sun_earth();
        let l2 = system.lagrange_position(LagrangePoint::L2);

        let earth_x = system.separation_km * (1.0 - system.mass_ratio());

        // L2 should be beyond earth
        assert!(l2.x > earth_x);
    }
}