#!/bin/bash
# Build script for Rust Minecraft Clone

echo "Building Minecraft Clone - Rust Edition..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

echo "Rust version:"
rustc --version
cargo --version
echo ""

# Build the project
echo "Building in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "Binary location: target/release/minecraft-clone"
    echo ""
    echo "To run the game:"
    echo "  cargo run --release"
    echo ""
    echo "Or directly:"
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        echo "  .\\target\\release\\minecraft-clone.exe"
    else
        echo "  ./target/release/minecraft-clone"
    fi
else
    echo ""
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi
