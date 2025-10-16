/**
 * Client entry point
 */

import * as THREE from 'three';
import { PointerLockControls } from 'three/examples/jsm/controls/PointerLockControls.js';
import { NetworkClient } from './network/client.js';
import { WorldRenderer } from './engine/renderer.js';
import { Player } from './engine/player.js';

class Game {
  constructor() {
    this.connected = false;
    this.paused = true;
    this.chatOpen = false;
    
    // Initialize Three.js
    this.initThree();
    
    // Initialize UI
    this.initUI();
    
    // Player
    this.player = new Player();
    
    // World renderer
    this.worldRenderer = new WorldRenderer(this.scene);
    
    // Network client
    this.client = null;
    
    // Stats
    this.lastTime = performance.now();
    this.frames = 0;
    this.fps = 0;
  }

  initThree() {
    // Scene
    this.scene = new THREE.Scene();
    this.scene.background = new THREE.Color(0x87CEEB); // Sky blue
    this.scene.fog = new THREE.Fog(0x87CEEB, 0, 300);

    // Camera
    this.camera = new THREE.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    this.camera.position.set(0, 80, 0);

    // Renderer
    this.renderer = new THREE.WebGLRenderer({
      canvas: document.getElementById('game-canvas'),
      antialias: false
    });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.setPixelRatio(window.devicePixelRatio);

    // Lighting
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    this.scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(50, 100, 50);
    this.scene.add(directionalLight);

    // Controls
    this.controls = new PointerLockControls(this.camera, document.body);

    // Window resize
    window.addEventListener('resize', () => {
      this.camera.aspect = window.innerWidth / window.innerHeight;
      this.camera.updateProjectionMatrix();
      this.renderer.setSize(window.innerWidth, window.innerHeight);
    });
  }

