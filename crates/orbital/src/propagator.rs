//! Two-body orbital propagation using Keplerian elements.

use crate::coordinates::{EciPosition, Vector3};
use hsrn_common::constants::sun;
use hsrn_common::julian_date::JulianDate;
use hsrn_common::orbital_log;
use serde::{Deserialize, Serialize};

/// Keplerian orbital elements.
///
/// Six elements that fully describe an orbit.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OrbitalElements {
    /// Semi-major axis (km)
    pub semi_major_axis_km: f64,
    /// Eccentricity (dimensionless, 0 ≤ e < 1 for elliptical orbits)
    pub eccentricity: f64,
    /// Inclination (radians, 0 ≤ i ≤ π)
    pub inclination_rad: f64,
    /// Right ascension of ascending node (RAAN) (radians)
    pub raan_rad: f64,
    /// Argument of periapsis (radians)
    pub argument_of_periapsis_rad: f64,
    /// True anomaly at epoch (radians) true_anomaly_rad: f64,
    pub true_anomaly_rad: f64,
    /// Epoch time (reference time for the orbital elements)
    pub epoch: JulianDate,
    /// Gravitational parameter of central body (km³/s²)
    /// Default: Sun
    pub mu_km3_s2: f64,
}

impl Default for OrbitalElements {
    fn default() -> Self {
        Self {
            semi_major_axis_km: sun::RADIUS_KM * 100.0, // Arbitrary default
            eccentricity: 0.0,
            inclination_rad: 0.0,
            raan_rad: 0.0,
            argument_of_periapsis_rad: 0.0,
            true_anomaly_rad: 0.0,
            epoch: JulianDate::J2000,
            mu_km3_s2: sun::MU_KM3_PER_S2,
        }
    }
}

impl OrbitalElements {
    /// Compute orbital period (seconds).
    ///
    /// T = 2π√(a³/μ)
    pub fn period_seconds(&self) -> f64 {
        let a = self.semi_major_axis_km;
        let mu = self.mu_km3_s2;

        std::f64::consts::TAU * (a.powi(3) / mu).sqrt()
    }

    /// Convert orbital elements to ECI position and velocity at epoch.
    pub fn to_eci(&self) -> EciPosition {
        let nu = self.true_anomaly_rad;
        let e = self.eccentricity;
        let a = self.semi_major_axis_km;
        let mu = self.mu_km3_s2;

        // Position and velocity in perifocal frame (PQW)
        let p = a * (1.0 - e * e); // Semi-latus rectum
        let r_mag = p / (1.0 + e * nu.cos());

        let r_pqw = Vector3::new(r_mag * nu.cos(), r_mag * nu.sin(), 0.0);

        let v_pqw = Vector3::new(
            -(mu / p).sqrt() * nu.sin(),
            (mu / p).sqrt() * (e + nu.cos()),
            0.0,
        );

        // Rotation matrices from PQW to ECI
        let i = self.inclination_rad;
        let omega = self.raan_rad;
        let w = self.argument_of_periapsis_rad;

        // R3(-Ω) * R1(-i) * R3(-ω)
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let cos_i = i.cos();
        let sin_i = i.sin();
        let cos_w = w.cos();
        let sin_w = w.sin();

        // Combined rotation matrix (PQW -> ECI)
        let r11 = cos_omega * cos_w - sin_omega * sin_w * cos_i;
        let r12 = -cos_omega * sin_w - sin_omega * cos_w * cos_i;
        let r21 = sin_omega * cos_w + cos_omega * sin_w * cos_i;
        let r22 = -sin_omega * sin_w + cos_omega * cos_w * cos_i;
        let r31 = sin_w * sin_i;
        let r32 = cos_w * sin_i;

        let position_km = Vector3::new(
            r11 * r_pqw.x + r12 * r_pqw.y,
            r21 * r_pqw.x + r22 * r_pqw.y,
            r31 * r_pqw.x + r32 * r_pqw.y,
        );

        let velocity_km_per_s = Vector3::new(
            r11 * v_pqw.x + r12 * v_pqw.y,
            r21 * v_pqw.x + r22 * v_pqw.y,
            r31 * v_pqw.x + r32 * v_pqw.y,
        );

        EciPosition::new(position_km, velocity_km_per_s)
    }

