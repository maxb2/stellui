# stellui

A terminal planetarium — view the night sky and weather forecast from your command line.

![stellui screenshot placeholder](https://via.placeholder.com/800x400?text=stellui)

## Features

- **Sky map** — stereographic projection of the visible sky with 9,000+ stars from the J2000 catalog
- **Sun & Moon** — real-time positions and moon phase
- **Weather forecast** — hourly seeing quality via [Open-Meteo](https://open-meteo.com/)
- **Live mode** — auto-updates at ~60fps
- **Time travel** — set any date/time to preview the sky

## Installation

```sh
cargo install --path .
```

Or run directly:

```sh
cargo run -- --lat 40.71 --lon -74.01
```

## Usage

```
stellui [--lat <degrees>] [--lon <degrees>] [--height <meters>]
```

Defaults to New York City (40.71°N, 74.01°W).

## Keybindings

| Key | Action |
|-----|--------|
| `s` | Sky tab |
| `w` | Weather tab |
| `Space` | Toggle live mode |
| `l` | Edit latitude |
| `o` | Edit longitude |
| `t` | Edit date/time (UTC, `YYYY-MM-DD HH:MM`) |
| `+` / `=` | Show fainter stars (increase magnitude limit) |
| `-` | Show fewer stars (decrease magnitude limit) |
| `r` | Refresh weather |
| `↑` / `↓` | Scroll weather forecast |
| `Enter` | Confirm input |
| `Esc` | Cancel input |
| `q` / `Ctrl+C` | Quit |

## Sky Map Orientation

The sky map uses a stereographic projection centered on the zenith:
- **South at top**, North at bottom
- **East at left**, West at right
- Stars beyond the horizon (altitude < 0°) are hidden

## Dependencies

- [astronomy-engine-bindings](https://crates.io/crates/astronomy-engine-bindings) — planetary positions
- [ratatui](https://ratatui.rs/) — TUI framework
- [Open-Meteo API](https://open-meteo.com/) — weather data (no API key required)

## License

MIT