  initUI() {
    // Connect button
    document.getElementById('connect-btn').addEventListener('click', () => {
      this.connect();
    });

    // Chat
    document.getElementById('chat-input').addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        this.sendChatMessage();
      }
    });

    // Keyboard controls
    document.addEventListener('keydown', (e) => {
      if (e.key === 't' && !this.chatOpen) {
        this.openChat();
      } else if (e.key === 'Escape') {
        if (this.chatOpen) {
          this.closeChat();
        } else {
          this.controls.unlock();
        }
      }
    });

    // Pointer lock
    this.controls.addEventListener('lock', () => {
      this.paused = false;
      document.getElementById('menu').style.display = 'none';
    });

    this.controls.addEventListener('unlock', () => {
      if (this.connected) {
        this.paused = true;
      }
    });

    // Mouse controls
    document.addEventListener('mousedown', (e) => {
      if (!this.paused && this.connected) {
        if (e.button === 0) {
          this.breakBlock();
        } else if (e.button === 2) {
          this.placeBlock();
        }
      }
    });

    document.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  async connect() {
    const username = document.getElementById('username').value;
    const serverAddress = document.getElementById('server-address').value;
    const errorMsg = document.getElementById('error-message');

    if (!username) {
      errorMsg.textContent = 'Please enter a username';
      return;
    }

    try {
      document.getElementById('loading').style.display = 'block';
      document.getElementById('menu').style.display = 'none';

      this.client = new NetworkClient(serverAddress, username);
      
      await this.client.connect();
      
      this.client.on('worldInfo', (data) => {
        console.log('Connected to world:', data);
      });

      this.client.on('chunkData', (data) => {
        this.worldRenderer.loadChunk(data);
      });

      this.client.on('blockChange', (data) => {
        this.worldRenderer.updateBlock(data.x, data.y, data.z, data.blockId);
      });

      this.client.on('chatMessage', (data) => {
        this.addChatMessage(data.message);
      });

      this.client.on('spawnPlayer', (data) => {
        console.log('Player spawned:', data);
      });

      this.client.on('playerPosition', (data) => {
        this.player.position.set(data.x, data.y, data.z);
        this.camera.position.copy(this.player.position);
      });

      this.connected = true;
      
      document.getElementById('loading').style.display = 'none';
      this.controls.lock();
      
    } catch (error) {
      console.error('Connection failed:', error);
      errorMsg.textContent = 'Connection failed: ' + error.message;
      document.getElementById('loading').style.display = 'none';
      document.getElementById('menu').style.display = 'block';
    }
  }

  openChat() {
    this.chatOpen = true;
    document.getElementById('chat').style.display = 'block';
    document.getElementById('chat-input').focus();
    this.controls.unlock();
  }

  closeChat() {
    this.chatOpen = false;
    document.getElementById('chat').style.display = 'none';
    document.getElementById('chat-input').value = '';
    if (this.connected) {
      this.controls.lock();
    }
  }

  sendChatMessage() {
    const input = document.getElementById('chat-input');
    const message = input.value.trim();
    
    if (message && this.client) {
      this.client.sendChatMessage(message);
      input.value = '';
    }
    
    this.closeChat();
  }

  addChatMessage(message) {
    const messagesDiv = document.getElementById('chat-messages');
    const messageEl = document.createElement('div');
    messageEl.className = 'chat-message';
    messageEl.textContent = message;
    messagesDiv.appendChild(messageEl);
    messagesDiv.scrollTop = messagesDiv.scrollHeight;

    // Keep only last 50 messages
    while (messagesDiv.children.length > 50) {
      messagesDiv.removeChild(messagesDiv.firstChild);
    }
  }

  breakBlock() {
    const raycaster = new THREE.Raycaster();
    raycaster.setFromCamera(new THREE.Vector2(0, 0), this.camera);
    
    const intersects = raycaster.intersectObjects(this.worldRenderer.meshes);
    
    if (intersects.length > 0) {
      const point = intersects[0].point;
      const normal = intersects[0].face.normal;
      
      const blockPos = point.clone().sub(normal.clone().multiplyScalar(0.5)).floor();
      
      if (this.client) {
        this.client.breakBlock(blockPos.x, blockPos.y, blockPos.z);
      }
    }
  }

  placeBlock() {
    const raycaster = new THREE.Raycaster();
    raycaster.setFromCamera(new THREE.Vector2(0, 0), this.camera);
    
    const intersects = raycaster.intersectObjects(this.worldRenderer.meshes);
    
    if (intersects.length > 0) {
      const point = intersects[0].point;
      const normal = intersects[0].face.normal;
      
      const blockPos = point.clone().add(normal.clone().multiplyScalar(0.5)).floor();
      
      // TODO: Get selected block from hotbar
      const blockId = 1; // Stone
      
      if (this.client) {
        this.client.placeBlock(blockPos.x, blockPos.y, blockPos.z, blockId);
      }
    }
  }

  update(deltaTime) {
    if (this.paused || !this.connected) return;

    // Update player
    this.player.update(deltaTime, this.controls);
    this.camera.position.copy(this.player.position);

    // Send position to server
    if (this.client && this.player.moved) {
      this.client.sendPosition(
        this.player.position.x,
        this.player.position.y,
        this.player.position.z,
        this.player.onGround
      );
      this.player.moved = false;
    }

    // Update debug info
    const chunkX = Math.floor(this.player.position.x / 16);
    const chunkZ = Math.floor(this.player.position.z / 16);
    
    document.getElementById('position').textContent = 
      `${this.player.position.x.toFixed(1)}, ${this.player.position.y.toFixed(1)}, ${this.player.position.z.toFixed(1)}`;
    document.getElementById('chunk').textContent = `${chunkX}, ${chunkZ}`;
    document.getElementById('blocks').textContent = this.worldRenderer.meshes.length;
  }

  animate() {
    requestAnimationFrame(() => this.animate());

    const currentTime = performance.now();
    const deltaTime = (currentTime - this.lastTime) / 1000;
    this.lastTime = currentTime;

    // FPS counter
    this.frames++;
    if (currentTime >= this.fps + 1000) {
      document.getElementById('fps').textContent = this.frames;
      this.frames = 0;
      this.fps = currentTime;
    }

    this.update(deltaTime);
    this.renderer.render(this.scene, this.camera);
  }
}

// Start game
const game = new Game();
game.animate();
