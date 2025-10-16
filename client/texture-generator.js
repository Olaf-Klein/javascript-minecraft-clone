// Texture generator for creating procedural Minecraft-style textures
class TextureGenerator {
  static createGrassBlockTop() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Base green color
    ctx.fillStyle = '#8DB360';
    ctx.fillRect(0, 0, 16, 16);

    // Add some grass-like variation
    ctx.fillStyle = '#6B8E23';
    for (let i = 0; i < 16; i += 2) {
      for (let j = 0; j < 16; j += 2) {
        if (Math.random() > 0.7) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }

  static createGrassBlockSide() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Dirt base
    ctx.fillStyle = '#8B4513';
    ctx.fillRect(0, 0, 16, 16);

    // Grass overlay on top half
    ctx.fillStyle = '#228B22';
    ctx.fillRect(0, 0, 16, 8);

    // Add some texture variation
    ctx.fillStyle = '#32CD32';
    for (let i = 0; i < 16; i += 3) {
      if (Math.random() > 0.5) {
        ctx.fillRect(i, Math.floor(Math.random() * 6), 2, 1);
      }
    }

    return canvas;
  }

  static createDirt() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Base dirt color
    ctx.fillStyle = '#8B4513';
    ctx.fillRect(0, 0, 16, 16);

    // Add darker spots for texture
    ctx.fillStyle = '#654321';
    for (let i = 0; i < 16; i += 2) {
      for (let j = 0; j < 16; j += 2) {
        if (Math.random() > 0.8) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }

  static createStone() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Base stone color
    ctx.fillStyle = '#808080';
    ctx.fillRect(0, 0, 16, 16);

    // Add lighter and darker spots
    ctx.fillStyle = '#A0A0A0';
    for (let i = 0; i < 16; i += 2) {
      for (let j = 0; j < 16; j += 2) {
        if (Math.random() > 0.7) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    ctx.fillStyle = '#606060';
    for (let i = 0; i < 16; i += 3) {
      for (let j = 0; j < 16; j += 3) {
        if (Math.random() > 0.8) {
          ctx.fillRect(i, j, 2, 2);
        }
      }
    }

    return canvas;
  }

  static createOakLog() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Base wood color
    ctx.fillStyle = '#DEB887';
    ctx.fillRect(0, 0, 16, 16);

    // Add wood grain lines
    ctx.strokeStyle = '#8B4513';
    ctx.lineWidth = 1;
    for (let i = 0; i < 16; i += 3) {
      ctx.beginPath();
      ctx.moveTo(i, 0);
      ctx.lineTo(i + Math.random() * 2 - 1, 16);
      ctx.stroke();
    }

    return canvas;
  }

  static createOakLogTop() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Wood color
    ctx.fillStyle = '#DEB887';
    ctx.fillRect(0, 0, 16, 16);

    // Add concentric rings for tree rings
    ctx.strokeStyle = '#8B4513';
    ctx.lineWidth = 1;
    for (let r = 2; r < 8; r += 2) {
      ctx.beginPath();
      ctx.arc(8, 8, r, 0, Math.PI * 2);
      ctx.stroke();
    }

    return canvas;
  }

  static createOakLeaves() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Transparent background
    ctx.clearRect(0, 0, 16, 16);

    // Leaf color
    ctx.fillStyle = '#228B22';
    ctx.fillRect(0, 0, 16, 16);

    // Add some darker spots
    ctx.fillStyle = '#006400';
    for (let i = 0; i < 16; i += 2) {
      for (let j = 0; j < 16; j += 2) {
        if (Math.random() > 0.85) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }

  static createOakPlanks() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Wood color
    ctx.fillStyle = '#DEB887';
    ctx.fillRect(0, 0, 16, 16);

    // Add plank lines
    ctx.strokeStyle = '#8B4513';
    ctx.lineWidth = 1;
    for (let i = 0; i < 16; i += 4) {
      ctx.beginPath();
      ctx.moveTo(0, i);
      ctx.lineTo(16, i);
      ctx.stroke();
    }

    return canvas;
  }

  static createSand() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Base sand color
    ctx.fillStyle = '#F4A460';
    ctx.fillRect(0, 0, 16, 16);

    // Add lighter and darker grains
    ctx.fillStyle = '#DAA520';
    for (let i = 0; i < 16; i++) {
      for (let j = 0; j < 16; j++) {
        if (Math.random() > 0.9) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    ctx.fillStyle = '#CD853F';
    for (let i = 0; i < 16; i++) {
      for (let j = 0; j < 16; j++) {
        if (Math.random() > 0.95) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }

  static createWater() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Water color (semi-transparent)
    ctx.fillStyle = 'rgba(0, 191, 255, 0.8)';
    ctx.fillRect(0, 0, 16, 16);

    // Add some wave-like patterns
    ctx.strokeStyle = 'rgba(0, 150, 255, 0.3)';
    ctx.lineWidth = 1;
    for (let i = 0; i < 16; i += 4) {
      ctx.beginPath();
      ctx.moveTo(0, i);
      ctx.quadraticCurveTo(8, i - 2, 16, i);
      ctx.stroke();
    }

    return canvas;
  }

  static createCoalOre() {
    const canvas = this.createStone();
    const ctx = canvas.getContext('2d');

    // Add coal deposits
    ctx.fillStyle = '#2F2F2F';
    for (let i = 0; i < 16; i += 3) {
      for (let j = 0; j < 16; j += 3) {
        if (Math.random() > 0.7) {
          const size = Math.random() > 0.8 ? 2 : 1;
          ctx.fillRect(i, j, size, size);
        }
      }
    }

    return canvas;
  }

  static createIronOre() {
    const canvas = this.createStone();
    const ctx = canvas.getContext('2d');

    // Add iron deposits
    ctx.fillStyle = '#D2691E';
    for (let i = 0; i < 16; i += 4) {
      for (let j = 0; j < 16; j += 4) {
        if (Math.random() > 0.8) {
          ctx.fillRect(i, j, 2, 2);
        }
      }
    }

    return canvas;
  }

  static createGoldOre() {
    const canvas = this.createStone();
    const ctx = canvas.getContext('2d');

    // Add gold deposits
    ctx.fillStyle = '#FFD700';
    for (let i = 0; i < 16; i += 5) {
      for (let j = 0; j < 16; j += 5) {
        if (Math.random() > 0.85) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }

  static createDiamondOre() {
    const canvas = this.createStone();
    const ctx = canvas.getContext('2d');

    // Add diamond deposits
    ctx.fillStyle = '#B0E0E6';
    for (let i = 0; i < 16; i += 6) {
      for (let j = 0; j < 16; j += 6) {
        if (Math.random() > 0.9) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }

  static createBedrock() {
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Dark bedrock color
    ctx.fillStyle = '#1C1C1C';
    ctx.fillRect(0, 0, 16, 16);

    // Add some lighter spots
    ctx.fillStyle = '#2C2C2C';
    for (let i = 0; i < 16; i += 2) {
      for (let j = 0; j < 16; j += 2) {
        if (Math.random() > 0.8) {
          ctx.fillRect(i, j, 1, 1);
        }
      }
    }

    return canvas;
  }
}

module.exports = TextureGenerator;