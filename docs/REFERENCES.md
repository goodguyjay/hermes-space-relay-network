# HERMES Space Relay Network — References

Numerical constants and algorithms used in this project, with sources and verification status.

---

**TODO: Gemini provided these values in the original code, but I need to verify them against authoritative sources and document the references here. This file will be a living document that evolves as we implement and test the various components of the system.**

---

## 1. Fundamental Physical Constants

### Speed of Light ($c$)
- **Value:** 299,792,458 m/s (exact)
- **Source:** NIST / CODATA 2018; fixed by SI definition since 1983
- **Status:** Verified (exact by definition)

### Newtonian Constant of Gravitation ($G$)
- **Value:** 6.67430 × 10⁻¹¹ m³ kg⁻¹ s⁻²
- **Source:** CODATA 2018 — Tiesinga et al. (2021), *Rev. Mod. Phys.* 93, 025010
- **Uncertainty:** ~22 ppm relative
- **Status:** Verified
- **Note:** G is the least precisely known fundamental constant. Never use G × M_body to derive μ — use body-specific μ values directly (determined from spacecraft tracking with sub-ppm accuracy). G is used here only for mass ratio calculations in the restricted three-body problem.

| Reference   | Value (10⁻¹¹ m³ kg⁻¹ s⁻²) | Uncertainty |
|-------------|---------------------------|-------------|
| CODATA 2014 | 6.67408 ± 0.00031         | 46 ppm      |
| CODATA 2018 | 6.67430 ± 0.00015         | 22 ppm      |
| CODATA 2022 | 6.67430 ± 0.00015         | 22 ppm      |

---

## 2. Earth Parameters

**Primary source:** Petit, G. & Luzum, B. (eds.), *IERS Conventions (2010)*, IERS Technical Note No. 36, Chapter 1, Table 1.1 — "IERS Numerical Standards"

| Parameter                | Value               | Status                         |
|--------------------------|---------------------|--------------------------------|
| μ_Earth (km³/s²)         | 398,600.4418        | Verified                       |
| Equatorial radius (km)   | 6,378.1366          | Verified                       |
| Polar radius (km)        | 6,356.751858        | Verified (derived: b = a(1-f)) |
| Inverse flattening (1/f) | 298.25642           | Verified                       |
| Rotation rate (rad/s)    | 7.2921151467 × 10⁻⁵ | Verified                       |
| Sidereal day (s)         | 86,164.0905         | Verified (derived from ω)      |

### WGS-84 vs IERS 2010
These are different standards and must not be mixed. IERS 2010 is used throughout this project for celestial mechanics. WGS-84 (equatorial radius 6,378.137 km, 1/f = 298.257223563) applies to GPS and mapping only.

**Source:** NIMA Technical Report TR8350.2 (3rd ed., Amendment 1, 2004), Table 3.1

---

## 3. Sun Parameters

**Primary source:** IAU 2015 Resolution B3 — *Recommended Nominal Conversion Constants*

| Parameter         | Value             | Status                      |
|-------------------|-------------------|-----------------------------|
| μ_Sun (km³/s²)    | 1.32712440 × 10¹¹ | Verified (IAU 2015 nominal) |
| Solar radius (km) | 695,700           | Verified (IAU 2015 nominal) |

The solar μ is a *nominal* constant fixed by IAU resolution for numerical stability. It is defined to be close to the current best estimate but will not be changed if new measurements refine the solar mass. Compatible with both TCB and TDB time scales.

---

## 4. Mars Parameters

**Primary source:** Archinal, B. A., et al. (2018). Report of the IAU/IAG Working Group on Cartographic Coordinates and Rotational Elements: 2015. *Celestial Mechanics and Dynamical Astronomy* 130(3):22. Table 1 (radii), Table 4 (rotation/μ).

**Supporting source for μ:** Konopliv, A. S., et al. (2011). Mars high resolution gravity fields from MRO, Mars seasonal gravity, and other dynamical parameters. *Icarus* 211(1):401–428.

| Parameter              | Value              | Status                         |
|------------------------|--------------------|--------------------------------|
| μ_Mars (km³/s²)        | 42,828.3752        | Verified                       |
| Equatorial radius (km) | 3,396.19           | Verified (updated from 3396.2) |
| Rotation rate (rad/s)  | 7.088218081 × 10⁻⁵ | Verified                       |
| Sol duration (s)       | 88,775.244         | Verified                       |

**Note on radius:** 3,396.2 km is a common approximation; 3,396.19 km is the formal IAU value. Mars is triaxial; for orbital mechanics a rotational ellipsoid or mean radius of 3,389.50 km is sufficient.

**Source for Sol:** Allison, M. & McEwen, M. (2000). A post-Pathfinder evaluation of areocentric solar coordinates. *Planetary and Space Science* 48(2-3):215–235. Table 4.

---

## 5. Astronomical Unit

- **Value:** 149,597,870.7 km (exact)
- **Source:** IAU 2012 Resolution B2 — *Re-definition of the astronomical unit of length*
- **Status:** Verified (exact by definition since 2012)

Since 2012 the AU is a fixed defined constant, not a measured value. Changes in solar mass estimates adjust μ_Sun; the AU remains fixed.

---

## 6. Mars Semi-Major Axis

- **Value:** 1.523679342 AU = 227,944,135.8 km
- **Source:** JPL DE430 planetary ephemeris (consistent with IAU conventions)
- **Status:** Verified (standard references cite 1.5237 AU; DE430 precision is appropriate for mission planning)

