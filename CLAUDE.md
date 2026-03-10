# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
cargo build          # build
cargo run            # run with default observer (NYC: 40.71°N, 74.01°W)
cargo run -- --lat 40.71 --lon -74.01 --height 0
cargo test           # run all tests
cargo clippy         # lint
```

## Architecture

Stellui is a single Cargo package that compiles as both a **library** (`src/lib.rs`) and a **binary** (`src/main.rs`). The library exposes astronomy math; the binary owns the TUI.

### Library (`stellui::`)
- **`astro`** — coordinate math and astronomy engine wrappers. Core types: `PolarCoordinates` (rad + phi in degrees), `CartesianCoordinates`, `HzCoordinates`. Key functions: `star_stereo` (catalog star → stereographic polar), `hor_to_stereo` (horizon alt/az → polar), `astro_time_from_datetime`. `SunMoonProjection` computes sun/moon equatorial→horizon positions and moon phase/tilt in one call.
- **`catalog`** — compile-time array `J2000_CATALOG: [Star; 9096]` with fields `ra` (hours), `dec` (degrees), `mag`.

### Binary modules (declared via `mod` in `main.rs`)
- **`sky`** — bridges library to render-ready structs. `compute_stars` iterates the catalog → filters by magnitude and horizon → returns `Vec<RenderedStar>` with canvas (x, y) coordinates. `compute_sun_moon` returns `SunMoonInfo`.
- **`app`** — `App` struct holding all state (observer, datetime, stars, forecasts, input mode). `App::recompute()` re-runs both sky computations.
- **`ui`** — pure rendering via ratatui. `render()` dispatches to `render_sky` or `render_weather` based on `app.tab`. No logic here.
- **`weather`** — synchronous HTTP fetch (ureq) in a spawned thread. `fetch_forecast(lat, lon)` hits Open-Meteo and maps results to `Vec<HourlyForecast>` with a derived `SeeingQuality` score.
- **`main`** — event loop, key handling, mpsc channel for weather results. Weather is fetched on startup and on observer change.

### Coordinate pipeline
```
catalog Star (J2000 RA/Dec)
  → astronomy-engine-bindings: Equator of date → Horizon (alt/az)
  → hor_to_stereo: stereographic radius = 2·tan(45° - alt/2), phi = azimuth
  → PolarCoordinates::canvas_orient: phi -= 90°  (rotates so N=bottom, S=top, E=left, W=right)
  → CartesianCoordinates::from(polar): x = r·cos(phi), y = r·sin(phi)
  → ratatui Canvas with x_bounds/y_bounds = [-2.2, 2.2]
```
Horizon sits at `rad == 2.0`. Stars with `rad > 2.0` are below the horizon and culled.

### Threading model
No async runtime. Weather fetch runs in `std::thread::spawn`; results returned over `mpsc::channel` and polled with `try_recv` each frame (~60fps via 16ms poll timeout).
