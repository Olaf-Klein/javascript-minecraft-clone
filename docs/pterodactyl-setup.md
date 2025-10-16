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
- Installs npm dependencies
- Builds the server bundle
- Creates logs directory

## Server API
The server exposes a REST API at the root endpoint (`/`) that returns server information in JSON format.

## Notes
- The server binds to 0.0.0.0 for proper networking
- Graceful shutdown is implemented for SIGINT and SIGTERM
- Uses Socket.IO for real-time multiplayer communication
- Default port is 3000 if not specified by Pterodactyl