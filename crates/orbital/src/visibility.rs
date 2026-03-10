//! Line-of-sight visibility and occultation for the HSRN constellation.
//!
//! Determines whether two points in ECI space have an unobstructed line-of-sight,
//! accounting for occultation by Earth, Sun, and Mars.
//!
//! # Algorithm
//! A line segment between positions A and B is occluded by a spherical body C
//! with radius R if the minimum distance from C to the line segment is < R.
//!
//! Parametrize the segment as P(t) = A + t*(B - A), t ∈ [0, 1].
//! The minimum distance point is at t* = -(A-C)·(B-A) / |B-A|²
//! clamped to [0, 1]. If |P(t*) - C| < R, the segment is occluded.

use crate::coordinates::Vector3;
use hsrn_common::constants::{earth, mars, sun};

/// A spherical body that can occlude line of sight
#[derive(Debug, Clone, Copy)]
pub struct OccludingBody {
    /// Center position in ECI (km)
    pub center_km: Vector3,
    /// Radius (km)
    pub radius_km: f64,
    /// Name for diagnostics
    pub name: &'static str,
}

impl OccludingBody {
    /// Earth, centered at ECI origin
    pub fn earth() -> Self {
        Self {
            center_km: Vector3::zeros(),
            radius_km: earth::RADIUS_EQUATORIAL_KM,
            name: "Earth",
        }
    }

    /// Sun at given ECI position (km)
    pub fn sun(position_km: Vector3) -> Self {
        Self {
            center_km: position_km,
            radius_km: sun::RADIUS_KM,
            name: "Sun",
        }
    }

    /// Mars at given ECI position (km)
    pub fn mars(position_km: Vector3) -> Self {
        Self {
            center_km: position_km,
            radius_km: mars::RADIUS_EQUATORIAL_KM,
            name: "Mars",
        }
    }
}

/// Result of a line of sight check
#[derive(Debug, Clone, PartialEq)]
pub enum VisibilityResult {
    /// Unobstructed line of sight
    Visible,
    /// Line of sight blocked by the named body
    Occluded { by: &'static str },
}

impl VisibilityResult {
    pub fn is_visible(&self) -> bool {
        matches!(self, Self::Visible)
    }
}

/// Check if the segment from `a` to `b` is occluded by a single spherical body
///
/// Returns true if occluded
fn occluded_by(a: Vector3, b: Vector3, body: &OccludingBody) -> bool {
    // translate so body center is origin
    let a = a - body.center_km;
    let b = b - body.center_km;

    let ab = b - a;
    let ab_sq = ab.dot(&ab);

    // Degenerate segment case: treat as point, just check distance
    if ab_sq < 1e-10 {
        return a.norm() < body.radius_km;
    }

    // t* = closest point on segment to body center
    let t_star = (-a.dot(&ab) / ab_sq).clamp(0.0, 1.0);
    let closest = a + ab * t_star;

    closest.norm() < body.radius_km
}

/// Check line of sight between two ECI positions against all three bodies
///
/// # Arguments
/// - `a`: First position in ECI (km)
/// - `b`: Second position in ECI (km)
/// - `sun_pos_km`: Sun position in ECI (km), not at origin
/// - `mars_pos_km`: Mars position in ECI (km)
///
/// # Returns
/// `VisibilityResult::Visible` if unobstructed, or the first occluding body found
/// Earth is checked first, then Sun, then Mars
pub fn check_los(
    a: Vector3,
    b: Vector3,
    sun_pos_km: Vector3,
    mars_pos_km: Vector3,
) -> VisibilityResult {
    let bodies = [
        OccludingBody::earth(),
        OccludingBody::sun(sun_pos_km),
        OccludingBody::mars(mars_pos_km),
    ];

    for body in &bodies {
        if occluded_by(a, b, body) {
            return VisibilityResult::Occluded { by: body.name };
        }
    }

    VisibilityResult::Visible
}

/// Compute the minimum distance (km) from a body center to the line segment from a to b
///
/// Used for diagnostics to determine how close to the limb a line of sight is.
pub fn los_margin_km(a: Vector3, b: Vector3, body: &OccludingBody) -> f64 {
    let a = a - body.center_km;
    let b = b - body.center_km;

    let ab = b - a;
    let ab_sq = ab.dot(&ab);

    if ab_sq < 1e-10 {
        return a.norm();
    }

    let t_star = (-a.dot(&ab) / ab_sq).clamp(0.0, 1.0);
    let closest = a + ab * t_star;

    closest.norm() - body.radius_km
}

#[cfg(test)]
mod tests {
    use super::*;
    use hsrn_common::constants::AU_KM;

    // Earth is at ECI origin. Sun is ~1 AU along +x for these tests
    fn sun_pos() -> Vector3 {
        Vector3::new(-AU_KM, 0.0, 0.0)
    }

    fn mars_pos() -> Vector3 {
        Vector3::new(mars::SEMI_MAJOR_AXIS_KM, 0.0, 0.0)
    }

    #[test]
    fn test_clear_los_in_space() {
        let a = Vector3::new(AU_KM * 0.5, AU_KM * 0.5, 0.0);
        let b = Vector3::new(AU_KM * 0.5, AU_KM * 0.6, 0.0);
        assert!(check_los(a, b, sun_pos(), mars_pos()).is_visible());
    }

    #[test]
    fn test_earth_occultation() {
        let r = earth::RADIUS_EQUATORIAL_KM * 2.0;
        let a = Vector3::new(-r, 0.0, 0.0);
        let b = Vector3::new(r, 0.0, 0.0);
        let result = check_los(a, b, sun_pos(), mars_pos());
        assert_eq!(result, VisibilityResult::Occluded { by: "Earth" });
    }

    #[test]
    fn test_earth_tangent_is_visible() {
        let r = earth::RADIUS_EQUATORIAL_KM * 1.0;
        let a = Vector3::new(-1e6, r, 0.0);
        let b = Vector3::new(1e6, r, 0.0);
        assert!(check_los(a, b, sun_pos(), mars_pos()).is_visible());
    }

    #[test]
    fn test_sun_occultation() {
        let sun = sun_pos();
        let a = sun + Vector3::new(-sun::RADIUS_KM * 2.0, 0.0, 0.0);
        let b = sun + Vector3::new(sun::RADIUS_KM * 2.0, 0.0, 0.0);
        let result = check_los(a, b, sun, mars_pos());
        assert_eq!(result, VisibilityResult::Occluded { by: "Sun" });
    }

    #[test]
    fn test_mars_occultation() {
        let mars = mars_pos();
        let a = mars + Vector3::new(-mars::RADIUS_EQUATORIAL_KM * 2.0, 0.0, 0.0);
        let b = mars + Vector3::new(mars::RADIUS_EQUATORIAL_KM * 2.0, 0.0, 0.0);
        let result = check_los(a, b, sun_pos(), mars);
        assert_eq!(result, VisibilityResult::Occluded { by: "Mars" });
    }

    #[test]
    fn test_los_margin_positive_when_clear() {
        let a = Vector3::new(0.0, 1e6, 0.0);
        let b = Vector3::new(1e6, 1e6, 0.0);
        let margin = los_margin_km(a, b, &OccludingBody::earth());
        assert!(margin > 0.0);
    }

    #[test]
    fn test_los_margin_negative_when_occluded() {
        let r = earth::RADIUS_EQUATORIAL_KM * 2.0;
        let a = Vector3::new(-r, 0.0, 0.0);
        let b = Vector3::new(r, 0.0, 0.0);
        let margin = los_margin_km(a, b, &OccludingBody::earth());
        assert!(margin < 0.0)
    }
}
