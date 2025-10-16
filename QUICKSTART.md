# Quick Start Guide

Get up and running with JavaScript Minecraft Clone in minutes!

## Prerequisites

- Node.js 18 or higher
- npm 9 or higher
- Modern web browser (Chrome, Firefox, Safari, Edge)

## Installation

1. **Clone the repository**:
```bash
git clone https://github.com/Olaf-Klein/javascript-minecraft-clone.git
cd javascript-minecraft-clone
```

2. **Install dependencies**:
```bash
npm install
```

This will install:
- Server dependencies (Express, WebSocket, SQLite)
- Client dependencies (Three.js, Vite)
- Development tools

## Running the Game

### Option 1: Development Mode (Recommended for testing)

**Terminal 1 - Start the server**:
```bash
npm start
```

You should see:
```
=================================
JavaScript Minecraft Server
=================================
Starting server...
World: world
Seed: 12345
Server starting on port 3000
...
Server is ready!
Listening on port 3000
Press Ctrl+C to stop
```

**Terminal 2 - Start the client**:
```bash
npm run client:dev
```

The browser will open automatically at `http://localhost:8080`

### Option 2: Production Build

1. **Build the client**:
```bash
npm run client:build
```

2. **Start the server**:
```bash
npm start
```

3. **Serve the built client** (use any static file server):
```bash
npx serve dist/client -p 8080
```

## Playing the Game

1. **Open your browser** to `http://localhost:8080`

2. **Enter your username** (default: "Player")

3. **Set server address** (default: "ws://localhost:3000")

4. **Click "Connect"**

5. **Wait for world to load** - this may take a few seconds

6. **Click to lock your mouse** and start playing!

### Controls

- **W/A/S/D**: Move forward/left/backward/right
- **Space**: Jump
- **Shift**: Sprint (hold while moving)
- **Mouse**: Look around
- **Left Click**: Break block
- **Right Click**: Place block
- **T**: Open chat
- **Escape**: Release mouse/close chat

## What You Can Do

### Explore the World

- The world generates procedurally with terrain, caves, and resources
- Different biomes (plains, desert, forest)
- Natural features like trees

### Multiplayer

- Other players can connect to the same server
- See other players in real-time
- Chat with other players (press T)

### Creative Mode

- Fly mode (not yet implemented)
- Break blocks instantly
- Unlimited resources
- No health or hunger

## Configuration

Edit `server/config.json` to customize:

```json
{
  "port": 3000,              // Server port
  "maxPlayers": 20,          // Maximum players
  "renderDistance": 8,       // Chunk render distance
  "worldName": "world",      // World save name
  "seed": 12345,             // World generation seed
  "pluginsEnabled": true     // Enable plugin system
}
```

## Plugins

The server includes an example plugin. Check `server/plugins/plugins-data/example-plugin/`.

To create your own plugins, see [docs/MODDING.md](docs/MODDING.md)

## Troubleshooting

### Server won't start

**Error: Port 3000 already in use**
- Another process is using port 3000
- Change the port in `server/config.json`
- Or stop the other process: `lsof -ti:3000 | xargs kill`

**Error: Cannot find module**
- Run `npm install` again
- Delete `node_modules` and run `npm install`

### Client won't connect

**Connection refused**
- Make sure the server is running
- Check the server address in the connect menu
- Try `ws://localhost:3000` instead of `ws://127.0.0.1:3000`

**WebSocket connection failed**
- Check browser console for errors (F12)
- Verify firewall isn't blocking the connection
- Make sure you're using `ws://` not `http://`

### World won't load

**Stuck on "Loading world..."**
- Check server logs for errors
- World generation might be slow on first run
- Try refreshing the page

**Blocks not rendering**
- Check browser console for Three.js errors
- Try a different browser
- Update your graphics drivers

### Performance issues

**Low FPS**
- Reduce render distance in `server/config.json`
- Close other browser tabs
- Try a different browser
- Update graphics drivers

**High memory usage**
- Reduce render distance
- Limit max players
- Restart the server periodically

## Next Steps

- Read the [Architecture Documentation](docs/ARCHITECTURE.md)
- Learn about [Deployment](docs/DEPLOYMENT.md)
- Create [Plugins/Mods](docs/MODDING.md)
- Contribute to the project ([CONTRIBUTING.md](CONTRIBUTING.md))

## Getting Help

- Check the documentation in the `docs/` directory
- Open an issue on GitHub
- Read the source code (it's well-commented!)

## Fun Things to Try

1. **Dig down** and find ores (coal, iron, gold, diamond)
2. **Build structures** with different block types
3. **Explore biomes** - walk far to find different terrain
4. **Invite friends** - give them your server IP to play together
5. **Create a plugin** - make the game do something new!

Happy crafting! 🎮⛏️
