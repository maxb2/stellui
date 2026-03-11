//! # stellui
//!
//! Stellui is a terminal-based planetarium that renders an all-sky stereographic
//! projection in a ratatui TUI. The library portion exposes the astronomy math
//! so that it can be tested and reused independently of the binary.
//!
//! ## Coordinate Pipeline
//!
//! ```text
//! catalog Star (J2000 RA/Dec)
//!   → astronomy-engine-bindings: Equator of date → Horizon (alt/az)
//!   → hor_to_stereo: stereographic radius = 2·tan(45° - alt/2), phi = azimuth
//!   → PolarCoordinates::canvas_orient: phi -= 90°  (N=bottom, S=top, E=left, W=right)
//!   → CartesianCoordinates::from(polar): x = r·cos(phi), y = r·sin(phi)
//!   → ratatui Canvas with x_bounds/y_bounds = [-2.2, 2.2]
//! ```
//!
//! The horizon sits at `rad == 2.0`. Stars with `rad > 2.0` are below the horizon.
//!
//! ## Modules
//!
//! - [`astro`] — coordinate math and astronomy engine wrappers
//! - [`catalog`] — compile-time J2000 star catalog (9 096 stars)

pub mod astro;
pub mod catalog;
