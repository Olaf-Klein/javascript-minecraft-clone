# Quick Start Guide — Rust Edition

Welcome to the Minecraft Clone Rust Edition! This guide will get you playing in under 5 minutes.

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **GPU**: Modern GPU with Vulkan/Metal/DirectX 12 support

## Installation

### Option 1: From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Olaf-Klein/javascript-minecraft-clone.git
cd javascript-minecraft-clone

# Run the game (this will compile and start)
cargo run --release
```

The first build will take a few minutes. Subsequent runs are much faster!

### Option 2: Using the Build Script

```bash
./build.sh
./target/release/minecraft-clone
```

## First Launch

1. The game will start in fullscreen windowed mode
2. Your mouse will be automatically captured
3. You'll spawn at coordinates (8, 80, 8) in a procedurally generated world

## Basic Controls

| Action | Key/Button |
|--------|-----------|
| Move Forward | W |
| Move Backward | S |
| Strafe Left | A |
| Strafe Right | D |
| Fly Up | Space |
| Fly Down | Shift |
| Look Around | Move Mouse |
| Release Mouse | Escape |
| Exit Game | Escape (when mouse is released) |
| Re-capture Mouse | Left Click |

## Tips for New Players

1. **Movement**: You start in creative/flying mode. Use WASD to move and Space/Shift to fly up/down.

2. **Camera**: Move your mouse to look around. The view is smooth and responsive.

3. **Exploring**: The world generates infinitely around you. Fly around to see different biomes:
   - Mountains (high terrain)
   - Plains (medium height)
   - Beaches (near water level)

4. **Performance**: If you experience low FPS:
   - Edit `~/.config/minecraft-clone-rust/settings.json`
   - Reduce `render_distance` from 8 to 4 or 6
   - Disable `vsync` if needed

5. **Quitting**: Press Escape once to release the mouse, then Escape again to exit.

## Adjusting Settings

Settings are stored in JSON format at:
- **Linux**: `~/.config/minecraft-clone-rust/settings.json`
- **macOS**: `~/Library/Application Support/minecraft-clone-rust/settings.json`
- **Windows**: `%APPDATA%\minecraft-clone-rust\settings.json`

Example settings file:
```json
{
  "graphics": {
    "quality_preset": "High",
    "render_distance": 12,
    "vsync": true,
    "fov": 80.0,
    "shadows": true,
    "antialiasing": true
  },
  "mouse_sensitivity": 0.003,
  "player_name": "Steve"
}
```

Edit these values to customize your experience!

## Troubleshooting

### "Failed to find an adapter"
- **Cause**: Your GPU doesn't support the required graphics APIs
- **Solution**: Update GPU drivers, or check if your GPU supports Vulkan/Metal/DX12

### Low FPS
- **Cause**: High render distance or slow GPU
- **Solution**: Reduce render_distance in settings to 4-6 chunks

### Mouse doesn't capture
- **Cause**: Window focus issues
- **Solution**: Click in the window once it opens

### Compilation errors
- **Cause**: Outdated Rust version
- **Solution**: Run `rustup update` to get the latest Rust

### "Permission denied" on Linux
- **Cause**: Build script not executable
- **Solution**: Run `chmod +x build.sh`

## What's Different from JavaScript Version?

✅ **Better Performance**: 2-3x faster rendering, lower memory usage

✅ **Smaller Size**: ~10MB binary vs ~100MB+ Electron app

✅ **Native**: No Electron overhead, true native application

❌ **No Multiplayer**: Server components not included (as requested)

❌ **No Textures Yet**: Uses procedural colors instead of texture atlas

❌ **No UI**: Minimal interface (no menus yet)

## Next Steps

1. Explore the world and enjoy the improved performance!
2. Try adjusting graphics settings to see the impact
3. Check out [COMPARISON.md](COMPARISON.md) for detailed differences
4. Read [README.md](README.md) for full documentation

## Getting Help

If you encounter issues:
1. Check the troubleshooting section above
2. Make sure you're on the `rust-rewrite` branch
3. Verify Rust is up to date: `rustc --version` (should be 1.70+)
4. Check GPU drivers are current

## Have Fun!

Enjoy exploring your procedurally generated Minecraft world in high-performance Rust! 🎮🦀
