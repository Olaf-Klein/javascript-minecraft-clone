// Enhanced voxel engine using Three.js with advanced graphics features
const THREE = require('three');
const { BLOCK_TYPES, CHUNK_SIZE } = require('../shared/constants');

class VoxelEngine {
  constructor() {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
    this.renderer.toneMappingExposure = 1.0;
    document.body.appendChild(this.renderer.domElement);

    this.camera.position.set(0, 10, 10);
    this.camera.lookAt(0, 0, 0);

    // Enhanced lighting setup
    this.setupLighting();

    // Quality settings
    this.qualitySettings = {
      shadows: true,
      antiAliasing: true,
      rayTracing: false,
      volumetrics: false,
      ssao: false,
      bloom: false
    };

    // Create initial chunk
    this.createChunk(0, 0, 0);

    // Start render loop
    this.animate();
  }

  setupLighting() {
    // Ambient light
    this.ambientLight = new THREE.AmbientLight(0x404040, 0.4);
    this.scene.add(this.ambientLight);

    // Directional light (sun) with enhanced shadows
    this.directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    this.directionalLight.position.set(50, 50, 25);
    this.directionalLight.castShadow = true;

    // Configure high-quality shadow properties
    this.directionalLight.shadow.mapSize.width = 2048;
    this.directionalLight.shadow.mapSize.height = 2048;
    this.directionalLight.shadow.camera.near = 0.5;
    this.directionalLight.shadow.camera.far = 500;
    this.directionalLight.shadow.camera.left = -100;
    this.directionalLight.shadow.camera.right = 100;
    this.directionalLight.shadow.camera.top = 100;
    this.directionalLight.shadow.camera.bottom = -100;

    this.scene.add(this.directionalLight);

    // Point lights array for dynamic lighting
    this.pointLights = [];
  }

  createChunk(x, y, z) {
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshLambertMaterial({
      color: 0x00ff00,
      transparent: false
    });

    const cube = new THREE.Mesh(geometry, material);
    cube.position.set(x * CHUNK_SIZE, y * CHUNK_SIZE, z * CHUNK_SIZE);
    cube.castShadow = true;
    cube.receiveShadow = true;
    this.scene.add(cube);

    return cube;
  }

  updateQualitySettings(settings) {
    this.qualitySettings = { ...this.qualitySettings, ...settings };

    // Update renderer settings
    this.renderer.shadowMap.enabled = this.qualitySettings.shadows;
    this.renderer.setPixelRatio(this.qualitySettings.antiAliasing ? window.devicePixelRatio : 1);

    // Update materials and lighting based on quality
    this.scene.traverse((object) => {
      if (object.material) {
        if (this.qualitySettings.shadows) {
          object.castShadow = true;
          object.receiveShadow = true;
        } else {
          object.castShadow = false;
          object.receiveShadow = false;
        }
      }
    });

    // Handle advanced features
    if (this.qualitySettings.rayTracing) {
      this.enableRayTracing();
    } else {
      this.disableRayTracing();
    }

    if (this.qualitySettings.volumetrics) {
      this.enableVolumetrics();
    } else {
      this.disableVolumetrics();
    }

    console.log('Updated graphics quality:', this.qualitySettings);
  }

  enableRayTracing() {
    // Placeholder for ray tracing implementation
    // In a real implementation, this would use WebGL extensions or WebGPU
    console.log('Ray tracing enabled (simulated with enhanced lighting)');
    this.renderer.setClearColor(0x87CEEB, 1); // Sky blue tint for ray tracing effect

    // Add some reflective materials
    this.scene.traverse((object) => {
      if (object.material && object.material.type === 'MeshLambertMaterial') {
        object.material.reflectivity = 0.3;
        object.material.refractionRatio = 0.98;
      }
    });
  }

  disableRayTracing() {
    console.log('Ray tracing disabled');
    this.renderer.setClearColor(0x000000, 1);

    // Remove reflective properties
    this.scene.traverse((object) => {
      if (object.material && object.material.type === 'MeshLambertMaterial') {
        object.material.reflectivity = 0;
        object.material.refractionRatio = 1;
      }
    });
  }

