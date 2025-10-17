# Rust Build & Packaging Guide

This guide explains how to build release binaries for the Rust edition of the Minecraft Clone.

## Prerequisites

- Rust toolchain 1.70 or newer (`rustup` is recommended)
- Target-specific build tools (see platform notes below)

## Release Build

```
cargo build --release
```

The optimized binary will be created in `target/release/`:

- Windows: `minecraft-clone.exe`
- macOS/Linux: `minecraft-clone`

## Running the Release Binary

```
# Windows (PowerShell)
./target/release/minecraft-clone.exe

# macOS/Linux
./target/release/minecraft-clone
```

## Cross-Compilation

The project supports cross-compiling by installing additional Rust targets:

```
# Example: build Windows binary from Windows or WSL
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc

# Example: build Linux binary
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS Specifics

- Install Xcode Command Line Tools: `xcode-select --install`
- For universal builds, compile separately for `x86_64-apple-darwin` and `aarch64-apple-darwin` and use `lipo`

### Windows Specifics

- Install the Microsoft Build Tools (included with Visual Studio Build Tools)
- Ensure the `Developer Command Prompt` environment variables are available or use the `x86_64-pc-windows-msvc` default toolchain

### Linux Specifics

- Install system dependencies required by `wgpu`, typically via the package manager (Vulkan SDK, X11/Wayland dev headers)

## Packaging Suggestions

- **Windows**: bundle the executable using tools like Inno Setup or `cargo wix`
- **macOS**: wrap the binary in an `.app` bundle via tools such as `cargo-bundle`
- **Linux**: distribute the binary directly or package with `.deb`, `.rpm`, or `AppImage` using community tooling

## Troubleshooting

- **Linker errors**: verify platform build tools are installed and `rustup toolchain list` shows the target
- **Missing GPU backend**: ensure the target platform drivers are present (Vulkan/Metal/DX12)
- **Binary fails to run on another machine**: ship the necessary runtime dependencies (e.g., MSVC redistributable on Windows)