---

## 7. Lagrange Point Stability — Routh's Criterion

L4/L5 triangular points are linearly stable when:

$$27\mu(1 - \mu) < 1 \quad \Rightarrow \quad \mu < \frac{1}{2}\left(1 - \sqrt{\frac{23}{27}}\right) \approx 0.0385208965$$

- **Threshold value:** 0.0385208965 (exact analytic result)
- **Source:** Szebehely, V. (1967). *Theory of Orbits: The Restricted Problem of Three Bodies.* Academic Press. pp. 138–142.
- **Status:** Verified (0.0385 is a common engineering approximation; exact value used in code)
- **Sun-Earth μ:** ~3.003 × 10⁻⁶ ≪ threshold | Stable
- **Sun-Mars μ:** ~3.23 × 10⁻⁷ ≪ threshold | Stable

**Note:** L1, L2, L3 (collinear points) are always unstable — they are saddle points in the effective potential. Spacecraft at these points (e.g. JWST at Sun-Earth L2) require active station-keeping.

---

## 8. Time Systems

### J2000.0 Epoch
- **Formal definition:** January 1, 2000, 12:00:00 **Terrestrial Time (TT)**
- **Julian Date:** 2451545.0 TT
- **Source:** IAU 2000 Resolution B1.6
- **Status:** Verified
- **UTC equivalent:** 2000-01-01 11:58:55.816 UTC
  - TT = TAI + 32.184 s (always)
  - TAI = UTC + 32 s (leap seconds at J2000)
  - → 12:00:00 TT = 11:58:55.816 UTC
- **Implementation note:** `chrono` has no TT scale. The epoch is stored as the correct UTC-equivalent instant. Callers performing GAST or precession/nutation calculations must handle the TT timescale externally.

### Earth Sidereal Day
- **Value:** 86,164.0905 s (derived from ω = 2π/T_sid, IERS rotation rate)
- **Source:** USNO Circular 179 (2005), p. 12
- **Status:** Verified

### Mars Sol
- **Value:** 88,775.244 s (24h 39m 35.244s)
- **Source:** Allison & McEwen (2000), *Planetary and Space Science* 48(2-3), Table 4
- **Status:** Verified

---

## 9. Light-Time Delay — Earth-Mars

- **Minimum distance (near opposition):** ~55.7 million km → ~185.8 s delay
- **Maximum distance (near conjunction):** ~401 million km → ~1337.6 s delay
- **Source:** JPL Horizons System ephemeris — https://ssd.jpl.nasa.gov/horizons/
- **Status:** Verified

**Note on variability:** Opposition distance ranges from 54.6–103 million km due to eccentricity. 55.7 million km is a particularly close approach (similar to 2003 opposition). These are physical bounds on the delay, not mission-specific predictions.

**Formula:** delay (s) = distance (km) / c (km/s)

---

## 10. Orbital Mechanics Algorithms

### Kepler's Equation Solver
- **Method:** Newton-Raphson iteration
- **Convergence:** Typically 3–5 iterations for ε < 10⁻¹²
- **Initial guess:** M for e ≤ 0.8; π for e > 0.8 (avoids slow convergence near apoapsis)
- **Source:** Vallado, D. A. (2013). *Fundamentals of Astrodynamics and Applications* (4th ed.). Microcosm Press. Algorithm 2, pp. 65–66
- **Status:** Implemented and tested

### ECI ↔ ECEF Coordinate Transform
- **Method:** Rotation by Greenwich Apparent Sidereal Time (GAST)
- **Source:** Vallado (4th ed.), Algorithm 28, pp. 227–230
- **Status:** Not yet implemented

### Geodetic ↔ ECEF Transform
- **Method:** Direct transform (geodetic → ECEF); iterative inverse
- **Source:** Vallado (4th ed.), Algorithm 12, pp. 172–173
- **Status:** Implemented (`EcefPosition::from_geodetic`), tests passing

### Rotating Frame → ECI Transform
- **Method:** 2D rotation by orbital phase angle θ(t) = θ₀ + n·Δt
- **Status:** In progress

---

## 11. Coordinate System Standards

### J2000 ECI Frame
- Origin: Earth/Solar system barycenter (context-dependent)
- X-axis: Vernal equinox direction at J2000.0 (fixed stars)
- Z-axis: Earth's rotation axis at J2000.0
- **Source:** IAU 2000 Resolution B1.6

### WGS-84 (ground stations only)
- Semi-major axis: 6,378.137 km
- Inverse flattening: 298.257223563
- **Source:** NIMA TR8350.2 (3rd ed., Amendment 1, 2004), Table 3.1
- **Usage:** Ground station lat/lon/alt → ECEF conversion only. Do not mix with IERS 2010 parameters.

---

## 12. TODO

- [ ] High-fidelity Earth orientation parameters (polar motion, UT1-UTC corrections)
- [ ] J₂ oblateness coefficient for Earth nodal precession
- [ ] Solar radiation pressure model for L4/L5 station-keeping budgets
- [ ] Verify Vallado Algorithm 28 (ECI ↔ ECEF) against known test cases
- [ ] Atmospheric drag models (not applicable to heliocentric orbit, but may be needed for Earth departure/arrival phases)
- [ ] Maybe... maybe implement `hifitime` crate for rigorous TT/TAI/UTC handling if sub-minute epoch precision becomes necessary