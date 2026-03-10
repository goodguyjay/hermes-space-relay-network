//! Contact interval computation for the HSRN constellation
//!
//! A contact interval is a time window during which two nodes have
//! unobstructed line of sight. These intervals feed the Contact Graph
//! Routing (CGR) algorithm in `crates/routing`
//!
//! # Approach
//! Time is stepped at a configurable resolution. LOS is checked at each step
//! via [`check_los`]. Rising/falling edges are recorded as
//! interval boundaries
//!
//! # Resolution
//! Lagrange point satellites have contact windows lasting hours to days, so
//! coarse sampling (60–300s) is appropriate

use crate::coordinates::Vector3;
use crate::visibility::check_los;
use hsrn_common::julian_date::JulianDate;
use std::fmt::Formatter;

/// A node in the contact graph, satellite or ground station
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeId(pub &'static str);

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A time window during which two nodes have unobstructed line of sight
#[derive(Debug, Clone)]
pub struct ContactInterval {
    pub node_a: NodeId,
    pub node_b: NodeId,
    pub start: JulianDate,
    pub end: JulianDate,
}

impl ContactInterval {
    /// Duration of the contact window (seconds)
    pub fn duration_seconds(&self) -> f64 {
        self.end.elapsed_seconds_since(self.start)
    }
}

impl std::fmt::Display for ContactInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ↔ {} [{} → {}] ({:.0}s)",
            self.node_a,
            self.node_b,
            self.start,
            self.end,
            self.duration_seconds()
        )
    }
}

/// Provides ECI positions for a node at a given time
///
/// Implements this for satellites (via orbital propagator) and orbital stations
/// (ECEF -> ECI rotation)
pub trait PositionProvider {
    fn position_eci(&self, time: JulianDate) -> Vector3;
}

/// Provides ECI positions for the sun and mars (occluding bodies)
/// Separate from node positions since these are shared across all link checks
pub trait EphemerisProvider {
    fn sun_position_eci(&self, time: JulianDate) -> Vector3;
    fn mars_position_eci(&self, time: JulianDate) -> Vector3;
}

/// Configuration for contact interval computation
pub struct ContactPlanConfig {
    /// Time step for LOS sampling (seconds). Default: 60s
    pub step_seconds: f64,
}

impl Default for ContactPlanConfig {
    fn default() -> Self {
        Self { step_seconds: 60.0 }
    }
}

impl ContactPlanConfig {
    pub fn with_step(step_seconds: f64) -> Self {
        Self { step_seconds }
    }
}

/// Compute contact intervals between two nodes over a time range
///
/// # Arguments
/// - `node_a`, `node_b`: identifiers for the two nodes
/// - `pos_a`, `pos_b`: position providers for each node
/// - `ephemeris`: provides sun and mars ECI positions for occultation
/// - `start`, `end`: simulation time window
/// - `config`: sampling configuration
///
/// # Returns
/// All contact intervals where LOS is unobstructed, sorted by start time
pub fn compute_contacts<A, B, E>(
    node_a: NodeId,
    node_b: NodeId,
    pos_a: &A,
    pos_b: &B,
    ephemeris: &E,
    start: JulianDate,
    end: JulianDate,
    config: &ContactPlanConfig,
) -> Vec<ContactInterval>
where
    A: PositionProvider,
    B: PositionProvider,
    E: EphemerisProvider,
{
    let mut contacts = Vec::new();
    let mut contact_start: Option<JulianDate> = None;
    let mut current = start;

    while current <= end {
        let a = pos_a.position_eci(current);
        let b = pos_b.position_eci(current);
        let sun = ephemeris.sun_position_eci(current);
        let mars = ephemeris.mars_position_eci(current);

        let visible = check_los(a, b, sun, mars).is_visible();

        match (visible, contact_start) {
            (true, None) => {
                // rising edge: contact starts
                contact_start = Some(current);
            }

            (false, Some(start_t)) => {
                // falling edge: contact closes
                contacts.push(ContactInterval {
                    node_a: node_a.clone(),
                    node_b: node_b.clone(),
                    start: start_t,
                    end: current,
                });
                contact_start = None;
            }

            _ => {
                // no change
            }
        }

        current = current.add_seconds(config.step_seconds);
    }

    // Close any open interval at the end of the window
    if let Some(start_t) = contact_start {
        contacts.push(ContactInterval {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
            start: start_t,
            end,
        })
    }

    contacts
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use hsrn_common::constants::{AU_KM, earth, mars};

    struct StaticPosition(Vector3);

    impl PositionProvider for StaticPosition {
        fn position_eci(&self, _time: JulianDate) -> Vector3 {
            self.0
        }
    }

    struct SimpleEphemeris;

    impl EphemerisProvider for SimpleEphemeris {
        fn sun_position_eci(&self, time: JulianDate) -> Vector3 {
            Vector3::new(-AU_KM, 0.0, 0.0)
        }

        fn mars_position_eci(&self, time: JulianDate) -> Vector3 {
            Vector3::new(mars::SEMI_MAJOR_AXIS_KM, 0.0, 0.0)
        }
    }

    #[test]
    fn test_clear_contact_full_window() {
        // two nodes with permanent LOS, should produce one interval spanning the full window
        let a = StaticPosition(Vector3::new(AU_KM * 0.5, AU_KM * 0.5, 0.0));
        let b = StaticPosition(Vector3::new(AU_KM * 0.6, AU_KM * 0.5, 0.0));

        let start = JulianDate::J2000;
        let end = JulianDate::J2000.add_seconds(3600.0);
        let config = ContactPlanConfig::with_step(60.0);

        let contacts = compute_contacts(
            NodeId("HSRN-1"),
            NodeId("HSRN-2"),
            &a,
            &b,
            &SimpleEphemeris,
            start,
            end,
            &config,
        );

        assert_eq!(contacts.len(), 1);
        assert_relative_eq!(contacts[0].duration_seconds(), 3600.0, epsilon = 60.0);
    }

    #[test]
    fn test_no_contact_when_occluded() {
        // both nodes on opposite sides of earth, always occluded
        let r = earth::RADIUS_EQUATORIAL_KM * 2.0;
        let a = StaticPosition(Vector3::new(-r, 0.0, 0.0));
        let b = StaticPosition(Vector3::new(r, 0.0, 0.0));

        let start = JulianDate::J2000;
        let end = JulianDate::J2000.add_seconds(3600.0);
        let config = ContactPlanConfig::with_step(60.0);

        let contacts = compute_contacts(
            NodeId("HSRN-1"),
            NodeId("GROUND"),
            &a,
            &b,
            &SimpleEphemeris,
            start,
            end,
            &config
        );

        assert_eq!(contacts.len(), 0);
    }

    #[test]
    fn test_contact_duration_seconds() {
        let start = JulianDate::J2000;
        let end = JulianDate::J2000.add_seconds(7200.0);
        let interval = ContactInterval {
            node_a: NodeId("A"),
            node_b: NodeId("B"),
            start,
            end
        };

        assert_relative_eq!(interval.duration_seconds(), 7200.0, epsilon = 1e-3);
    }

    #[test]
    fn test_node_id_display() {
        let id = NodeId("HSRN-1");
        assert_eq!(format!("{}", id), "HSRN-1");
    }
}
