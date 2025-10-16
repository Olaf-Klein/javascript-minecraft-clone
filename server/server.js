const express = require('express');
const { createServer } = require('http');
const { Server } = require('socket.io');
const { v4: uuidv4 } = require('uuid');
const { World } = require('../shared/world');
const fs = require('fs');
const path = require('path');

// Create logs directory if it doesn't exist
const logsDir = path.join(__dirname, 'logs');
if (!fs.existsSync(logsDir)) {
  fs.mkdirSync(logsDir);
}

// Simple logging function
function log(message) {
  const timestamp = new Date().toISOString();
  const logMessage = `[${timestamp}] ${message}\n`;
  console.log(message);
  fs.appendFileSync(path.join(logsDir, 'latest.log'), logMessage);
}

const app = express();
const server = createServer(app);
const io = new Server(server);

const PORT = process.env.PORT || 3000;
const SERVER_NAME = process.env.SERVER_NAME || 'JavaScript Minecraft Server';
const MAX_PLAYERS = parseInt(process.env.MAX_PLAYERS) || 20;
const WORLD_SEED = process.env.WORLD_SEED || null;
const GAME_MODE = process.env.GAME_MODE || 'survival';

log(`Starting ${SERVER_NAME}`);
log(`Max Players: ${MAX_PLAYERS}`);
log(`Game Mode: ${GAME_MODE}`);
if (WORLD_SEED) log(`World Seed: ${WORLD_SEED}`);

const players = {};
const world = new World();

app.get('/', (req, res) => {
  res.json({
    name: SERVER_NAME,
    version: '1.0.0',
    players: Object.keys(players).length,
    maxPlayers: MAX_PLAYERS,
    gameMode: GAME_MODE
  });
});

io.on('connection', (socket) => {
  // Check player limit
  if (Object.keys(players).length >= MAX_PLAYERS) {
    socket.emit('connectionRejected', { reason: 'Server is full' });
    socket.disconnect();
    return;
  }

  log('A user connected:', socket.id);

  // Assign player ID
  const playerId = uuidv4();
  players[playerId] = { id: playerId, position: { x: 0, y: 10, z: 0 } };
  socket.playerId = playerId;

  // Send initial world data (for now, send a small area)
  const worldData = {};
  for (let x = -1; x <= 1; x++) {
    for (let z = -1; z <= 1; z++) {
      const chunk = world.getChunk(x, z);
      worldData[`${x},${z}`] = chunk.blocks;
    }
  }
  socket.emit('worldData', worldData);
  socket.emit('playerJoined', players[playerId]);

  // Broadcast to other players
  socket.broadcast.emit('playerJoined', players[playerId]);

  socket.on('playerMove', (data) => {
    if (players[playerId]) {
      players[playerId].position = data.position;
      socket.broadcast.emit('playerMoved', { id: playerId, position: data.position });
    }
  });

  socket.on('blockUpdate', (data) => {
    // Update world
    world.setBlock(data.x, data.y, data.z, data.type);
    io.emit('blockUpdated', data);
  });

  socket.on('disconnect', () => {
    log('User disconnected:', socket.id);
    delete players[playerId];
    io.emit('playerLeft', playerId);
  });
});

server.listen(PORT, '0.0.0.0', () => {
  log(`Server running on port ${PORT}`);
});

// Graceful shutdown
process.on('SIGINT', () => {
  log('Shutting down server...');
  io.close(() => {
    log('Socket.IO closed');
    server.close(() => {
      log('Server closed');
      process.exit(0);
    });
  });
});

process.on('SIGTERM', () => {
  log('Shutting down server...');
  io.close(() => {
    log('Socket.IO closed');
    server.close(() => {
      log('Server closed');
      process.exit(0);
    });
  });
});