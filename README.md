# JavaScript Minecraft Clone

A complete recreation of Minecraft in JavaScript, featuring a desktop client built with Electron and a dedicated server compatible with Pterodactyl.

## Features

- Desktop application (not browser-based)
- Full vanilla Minecraft blocks and mechanics (targeting 1.21.10 compatibility)
- LAN and dedicated server multiplayer
- World creation and management
- Mod and plugin support
- Pterodactyl panel compatibility for dedicated servers

## Project Structure

- `client/` - Electron desktop application
- `server/` - Node.js dedicated server
- `shared/` - Common code and constants
- `mods/` - Mod and plugin directory
- `assets/` - Textures, sounds, and other assets
- `docs/` - Documentation

## Getting Started

1. Install dependencies:
   ```bash
   npm install
   cd client && npm install
   cd ../server && npm install
   cd ../shared && npm install
   ```

2. Start the server:
   ```bash
   npm run start:server
   ```

3. Start the client:
   ```bash
   npm run start:client
   ```

## Development

- Client uses Electron with Three.js for 3D rendering
- Server uses Node.js with Socket.IO for networking
- Shared code is bundled with Webpack

## Modding

Mods can be placed in the `mods/` directory. See `docs/modding.md` for API documentation.

## Server Setup

For dedicated servers, use the server component. It's compatible with Pterodactyl panel.

### Pterodactyl Configuration
- **Egg Import**: Use `pterodactyl-egg.json` to import the server egg into Pterodactyl
- **Startup Command**: `npm start` (handled by egg)
- **Docker Image**: `ghcr.io/pterodactyl/images:node-18` (handled by egg)
- **Environment Variables**: Configurable via Pterodactyl panel (name, max players, game mode, etc.)
- **Installation**: Automatic via egg installation script

See `docs/pterodactyl-setup.md` for detailed setup instructions.

## License

MIT