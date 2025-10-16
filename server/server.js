const express = require('express');
const { createServer } = require('http');
const { Server } = require('socket.io');
const { v4: uuidv4 } = require('uuid');
const { World } = require('../shared/world');

const app = express();
const server = createServer(app);
const io = new Server(server);

const PORT = process.env.PORT || 3000;

app.get('/', (req, res) => {
  res.send('Minecraft Clone Server');
});

const players = {};
const world = new World();

io.on('connection', (socket) => {
  console.log('A user connected:', socket.id);

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
    console.log('User disconnected:', socket.id);
    delete players[playerId];
    io.emit('playerLeft', playerId);
  });
});

server.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});