# Pterodactyl Configuration for JavaScript Minecraft Clone Server

## Egg Import

Use the `pterodactyl-egg.json` file to import this egg into your Pterodactyl panel:

1. Go to Admin Panel → Nests → Import Egg
2. Upload the `pterodactyl-egg.json` file
3. The egg will be created with all necessary configurations

## Startup Command
```
npm start
```

## Docker Image
Uses `ghcr.io/pterodactyl/images:node-18` or `ghcr.io/pterodactyl/images:node-20`

## Environment Variables
- `PORT`: The port the server should listen on (set by Pterodactyl)
- `SERVER_NAME`: Name of your server (default: "JavaScript Minecraft Server")
- `MAX_PLAYERS`: Maximum players (default: 20)
- `WORLD_SEED`: World generation seed (optional)
- `GAME_MODE`: Game mode - survival, creative, or adventure (default: survival)

## Installation Script
The egg includes an automatic installation script that:
# Pterodactyl Setup (Legacy)

The original JavaScript/Node.js implementation shipped with a dedicated multiplayer server that could be deployed on Pterodactyl via `pterodactyl-egg.json`. The current Rust rewrite focuses on a single-player native client and does **not** include a standalone server.

## Current Status

- Rust client: single-player only
- Dedicated server: not implemented
- Pterodactyl egg: archived alongside the legacy JavaScript branch for reference

If multiplayer support is reintroduced in the Rust version, this document will be updated with new deployment instructions. Until then, the legacy egg remains available solely for historical purposes.