  enableVolumetrics() {
    console.log('Volumetrics enabled (simulated with atmospheric scattering)');

    // Add volumetric light scattering effect
    if (!this.volumetricLight) {
      const geometry = new THREE.SphereGeometry(50, 32, 32);
      const material = new THREE.MeshBasicMaterial({
        color: 0xffffff,
        transparent: true,
        opacity: 0.05,
        side: THREE.BackSide
      });
      this.volumetricLight = new THREE.Mesh(geometry, material);
      this.scene.add(this.volumetricLight);
    }

    // Add god rays effect
    if (!this.godRays) {
      // Simple god rays simulation with particles
      this.createGodRays();
    }
  }

  disableVolumetrics() {
    console.log('Volumetrics disabled');
    if (this.volumetricLight) {
      this.scene.remove(this.volumetricLight);
      this.volumetricLight = null;
    }
    if (this.godRays) {
      this.scene.remove(this.godRays);
      this.godRays = null;
    }
  }

  createGodRays() {
    // Create simple god rays effect with transparent planes
    const raysGeometry = new THREE.PlaneGeometry(100, 100);
    const raysMaterial = new THREE.MeshBasicMaterial({
      color: 0xffffff,
      transparent: true,
      opacity: 0.1,
      side: THREE.DoubleSide
    });

    this.godRays = new THREE.Group();

    // Create multiple ray planes at different angles
    for (let i = 0; i < 8; i++) {
      const ray = new THREE.Mesh(raysGeometry, raysMaterial.clone());
      ray.rotation.z = (Math.PI / 4) * i;
      ray.position.y = 25;
      this.godRays.add(ray);
    }

    this.scene.add(this.godRays);
  }

  updateLighting(intensity, color) {
    if (this.directionalLight) {
      this.directionalLight.intensity = intensity;
      this.directionalLight.color.setHex(color);
    }
  }

  addPointLight(position, color, intensity) {
    const light = new THREE.PointLight(color, intensity, 100);
    light.position.copy(position);
    light.castShadow = this.qualitySettings.shadows;

    if (this.qualitySettings.shadows) {
      light.shadow.mapSize.width = 1024;
      light.shadow.mapSize.height = 1024;
    }

    this.scene.add(light);
    this.pointLights.push(light);
    return light;
  }

  removePointLight(light) {
    this.scene.remove(light);
    const index = this.pointLights.indexOf(light);
    if (index > -1) {
      this.pointLights.splice(index, 1);
    }
  }

  setRenderDistance(distance) {
    // Update camera far plane
    this.camera.far = distance * CHUNK_SIZE;
    this.camera.updateProjectionMatrix();
    console.log(`Render distance set to: ${distance} chunks`);
  }

  setFOV(fov) {
    this.camera.fov = fov;
    this.camera.updateProjectionMatrix();
  }

  setMaxFPS(fps) {
    // This would be handled by the settings manager
    console.log(`Max FPS set to: ${fps}`);
  }

  render() {
    // Update animated elements
    if (this.volumetricLight) {
      this.volumetricLight.rotation.y += 0.001;
    }

    if (this.godRays) {
      this.godRays.rotation.y += 0.0005;
    }

    this.renderer.render(this.scene, this.camera);
  }

  animate() {
    requestAnimationFrame(() => this.animate());
    this.render();
  }

  onWindowResize() {
    this.camera.aspect = window.innerWidth / window.innerHeight;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(window.innerWidth, window.innerHeight);
  }

  dispose() {
    // Clean up resources
    this.scene.traverse((object) => {
      if (object.geometry) {
        object.geometry.dispose();
      }
      if (object.material) {
        if (Array.isArray(object.material)) {
          object.material.forEach(material => material.dispose());
        } else {
          object.material.dispose();
        }
      }
    });

    this.renderer.dispose();
  }
}

module.exports = VoxelEngine;