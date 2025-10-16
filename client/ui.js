// UI Management for JavaScript Minecraft Clone

import './styles.css';

class UIManager {
  constructor() {
    this.currentScreen = 'main-menu';
    this.connectedServer = null;
    this.isInGame = false;

    this.initializeEventListeners();
    this.showScreen('main-menu');
  }

  initializeEventListeners() {
    // Main menu buttons
    document.getElementById('singleplayer-btn').addEventListener('click', () => {
      this.startSingleplayer();
    });

    document.getElementById('multiplayer-btn').addEventListener('click', () => {
      this.showScreen('multiplayer-menu');
      this.refreshServerList();
    });

    document.getElementById('settings-btn').addEventListener('click', () => {
      this.showSettings();
    });

    document.getElementById('quit-btn').addEventListener('click', () => {
      if (confirm('Are you sure you want to quit?')) {
        window.close();
      }
    });

    // Multiplayer menu buttons
    document.getElementById('direct-connect-btn').addEventListener('click', () => {
      this.showScreen('direct-connect-menu');
    });

    document.getElementById('back-to-main-btn').addEventListener('click', () => {
      this.showScreen('main-menu');
    });

    // Direct connect menu buttons
    document.getElementById('connect-btn').addEventListener('click', () => {
      const serverAddress = document.getElementById('server-ip').value;
      this.connectToServer(serverAddress);
    });

    document.getElementById('back-to-multiplayer-btn').addEventListener('click', () => {
      this.showScreen('multiplayer-menu');
    });

    // Settings menu buttons
    document.getElementById('apply-settings-btn').addEventListener('click', () => {
      this.applySettings();
    });

    document.getElementById('reset-settings-btn').addEventListener('click', () => {
      if (confirm('Reset all settings to defaults?')) {
        this.resetSettings();
      }
    });

    document.getElementById('back-from-settings-btn').addEventListener('click', () => {
      this.showScreen('main-menu');
    });

    // Settings tabs
    document.querySelectorAll('.tab-button').forEach(button => {
      button.addEventListener('click', () => {
        this.switchSettingsTab(button.dataset.tab);
      });
    });

    // Settings sliders
    this.initializeSettingsSliders();

    // Server list join buttons
    document.addEventListener('click', (e) => {
      if (e.target.classList.contains('join-button')) {
        const serverItem = e.target.closest('.server-item');
        const ip = serverItem.dataset.ip;
        const port = serverItem.dataset.port;
        this.connectToServer(`${ip}:${port}`);
      }
    });

    // Chat input
    const chatInput = document.getElementById('chat-input');
    chatInput.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') {
        this.sendChatMessage(chatInput.value);
        chatInput.value = '';
      }
    });

    // Toggle chat with T key
    document.addEventListener('keydown', (e) => {
      if (e.key === 't' || e.key === 'T') {
        e.preventDefault();
        if (this.isInGame) {
          this.toggleChat();
        }
      }

      // ESC key handling
      if (e.key === 'Escape') {
        if (this.isInGame) {
          this.showScreen('main-menu');
          this.isInGame = false;
        }
      }
    });
  }

  initializeSettingsSliders() {
    // Graphics sliders
    this.bindSlider('render-distance', 'render-distance-value', 'graphics', 'renderDistance');
    this.bindSlider('fov', 'fov-value', 'graphics', 'fov');
    this.bindSlider('max-fps', 'max-fps-value', 'performance', 'maxFPS');
    this.bindSlider('particle-limit', 'particle-limit-value', 'performance', 'particleLimit');
    this.bindSlider('mouse-sensitivity', 'mouse-sensitivity-value', 'controls', 'mouseSensitivity');
    this.bindSlider('master-volume', 'master-volume-value', 'audio', 'masterVolume');
    this.bindSlider('music-volume', 'music-volume-value', 'audio', 'musicVolume');
    this.bindSlider('sound-volume', 'sound-volume-value', 'audio', 'soundVolume');

    // Checkboxes
    this.bindCheckbox('vsync', 'graphics', 'vsync');
    this.bindCheckbox('anti-aliasing', 'graphics', 'antiAliasing');
    this.bindCheckbox('shadows', 'graphics', 'shadows');
    this.bindCheckbox('ray-tracing', 'graphics', 'rayTracing');
    this.bindCheckbox('volumetrics', 'graphics', 'volumetrics');
    this.bindCheckbox('invert-mouse', 'controls', 'invertMouse');
    this.bindCheckbox('show-coordinates', 'gameplay', 'showCoordinates');
    this.bindCheckbox('auto-save', 'gameplay', 'autoSave');

    // Selects
    this.bindSelect('quality-preset', 'graphics', 'quality');
    this.bindSelect('difficulty', 'gameplay', 'difficulty');
  }

  bindSlider(sliderId, valueId, category, key) {
    const slider = document.getElementById(sliderId);
    const valueDisplay = document.getElementById(valueId);

    slider.addEventListener('input', () => {
      valueDisplay.textContent = slider.value;
    });

    slider.addEventListener('change', () => {
      if (window.settingsManager) {
        window.settingsManager.setSetting(category, key, parseFloat(slider.value));
      }
    });
  }

  bindCheckbox(checkboxId, category, key) {
    const checkbox = document.getElementById(checkboxId);
    checkbox.addEventListener('change', () => {
      if (window.settingsManager) {
        window.settingsManager.setSetting(category, key, checkbox.checked);
      }
    });
  }

  bindSelect(selectId, category, key) {
    const select = document.getElementById(selectId);
    select.addEventListener('change', () => {
      if (window.settingsManager) {
        if (key === 'quality') {
          window.settingsManager.applyQualityPreset(select.value);
          this.updateSettingsUI();
        } else {
          window.settingsManager.setSetting(category, key, select.value);
        }
      }
    });
  }

  showSettings() {
    this.updateSettingsUI();
    this.showScreen('settings-menu');
  }

  updateSettingsUI() {
    if (!window.settingsManager) return;

    const settings = window.settingsManager.settings;

    // Update sliders
    this.updateSlider('render-distance', 'render-distance-value', settings.graphics.renderDistance);
    this.updateSlider('fov', 'fov-value', settings.graphics.fov);
    this.updateSlider('max-fps', 'max-fps-value', settings.performance.maxFPS);
    this.updateSlider('particle-limit', 'particle-limit-value', settings.performance.particleLimit);
    this.updateSlider('mouse-sensitivity', 'mouse-sensitivity-value', settings.controls.mouseSensitivity);
    this.updateSlider('master-volume', 'master-volume-value', settings.audio.masterVolume);
    this.updateSlider('music-volume', 'music-volume-value', settings.audio.musicVolume);
    this.updateSlider('sound-volume', 'sound-volume-value', settings.audio.soundVolume);

    // Update checkboxes
    this.updateCheckbox('vsync', settings.graphics.vsync);
    this.updateCheckbox('anti-aliasing', settings.graphics.antiAliasing);
    this.updateCheckbox('shadows', settings.graphics.shadows);
    this.updateCheckbox('ray-tracing', settings.graphics.rayTracing);
    this.updateCheckbox('volumetrics', settings.graphics.volumetrics);
    this.updateCheckbox('invert-mouse', settings.controls.invertMouse);
    this.updateCheckbox('show-coordinates', settings.gameplay.showCoordinates);
    this.updateCheckbox('auto-save', settings.gameplay.autoSave);

    // Update selects
    this.updateSelect('quality-preset', settings.graphics.quality);
    this.updateSelect('difficulty', settings.gameplay.difficulty);
  }

  updateSlider(sliderId, valueId, value) {
    const slider = document.getElementById(sliderId);
    const valueDisplay = document.getElementById(valueId);
    slider.value = value;
    valueDisplay.textContent = value;
  }

  updateCheckbox(checkboxId, checked) {
    document.getElementById(checkboxId).checked = checked;
  }

  updateSelect(selectId, value) {
    document.getElementById(selectId).value = value;
  }

  switchSettingsTab(tabName) {
    // Update tab buttons
    document.querySelectorAll('.tab-button').forEach(btn => {
      btn.classList.remove('active');
    });
    document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

    // Update tab content
    document.querySelectorAll('.settings-tab').forEach(tab => {
      tab.classList.remove('active');
    });
    document.getElementById(`${tabName}-tab`).classList.add('active');
  }

  applySettings() {
    if (window.settingsManager) {
      window.settingsManager.applySettings();
      alert('Settings applied successfully!');
    }
  }

  resetSettings() {
    if (window.settingsManager) {
      window.settingsManager.resetToDefaults();
      this.updateSettingsUI();
      alert('Settings reset to defaults!');
    }
  }

  updateSettings(settings) {
    // Called when settings are updated externally
    this.updateSettingsUI();
  }

  showScreen(screenId) {
    // Hide all screens
    document.querySelectorAll('.menu-screen, .game-screen, .loading-screen').forEach(screen => {
      screen.classList.remove('active');
    });

    // Show the requested screen
    const screen = document.getElementById(screenId);
    if (screen) {
      screen.classList.add('active');
    }

    this.currentScreen = screenId;
  }

  showLoading(message = 'Loading...') {
    document.querySelector('.loading-text').textContent = message;
    this.showScreen('loading-screen');
  }

  startSingleplayer() {
    this.showLoading('Starting singleplayer world...');
    setTimeout(() => {
      this.enterGame('singleplayer');
    }, 1000);
  }

  connectToServer(address) {
    this.showLoading(`Connecting to ${address}...`);
    this.connectedServer = address;

    // Emit connection event to the game client
    if (window.gameClient) {
      window.gameClient.connectToServer(address);
    } else {
      // Fallback for when game client isn't ready
      setTimeout(() => {
        this.enterGame('multiplayer', address);
      }, 1000);
    }
  }

  enterGame(mode, serverAddress = null) {
    this.isInGame = true;
    this.showScreen('game-ui');

    // Notify game client
    if (window.gameClient) {
      window.gameClient.enterGame(mode, serverAddress);
    }

    this.updateDebugInfo();
  }

  refreshServerList() {
    // For now, just show the local server
    // TODO: Implement server browser with actual server discovery
    const serverList = document.querySelector('.server-list');
    serverList.innerHTML = `
      <div class="server-item" data-ip="localhost" data-port="3000">
        <div class="server-info">
          <div class="server-name">Local Server</div>
          <div class="server-details">localhost:3000 - 0/20 players</div>
        </div>
        <button class="join-button">Join</button>
      </div>
    `;
  }

  updateDebugInfo() {
    if (!this.isInGame) return;

    const updateInfo = () => {
      // Get player position from game client
      let positionText = 'Position: 0, 10, 0';
      if (window.gameClient && window.gameClient.myPlayer) {
        const pos = window.gameClient.myPlayer.position;
        positionText = `Position: ${pos.x.toFixed(1)}, ${pos.y.toFixed(1)}, ${pos.z.toFixed(1)}`;
      }

      document.getElementById('position-display').textContent = positionText;
      document.getElementById('fps-display').textContent = `FPS: ${this.getFPS()}`;

      if (this.isInGame) {
        requestAnimationFrame(updateInfo);
      }
    };

    updateInfo();
  }

  getFPS() {
    // Simple FPS counter
    if (!this.lastFrameTime) {
      this.lastFrameTime = performance.now();
      return 60;
    }

    const now = performance.now();
    const delta = now - this.lastFrameTime;
    this.lastFrameTime = now;

    return Math.round(1000 / delta);
  }

  toggleChat() {
    const chatContainer = document.getElementById('chat-container');
    const chatInput = document.getElementById('chat-input');

    if (chatContainer.style.display === 'block') {
      chatContainer.style.display = 'none';
      chatInput.blur();
    } else {
      chatContainer.style.display = 'block';
      chatInput.focus();
    }
  }

  sendChatMessage(message) {
    if (!message.trim()) return;

    // Add message to chat
    this.addChatMessage(`You: ${message}`);

    // Send to server if connected
    if (window.gameClient && window.gameClient.socket) {
      window.gameClient.socket.emit('chatMessage', { message });
    }
  }

  addChatMessage(message) {
    const chatMessages = document.getElementById('chat-messages');
    const messageElement = document.createElement('div');
    messageElement.textContent = message;
    chatMessages.appendChild(messageElement);
    chatMessages.scrollTop = chatMessages.scrollHeight;
  }

  showConnectionError(error) {
    alert(`Connection failed: ${error}`);
    this.showScreen('multiplayer-menu');
  }

  disconnectFromServer() {
    this.connectedServer = null;
    this.isInGame = false;
    this.showScreen('main-menu');

    if (window.gameClient) {
      window.gameClient.disconnect();
    }
  }
}

// Initialize UI when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  window.uiManager = new UIManager();
});