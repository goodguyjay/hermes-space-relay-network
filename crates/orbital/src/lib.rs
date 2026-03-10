//! HERMES ORBITAL MECHANICS SUBSYSTEM
//!
//! Two-body orbital propagation, coordinate transforms, and line-of-sight calculations.
//!
//! # Coordinate Systems
//!
//! - **ECI (Earth-Centered Inertial)**: Origin at Earth center, fixed relative to stars.
//!   Used for orbital propagation (no fictitious forces).
//!
//! - **ECEF (Earth-Centered Earth-Fixed)**: Origin at Earth center, rotates with Earth.
//!   Used for ground station positions (lat/lon conversion).

pub mod coordinates;
pub mod propagator;
pub mod lagrange;
pub mod visibility;
mod contact;

pub use hsrn_common::constants::*;
pub use coordinates::{EcefPosition, EciPosition, Vector3};
pub use propagator::{solve_kepler, OrbitalElements, OrbitalState};
pub use lagrange::{LagrangePoint, ThreeBodySystem};