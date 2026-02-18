This document lists the official sources and numerical values for the constants used in the project.

## 1. Fundamental Physical Constants
* **Speed of Light ($c$):** 299,792,458 m/s (Exact)
    * *Source:* NIST / CODATA 2018
    * *Ref:* Fixed value in the International System of Units (SI).
* **Gravitational Constant ($G$):** 6.67430e-11 m³ kg⁻¹ s⁻²
    * *Source:* CODATA 2018 (E. Tiesinga et al., "The CODATA 2018 periodic table of the constants")
    * *Page:* 38 (Table 1).

## 2. Earth Parameters (IERS Conventions 2010)
Used for ECI/ECEF transforms and high-fidelity orbital propagation.
* **Standard Gravitational Parameter ($\mu_e$):** 398,600.4418 km³/s²
* **Equatorial Radius ($a_e$):** 6,378.1366 km
* **Polar Radius:** 6,356.751858 km
* **Flattening Factor ($1/f$):** 298.25642
* **Nominal Mean Angular Velocity ($\omega$):** 7.2921151467e-5 rad/s
    * *Source:* Petit, G. and Luzum, B. (eds.), IERS Conventions (2010), IERS Technical Note No. 36.
    * *Section:* Chapter 1, Page 18 (Table 1.1: "IERS Numerical Standards").
    * *Note:* Flattening differs from WGS84 (298.257223563), which is optimized for GPS/mapping rather than pure celestial mechanics.

## 3. Solar System & Nominal Parameters (IAU 2015)
Standardized nominal values for planetary conversion and mass ratios.
* **Solar Gravitational Parameter ($\mu_{sun}$):** 1.32712440e11 km³/s²
* **Solar Nominal Radius ($R_{sun}$):** 695,700 km
* **Earth Nominal $\mu$:** 398,600.4 km³/s²
    * *Source:* IAU 2015 Resolution B3 on Recommended Nominal Conversion Constants.
    * *Page:* 2 (Table 1: "Nominal central mass parameters") and Page 3 (Table 2: "Nominal solar and planetary radii").

## 4. Mars Parameters
* **Gravitational Parameter ($\mu_{mars}$):** 42,828.3752 km³/s²
* **Equatorial Radius ($R_{eq}$):** 3,396.19 km
* **Rotation Rate:** 7.0882181e-5 rad/s
    * *Source:* Archinal, B. A., et al. (2018). Report of the IAU/IAG Working Group on Cartographic Coordinates and Rotational Elements: 2015.
    * *Page:* 24 (Table 1: Radii) and Page 26 (Table 4: Rotation/Mu).

## 5. Astronomical Unit (AU)
* **Value:** 149,597,870.7 km (Exact)
    * *Source:* IAU 2012 Resolution B2 on the re-definition of the astronomical unit of length.
    * *Page:* 1 (Resolution text).

## 6. Coordinate System Standards

### WGS-84 (World Geodetic System 1984)
Used for GPS, mapping, and ground station geodetic positions.
* **Semi-major axis ($a$):** 6,378.137 km
* **Flattening ($1/f$):** 298.257223563
  * *Source:* NIMA Technical Report TR8350.2 (3rd ed., Amendment 1, 2004)
  * *Page:* 3-2 (Table 3.1: "Defining Parameters")
  * *Note:* Used when converting lat/lon/alt to ECEF for ground stations.

### J2000 Epoch Reference Frame
Standard epoch for celestial mechanics calculations.
* **Epoch:** January 1, 2000, 12:00 TT (Terrestrial Time)
* **Julian Date:** 2451545.0 TT
  * *Source:* IAU 2000 Resolution B1.6
  * *Note:* ECI coordinates are typically referenced to J2000.

## 7. Time Systems

### Sidereal Day
* **Earth Mean Sidereal Day:** 86,164.0905 seconds
  * *Derived from:* $\omega = 2\pi / T_{sid}$ where $\omega$ is IERS 2010 rotation rate
  * *Source:* USNO Circular 179 (2005), Page 12

### Mars Sol
* **Mean Solar Day on Mars:** 88,775.244 seconds (24h 39m 35.244s)
  * *Source:* Allison, M. & McEwen, M. (2000). "A post-Pathfinder evaluation of areocentric solar coordinates with improved timing recipes for Mars seasonal/diurnal climate studies." Planetary and Space Science, 48(2-3), 215-235.
  * *Page:* 223 (Table 4)

## 8. Lagrange Point Stability

### Sun-Earth L4/L5 Points
* **Distance from Earth:** ~1 AU (±60° in orbit)
* **Stability:** Stable for small perturbations if mass ratio $\mu < 0.0385$
  * *Calculation:* Earth/Sun mass ratio = 3.003e-6 << 0.0385 ✓
  * *Source:* Szebehely, V. (1967). "Theory of Orbits: The Restricted Problem of Three Bodies." Academic Press.
  * *Page:* 138-142 (Chapter 5: Linear stability analysis)

### Mars-Sun System
* **Mars/Sun mass ratio:** ~3.23e-7
* **Lagrange point distances:** L1/L2 at ~1.5 million km from Mars
  * *Source:* Derived from standard three-body problem equations
  * *Ref:* Murray, C. D. & Dermott, S. F. (1999). "Solar System Dynamics." Cambridge University Press, pp. 63-65.

## 9. Light-Time Delay

### Mars-Earth Communication
* **Minimum distance (Opposition):** ~55.7 million km → 3m 6s delay
* **Maximum distance (Conjunction):** ~401 million km → 22m 16s delay
  * *Source:* JPL Horizons System ephemeris calculations
  * *Note:* Actual values vary due to orbital eccentricity
  * *Ref:* https://ssd.jpl.nasa.gov/horizons/

## 10. Orbital Mechanics Algorithms

### Two-Body Problem
* **Kepler's Equation Solver:** Newton-Raphson iteration
  * *Convergence:* Typically 3-5 iterations for $\epsilon < 10^{-12}$
  * *Source:* Vallado (4th ed), Algorithm 2, pp. 65-66
  * *Status:* Needs verification against test cases

### Coordinate Transformations
* **ECI ↔ ECEF:** Rotation by Greenwich Apparent Sidereal Time (GAST)
  * *Source:* Vallado (4th ed), Algorithm 28, pp. 227-230
  * *Status:* Needs fact-checking
* **Geodetic ↔ ECEF:** Iterative solution for inverse transform
  * *Source:* Vallado (4th ed), Algorithm 12, pp. 172-173
  * *Status:* Needs fact-checking

## Notes on Source Verification

### Verified
- Speed of light (exact by SI definition)
- IAU 2015 nominal values (primary source checked)
- Astronomical Unit (IAU 2012 resolution)

### TODO: Needs Verification
- IERS 2010 constants (need to check actual IERS TN 36 document)
- Mars parameters (check IAU/IAG 2015 report)
- Vallado algorithms (cross-reference with test cases from textbook)
- WGS-84 parameters (verify NIMA TR8350.2)

### TODO: Research
- High-fidelity Earth orientation parameters (polar motion, UT1-UTC)
- J2 perturbation coefficient for Earth oblateness
- Atmospheric drag models (if adding low Earth orbit satellites)
- Solar radiation pressure coefficients (for lagrange point station-keeping)