    /// Propagate orbital elements forward in time.
    ///
    /// Uses mean anomaly propagation (assumes two-body dynamics only).
    ///
    /// # Arguments
    /// - `target_time`: Time to propagate to (UTC)
    ///
    /// # Returns
    /// New orbital elements at target_time with updated true anomaly.
    pub fn propagate_to(&self, target_time: JulianDate) -> Self {
        let dt_seconds = target_time.elapsed_seconds_since(self.epoch);

        // Mean motion: n = √(μ/a³)
        let n = (self.mu_km3_s2 / self.semi_major_axis_km.powi(3)).sqrt();

        // Mean anomaly at epoch
        let e_anom_0 = true_to_eccentric_anomaly(self.true_anomaly_rad, self.eccentricity);
        let m0 = e_anom_0 - self.eccentricity * e_anom_0.sin();

        // Propagate mean anomaly
        let mut m = m0 + n * dt_seconds;
        m %= std::f64::consts::TAU;
        if m < 0.0 {
            m += std::f64::consts::TAU
        }

        // Solve Kepler's equation for eccentric anomaly at target time
        let e_anom = solve_kepler(m, self.eccentricity);

        // Convert back to true anomaly
        let nu = eccentric_to_true_anomaly(e_anom, self.eccentricity);

        Self {
            true_anomaly_rad: nu,
            epoch: target_time,
            ..*self
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OrbitalState {
    pub position: EciPosition,
    pub time: JulianDate,
}

impl OrbitalState {
    pub fn new(position: EciPosition, time: JulianDate) -> Self {
        Self { position, time }
    }

    /// Propagate state forward using RK4 integration.
    ///
    /// # Arguments
    /// - `dt`: Time step duration (seconds)
    /// - `mu_km3_per_s2`: Gravitational parameter of central body
    pub fn propagate(&self, dt_seconds: f64, mu_km3_per_s2: f64) -> Self {
        let h = dt_seconds;

        let r0 = self.position.position_km;
        let v0 = self.position.velocity_km_per_s;

        // RK4 for coupled ODEs: dr/dt = v, dv/dt = a(r)

        // k1 = f(t, y)
        let k1_v = v0;
        let k1_a = acceleration(r0, mu_km3_per_s2);

        // k2 = f(t + h/2, y + k1*h/2)
        let r_k2 = r0 + k1_v * (h / 2.0);
        let v_k2 = v0 + k1_a * (h / 2.0);
        let k2_v = v_k2;
        let k2_a = acceleration(r_k2, mu_km3_per_s2);

        // k3 = f(t + h/2, y + k2*h/2)
        let r_k3 = r0 + k2_v * (h / 2.0);
        let v_k3 = v0 + k2_a * (h / 2.0);
        let k3_v = v_k3;
        let k3_a = acceleration(r_k3, mu_km3_per_s2);

        // k4 = f(t + h, y + k3*h)
        let r_k4 = r0 + k3_v * h;
        let v_k4 = v0 + k3_a * h;
        let k4_v = v_k4;
        let k4_a = acceleration(r_k4, mu_km3_per_s2);

        let new_position = r0 + (k1_v + k2_v * 2.0 + k3_v * 2.0 + k4_v) * (h / 6.0);
        let new_velocity = v0 + (k1_a + k2_a * 2.0 + k3_a * 2.0 + k4_a) * (h / 6.0);

        Self {
            position: EciPosition::new(new_position, new_velocity),
            time: self.time.add_seconds(dt_seconds),
        }
    }
}

/// Solve Kepler's equation for eccentric anomaly (E).
///
/// Kepler's equation: M = E - e*sin(E)
///
/// # Arguments
/// - `m`: Mean anomaly (radians)
/// - `e`: Eccentricity (0.0 <= e < 1.0 for elliptical orbits)
///
/// # Panics
/// Panics if eccentricity is >= 1.0 (parabolic/hyperbolic orbits not supported).
pub fn solve_kepler(m: f64, e: f64) -> f64 {
    // SAFETY: Newton-Raphson diverges for e >= 1.0
    assert!(
        e >= 0.0 && e < 1.0,
        "CRITICAL: eccentricity {} outside elliptical range (0.0, 1.0)",
        e
    );

    const TOLERANCE: f64 = 1e-12;
    const MAX_ITERATIONS: usize = 100;

    // Initial guess strategy
    // - High eccentricity (e > 0.8): use π to avoid bad convergence near apoapsis
    // - Normal eccentricity: use M as initial guess
    let mut e_anomaly = if e > 0.8 { std::f64::consts::PI } else { m };

    for iter in 0..MAX_ITERATIONS {
        let sin_e = e_anomaly.sin();
        let cos_e = e_anomaly.cos();

        // f(E) = E - e*sin(E) - M
        let f = e_anomaly - e * sin_e - m;

        if f.abs() < TOLERANCE {
            return e_anomaly;
        }

        // f'(E) = 1 - e*cos(E)
        let f_prime = 1.0 - e * cos_e;

        // SAFETY: f_prime approaches 0 when e->1 and E near 0 or 2π
        if f_prime.abs() < 1e-10 {
            orbital_log!(
                warn,
                e = e,
                e_anomaly = e_anomaly,
                iter = iter,
                "Kepler solver: derivative near zero, returning current estimate (potentially inaccurate)"
            );
            return e_anomaly;
        }

        e_anomaly -= f / f_prime;
    }

    orbital_log!(
        error,
        e = e,
        m = m,
        e_anomaly = e_anomaly,
        "Kepler equation did not converge after {} iterations",
        MAX_ITERATIONS
    );

    e_anomaly
}

/// Convert true anomaly to eccentric anomaly.
fn true_to_eccentric_anomaly(nu: f64, e: f64) -> f64 {
    let sin_e = ((1.0 - e * e).sqrt() * nu.sin()) / (1.0 + e * nu.cos());
    let cos_e = (e + nu.cos()) / (1.0 + e * nu.cos());
    sin_e.atan2(cos_e)
}

/// Convert eccentric anomaly to true anomaly.
fn eccentric_to_true_anomaly(e_anom: f64, e: f64) -> f64 {
    let nu = 2.0 * ((1.0 + e).sqrt() * (e_anom / 2.0).tan()).atan2((1.0 - e).sqrt());

    // Normalize to [0, 2π)
    if nu < 0.0 {
        nu + std::f64::consts::TAU
    } else {
        nu
    }
}

/// Compute two-body gravitational acceleration.
///
/// a = -μ * r / |r|³
fn acceleration(r: Vector3, mu_km3_per_s2: f64) -> Vector3 {
    let r_mag = r.magnitude();
    r * (-mu_km3_per_s2 / r_mag.powi(3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_kepler_solver_circular() {
        // Circular orbit: e = 0, so M = E = nu
        let m = 1.0;
        let e = 0.0;
        let e_anom = solve_kepler(m, e);

        assert_relative_eq!(e_anom, m, epsilon = 1e-12);
    }

    #[test]
    fn test_kepler_solver_elliptical() {
        // Moderate eccentricity
        let m = 1.5;
        let e = 0.3;
        let e_anom = solve_kepler(m, e);

        // Verify Kepler's equation is satisfied
        let residual = e_anom - e * e_anom.sin() - m;
        assert!(residual.abs() < 1e-12);
    }

    #[test]
    #[should_panic(expected = "eccentricity")]
    fn test_kepler_solver_parabolic_panics() {
        solve_kepler(1.0, 1.0);
    }

    #[test]
    fn test_orbital_period() {
        // Earth orbit around Sun
        let elements = OrbitalElements {
            semi_major_axis_km: hsrn_common::constants::AU_KM,
            mu_km3_s2: sun::MU_KM3_PER_S2,
            ..Default::default()
        };

        let period_days = elements.period_seconds() / 86400.0;

        // Should be approximately 365.25 days
        assert_relative_eq!(period_days, 365.25, epsilon = 1.0)
    }

    #[test]
    fn test_rk4_energy_conservation() {
        // Circular orbit around earth
        let a = hsrn_common::constants::earth::RADIUS_EQUATORIAL_KM + 400.0; // LEO
        let mu = hsrn_common::constants::earth::MU_KM3_PER_S2;
        let v_circular = (mu / a).sqrt();

        let initial_state = OrbitalState::new(
            EciPosition::new(
                Vector3::new(a, 0.0, 0.0),
                Vector3::new(0.0, v_circular, 0.0),
            ),
            JulianDate::J2000,
        );

        let initial_energy = orbital_energy(&initial_state, mu);

        // Propagate for one orbit (90 minute timesteps)
        let mut state = initial_state;
        let orbital_period_minutes = 90;

        for _ in 0..(orbital_period_minutes * 7) {
            // one week
            state = state.propagate(60.0, mu);
        }

        let final_energy = orbital_energy(&state, mu);

        // Energy should be conserverd within ~0.01% for RK4
        let energy_error = ((final_energy - initial_energy) / initial_energy).abs();
        assert!(
            energy_error < 1e-4,
            "Energy drift: {:.6}%",
            energy_error * 100.0
        );
    }

    /// Compute specific orbital energy: ε = v²/2 - μ/r
    fn orbital_energy(state: &OrbitalState, mu: f64) -> f64 {
        let r = state.position.position_km.norm();
        let v = state.position.velocity_km_per_s.norm();

        v * v / 2.0 - mu / r
    }
}
