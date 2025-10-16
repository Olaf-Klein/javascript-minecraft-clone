// Client-side code for the Minecraft clone
const { io } = require('socket.io-client');
const VoxelEngine = require('./voxel');
const { World } = require('../shared/world');

console.log('Minecraft Clone Client Starting...');

const voxelEngine = new VoxelEngine();
voxelEngine.animate();

const world = new World();

const socket = io('http://localhost:3000'); // Connect to server

let myPlayer = null;
const otherPlayers = {};

socket.on('connect', () => {
  console.log('Connected to server');
});

socket.on('connectionRejected', (data) => {
  console.error('Connection rejected:', data.reason);
  alert(`Connection rejected: ${data.reason}`);
});

socket.on('worldData', (data) => {
  console.log('Received world data');
  // Load chunks into local world
  for (const [key, blocks] of Object.entries(data)) {
    const [x, z] = key.split(',').map(Number);
    const chunk = world.getChunk(x, z);
    chunk.blocks = blocks;
  }
  // TODO: Render the world
});

socket.on('playerJoined', (player) => {
  if (player.id === myPlayer?.id) {
    // This is us, already handled
  } else {
    otherPlayers[player.id] = player;
    console.log('Player joined:', player);
  }
});

socket.on('playerMoved', (data) => {
  if (otherPlayers[data.id]) {
    otherPlayers[data.id].position = data.position;
    // TODO: Update player position in scene
  }
});

socket.on('playerLeft', (playerId) => {
  delete otherPlayers[playerId];
  console.log('Player left:', playerId);
});

socket.on('blockUpdated', (data) => {
  world.setBlock(data.x, data.y, data.z, data.type);
  console.log('Block updated:', data);
  // TODO: Update rendering
});

socket.on('disconnect', () => {
  console.log('Disconnected from server');
});

// Example: Send player movement (placeholder)
setInterval(() => {
  if (myPlayer) {
    // Simulate movement
    myPlayer.position.x += 0.1;
    socket.emit('playerMove', { position: myPlayer.position });
  }
}, 1000);