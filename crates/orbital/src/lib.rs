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

mod constants;
mod coordinates;
pub mod propagator;

pub use constants::*;
pub use coordinates::{Vector3, EciPosition, EcefPosition};
pub use propagator::{OrbitalElements, OrbitalState, solve_kepler};