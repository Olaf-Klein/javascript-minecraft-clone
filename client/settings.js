// Settings Manager for JavaScript Minecraft Clone

class SettingsManager {
  constructor() {
    this.settings = this.getDefaultSettings();

    // Auto-detect recommended quality on first run
    if (!localStorage.getItem('minecraftCloneSettings')) {
      const recommendedQuality = this.getRecommendedQuality();
      console.log(`Auto-detected recommended quality: ${recommendedQuality}`);
      this.applyQualityPreset(recommendedQuality);
    }

    this.loadSettings();
  }

  getDefaultSettings() {
    return {
      graphics: {
        quality: 'medium', // low, medium, high, ultra
        renderDistance: 8,
        fov: 75,
        vsync: true,
        antiAliasing: true,
        shadows: true,
        shadowQuality: 'medium', // low, medium, high
        lighting: 'dynamic', // static, dynamic, advanced
        particles: true,
        particleQuality: 'medium', // low, medium, high
        textures: 'high', // low, medium, high
        anisotropicFiltering: true,
        mipmaps: true,
        rayTracing: false,
        volumetrics: false,
        ssao: false,
        bloom: false,
        motionBlur: false,
        depthOfField: false
      },
      performance: {
        maxFPS: 60,
        chunkLoadingThreads: 2,
        entityDistance: 64,
        particleLimit: 1000,
        textureStreaming: true,
        lodDistance: 32
      },
      controls: {
        mouseSensitivity: 1.0,
        invertMouse: false,
        keyBindings: {
          forward: 'KeyW',
          backward: 'KeyS',
          left: 'KeyA',
          right: 'KeyD',
          jump: 'Space',
          sneak: 'ShiftLeft',
          sprint: 'ControlLeft',
          inventory: 'KeyE',
          chat: 'KeyT',
          pause: 'Escape'
        }
      },
      audio: {
        masterVolume: 1.0,
        musicVolume: 0.7,
        soundVolume: 1.0,
        ambientVolume: 0.8,
        voiceVolume: 1.0
      },
      gameplay: {
        difficulty: 'normal', // peaceful, easy, normal, hard
        showCoordinates: true,
        autoSave: true,
        autoSaveInterval: 300, // seconds
        language: 'en'
      }
    };
  }

  getQualityPresets() {
    return {
      low: {
        renderDistance: 4,
        shadows: false,
        antiAliasing: false,
        particles: false,
        anisotropicFiltering: false,
        mipmaps: false,
        rayTracing: false,
        volumetrics: false,
        ssao: false,
        bloom: false,
        motionBlur: false,
        depthOfField: false,
        particleLimit: 100,
        lodDistance: 16,
        maxFPS: 30
      },
      medium: {
        renderDistance: 8,
        shadows: true,
        shadowQuality: 'low',
        antiAliasing: true,
        particles: true,
        particleQuality: 'low',
        anisotropicFiltering: true,
        mipmaps: true,
        rayTracing: false,
        volumetrics: false,
        ssao: false,
        bloom: false,
        motionBlur: false,
        depthOfField: false,
        particleLimit: 500,
        lodDistance: 24,
        maxFPS: 60
      },
      high: {
        renderDistance: 12,
        shadows: true,
        shadowQuality: 'medium',
        antiAliasing: true,
        particles: true,
        particleQuality: 'medium',
        anisotropicFiltering: true,
        mipmaps: true,
        rayTracing: false,
        volumetrics: true,
        ssao: true,
        bloom: true,
        motionBlur: false,
        depthOfField: false,
        particleLimit: 2000,
        lodDistance: 48,
        maxFPS: 60
      },
      ultra: {
        renderDistance: 16,
        shadows: true,
        shadowQuality: 'high',
        antiAliasing: true,
        particles: true,
        particleQuality: 'high',
        anisotropicFiltering: true,
        mipmaps: true,
        rayTracing: true,
        volumetrics: true,
        ssao: true,
        bloom: true,
        motionBlur: true,
        depthOfField: true,
        particleLimit: 5000,
        lodDistance: 64,
        maxFPS: 144
      }
    };
  }

  applyQualityPreset(quality) {
    const presets = this.getQualityPresets();
    if (presets[quality]) {
      Object.assign(this.settings.graphics, presets[quality]);
      this.saveSettings();
      this.applySettings();
    }
  }

  getSetting(category, key) {
    return this.settings[category]?.[key];
  }

  setSetting(category, key, value) {
    if (this.settings[category]) {
      this.settings[category][key] = value;
      this.saveSettings();
      this.applySettings();
    }
  }

  loadSettings() {
    try {
      const saved = localStorage.getItem('minecraftCloneSettings');
      if (saved) {
        const parsed = JSON.parse(saved);
        this.settings = this.deepMerge(this.getDefaultSettings(), parsed);
      }
    } catch (error) {
      console.warn('Failed to load settings:', error);
    }
  }

