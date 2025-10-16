# JavaScript Minecraft Clone - Building Executables

This guide explains how to build executable files for the JavaScript Minecraft Clone desktop application.

## Prerequisites

- Node.js (v16 or higher)
- npm or yarn

## Installation

1. Install dependencies for all workspaces:
```bash
npm run install:all
```

Or install manually:
```bash
# Root dependencies
npm install

# Client dependencies
cd client
npm install

# Server dependencies
cd ../server
npm install

# Shared dependencies
cd ../shared
npm install
```

## Building Executables

### Windows (.exe installer)

```bash
cd client
npm run dist:win
```

This creates a Windows NSIS installer in `client/dist/`.

### macOS (.dmg)

```bash
cd client
npm run dist:mac
```

This creates a macOS DMG installer in `client/dist/`.

### Linux (AppImage)

```bash
cd client
npm run dist:linux
```

This creates a Linux AppImage in `client/dist/`.

### All Platforms

```bash
cd client
npm run dist
```

This builds for all configured platforms.

## Icon Generation

Icons are automatically generated from the SVG file during the build process:

```bash
cd client
npm run generate-icons
```

This uses the `sharp` library to convert `assets/icon.svg` to the required formats.

## Running the Application

### Development Mode

```bash
# Start client
npm run start:client

# Start server
npm run start:server
```

### Production Build

After building the executable, run the installer or executable file directly.

## Distribution

The built executables are located in `client/dist/` and can be distributed to users for easy installation and execution on their devices.

## Troubleshooting

- If icon generation fails, ensure `sharp` is installed: `npm install --save-dev sharp`
- If builds fail, ensure all dependencies are installed in all workspaces
- For Windows builds, ensure you have the Windows Build Tools installed if needed