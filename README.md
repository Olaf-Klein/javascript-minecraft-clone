# Minecraft Clone — Rust Edition

A complete rewrite of the JavaScript Minecraft Clone in Rust, featuring high-performance voxel rendering, procedural world generation, and native cross-platform support without Electron.

## Features

- High-performance 3D voxel rendering powered by `wgpu`
- Procedural terrain generation with noise functions
- Infinite world backed by chunk-based streaming
- First-person camera controls with smooth mouse look
- Creative-style player flight (WASD + Space/Shift)
- 600+ Minecraft block types (parity with 1.21.10 definitions)
- Dynamic chunk meshing and face culling for performance
- Configurable graphics and input settings persisted per user
- Native binaries for Windows, macOS, and Linux

## Why Rust?

Rust delivers predictable performance and memory safety without the overhead of a garbage collector. Combined with modern tooling (`cargo`) and the `wgpu` graphics API, it allows this project to ship as a lightweight native executable (~10 MB) while dramatically improving FPS, memory usage, and startup time compared to the original Electron build.

## Requirements

- Rust 1.70 or newer (install via [rustup.rs](https://rustup.rs/))
- GPU and drivers supporting Vulkan (Windows/Linux), Metal (macOS), or DirectX 12

## Build & Run

```bash
cargo run --release
```

For faster iteration during development:

```bash
cargo run
```

To produce an optimized binary only:

```bash
cargo build --release
# Windows: target\release\minecraft-clone.exe
# macOS/Linux: target/release/minecraft-clone
```

The repository also ships with `build.sh` (Bash) for convenience on Unix-like systems.

## Controls

| Action        | Input                |
|---------------|----------------------|
| Move          | W / A / S / D        |
| Fly up / down | Space / Shift        |
| Sprint        | Hold Shift while moving |
| Look around   | Mouse                |
| Toggle cursor | Escape               |
| Re-capture    | Left mouse button    |

## Settings

Settings are stored per platform as JSON:

- Linux: `~/.config/minecraft-clone-rust/settings.json`
- macOS: `~/Library/Application Support/minecraft-clone-rust/settings.json`
- Windows: `%APPDATA%\minecraft-clone-rust\settings.json`

Example:

```json
{
   "graphics": {
      "quality_preset": "Medium",
      "render_distance": 8,
      "vsync": true,
      "fov": 75.0,
      "shadows": true,
      "antialiasing": true
   },
   "mouse_sensitivity": 0.003,
   "player_name": "Player"
}
```

Reduce `render_distance` or disable `vsync` if you need better performance on lower-end hardware.

## Project Structure

```
src/
├── main.rs           # Application entry point & event loop
├── world/            # Block definitions, chunk management, terrain generation
├── renderer/         # wgpu renderer, camera, GPU shader
├── input/            # Keyboard and mouse input state
└── settings/         # Persistent settings handling
```

Additional docs:

- `COMPARISON.md` — JavaScript vs Rust feature overview
- `QUICKSTART.md` — 5-minute setup guide for the Rust build
- `RUST_SUMMARY.md` — High-level summary of the rewrite

## Performance Highlights

- Startup time: < 1 second (vs 2–3 seconds in Electron)
- Memory usage: ~50–100 MB (vs 200–300 MB)
- Binary size: ~10 MB (vs ~100 MB installer)
- FPS (render distance 8): 60+ on mid-range GPUs

## Roadmap

- Texture atlas + PBR materials
- Block breaking/placement mechanics
- In-game settings UI
- Inventory system
- Optional multiplayer reintroduction

## Troubleshooting

- **“cargo” not found**: install Rust from [rustup.rs](https://rustup.rs/) and ensure your shell is reloaded.
- **Build errors**: update toolchain via `rustup update`.
- **“Failed to find an adapter”**: update GPU drivers; ensure your GPU supports Vulkan/Metal/DX12.
- **Low FPS**: lower `render_distance` in the settings file or run with `cargo run --release`.

## License

MIT