  saveSettings() {
    try {
      localStorage.setItem('minecraftCloneSettings', JSON.stringify(this.settings));
    } catch (error) {
      console.warn('Failed to save settings:', error);
    }
  }

  deepMerge(target, source) {
    const result = { ...target };
    for (const key in source) {
      if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
        result[key] = this.deepMerge(target[key] || {}, source[key]);
      } else {
        result[key] = source[key];
      }
    }
    return result;
  }

  applySettings() {
    // Apply graphics settings to the renderer
    if (window.gameClient && window.gameClient.voxelEngine) {
      this.applyGraphicsSettings();
    }

    // Apply performance settings
    this.applyPerformanceSettings();

    // Apply audio settings
    this.applyAudioSettings();

    // Notify other systems
    if (window.uiManager) {
      window.uiManager.updateSettings(this.settings);
    }
  }

  applyGraphicsSettings() {
    const graphics = this.settings.graphics;
    const engine = window.gameClient.voxelEngine;

    if (engine) {
      // Update renderer settings
      if (engine.renderer) {
        engine.renderer.shadowMap.enabled = graphics.shadows;
        engine.renderer.shadowMap.type = this.getShadowMapType(graphics.shadowQuality);
      }

      // Update camera settings
      engine.setFOV(graphics.fov);
      engine.setRenderDistance(graphics.renderDistance);

      // Update quality settings
      engine.updateQualitySettings({
        shadows: graphics.shadows,
        antiAliasing: graphics.antiAliasing,
        rayTracing: graphics.rayTracing,
        volumetrics: graphics.volumetrics,
        ssao: graphics.ssao,
        bloom: graphics.bloom
      });

      if (graphics.maxFPS && graphics.maxFPS > 0) {
        this.setMaxFPS(graphics.maxFPS);
      }
    }

    // Apply quality-specific optimizations
    this.optimizeForQuality(graphics.quality);
  }

  getShadowMapType(quality) {
    const THREE = require('three');
    switch (quality) {
      case 'low': return THREE.BasicShadowMap;
      case 'medium': return THREE.PCFShadowMap;
      case 'high': return THREE.PCFSoftShadowMap;
      default: return THREE.PCFShadowMap;
    }
  }

  optimizeForQuality(quality) {
    const engine = window.gameClient.voxelEngine;

    switch (quality) {
      case 'low':
        // Disable expensive effects
        this.disableAdvancedFeatures();
        break;
      case 'medium':
        // Enable basic effects
        this.enableBasicFeatures();
        break;
      case 'high':
        // Enable advanced effects
        this.enableAdvancedFeatures();
        break;
      case 'ultra':
        // Enable all effects
        this.enableUltraFeatures();
        break;
    }
  }

  disableAdvancedFeatures() {
    // Disable ray tracing, volumetrics, etc.
    console.log('Optimizing for low quality');
  }

  enableBasicFeatures() {
    // Enable basic shadows, particles, etc.
    console.log('Optimizing for medium quality');
  }

  enableAdvancedFeatures() {
    // Enable SSAO, bloom, etc.
    console.log('Optimizing for high quality');
  }

  enableUltraFeatures() {
    // Enable ray tracing, volumetrics, etc.
    console.log('Optimizing for ultra quality');
  }

  applyPerformanceSettings() {
    const performance = this.settings.performance;

    // Set chunk loading threads (simulate with timeouts)
    console.log(`Setting chunk loading threads to: ${performance.chunkLoadingThreads}`);

    // Set entity render distance
    console.log(`Setting entity distance to: ${performance.entityDistance}`);
  }

  applyAudioSettings() {
    const audio = this.settings.audio;
    // Apply volume settings to audio context
    console.log('Applying audio settings:', audio);
  }

  setMaxFPS(fps) {
    if (this.fpsInterval) {
      clearInterval(this.fpsInterval);
    }

    if (fps > 0 && fps < 300) {
      this.fpsInterval = setInterval(() => {
        // Force frame rate limiting
        if (window.gameClient && window.gameClient.voxelEngine) {
          window.gameClient.voxelEngine.render();
        }
      }, 1000 / fps);
    }
  }

  getRecommendedQuality() {
    // Simple hardware detection
    const canvas = document.createElement('canvas');
    const gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');

    if (!gl) return 'low';

    const renderer = gl.getParameter(gl.RENDERER).toLowerCase();
    const isIntegrated = renderer.includes('intel') || renderer.includes('integrated');

    // Check available memory (rough estimate)
    const maxTextureSize = gl.getParameter(gl.MAX_TEXTURE_SIZE);

    if (maxTextureSize >= 8192 && !isIntegrated) {
      return 'ultra';
    } else if (maxTextureSize >= 4096) {
      return 'high';
    } else if (maxTextureSize >= 2048) {
      return 'medium';
    } else {
      return 'low';
    }
  }

  resetToDefaults() {
    this.settings = this.getDefaultSettings();
    this.saveSettings();
    this.applySettings();
  }
}

// Create global settings manager
window.settingsManager = new SettingsManager();