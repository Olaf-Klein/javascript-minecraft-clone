/**
 * World renderer using Three.js
 */

import * as THREE from 'three';

export class WorldRenderer {
  constructor(scene) {
    this.scene = scene;
    this.chunks = new Map();
    this.meshes = [];
    
    // Create block textures
    this.createTextures();
  }

  /**
   * Create simple colored materials for blocks
   */
  createTextures() {
    this.materials = {
      0: null, // Air
      1: new THREE.MeshLambertMaterial({ color: 0x808080 }), // Stone
      2: new THREE.MeshLambertMaterial({ color: 0x8B4513 }), // Dirt
      3: this.createGrassMaterial(), // Grass
      4: new THREE.MeshLambertMaterial({ color: 0xF4A460 }), // Sand
      5: new THREE.MeshLambertMaterial({ 
        color: 0x1E90FF, 
        transparent: true, 
        opacity: 0.6 
      }), // Water
      6: new THREE.MeshLambertMaterial({ color: 0x545454 }), // Bedrock
      7: new THREE.MeshLambertMaterial({ color: 0x654321 }), // Oak log
      8: new THREE.MeshLambertMaterial({ color: 0x228B22 }), // Oak leaves
    };
  }

  /**
   * Create grass block material with colored top
   */
  createGrassMaterial() {
    return [
      new THREE.MeshLambertMaterial({ color: 0x654321 }), // Right
      new THREE.MeshLambertMaterial({ color: 0x654321 }), // Left
      new THREE.MeshLambertMaterial({ color: 0x00FF00 }), // Top (grass)
      new THREE.MeshLambertMaterial({ color: 0x8B4513 }), // Bottom (dirt)
      new THREE.MeshLambertMaterial({ color: 0x654321 }), // Front
      new THREE.MeshLambertMaterial({ color: 0x654321 })  // Back
    ];
  }

  /**
   * Load chunk data and create mesh
   */
  loadChunk(chunkData) {
    const chunkKey = `${chunkData.chunkX},${chunkData.chunkZ}`;
    
    // Remove old chunk if exists
    if (this.chunks.has(chunkKey)) {
      this.removeChunk(chunkData.chunkX, chunkData.chunkZ);
    }

    // Decompress blocks
    const blocks = this.decompressBlocks(chunkData.blocks);
    
    // Create chunk mesh
    const mesh = this.createChunkMesh(chunkData.chunkX, chunkData.chunkZ, blocks);
    
    if (mesh) {
      this.scene.add(mesh);
      this.chunks.set(chunkKey, {
        mesh,
        blocks,
        x: chunkData.chunkX,
        z: chunkData.chunkZ
      });
      this.meshes.push(mesh);
    }
  }

  /**
   * Decompress chunk blocks (run-length encoding)
   */
  decompressBlocks(compressed) {
    const blocks = [];
    for (let i = 0; i < compressed.length; i += 2) {
      const blockId = compressed[i];
      const count = compressed[i + 1];
      for (let j = 0; j < count; j++) {
        blocks.push(blockId);
      }
    }
    return blocks;
  }

  /**
   * Create mesh for a chunk
   */
  createChunkMesh(chunkX, chunkZ, blocks) {
    const geometry = new THREE.BufferGeometry();
    const vertices = [];
    const normals = [];
    const indices = [];
    const colors = [];
    const materialGroups = [];

    let vertexOffset = 0;

    // Iterate through all blocks in chunk
    for (let y = 0; y < 256; y++) {
      for (let z = 0; z < 16; z++) {
        for (let x = 0; x < 16; x++) {
          const blockId = blocks[this.getBlockIndex(x, y, z)];
          
          if (blockId === 0) continue; // Skip air
          
          const worldX = chunkX * 16 + x;
          const worldZ = chunkZ * 16 + z;
          
          // Check adjacent blocks to determine which faces to render
          const adjacentBlocks = {
            top: y < 255 ? blocks[this.getBlockIndex(x, y + 1, z)] : 0,
            bottom: y > 0 ? blocks[this.getBlockIndex(x, y - 1, z)] : 1,
            north: z > 0 ? blocks[this.getBlockIndex(x, y, z - 1)] : 0,
            south: z < 15 ? blocks[this.getBlockIndex(x, y, z + 1)] : 0,
            east: x < 15 ? blocks[this.getBlockIndex(x + 1, y, z)] : 0,
            west: x > 0 ? blocks[this.getBlockIndex(x - 1, y, z)] : 0
          };

          // Add faces only if adjacent block is air or transparent
          const faces = this.getVisibleFaces(blockId, adjacentBlocks);
          
          for (const face of faces) {
            this.addBlockFace(
              vertices, normals, indices, colors,
              worldX, y, worldZ,
              face, blockId, vertexOffset
            );
            vertexOffset += 4;
          }
        }
      }
    }

    if (vertices.length === 0) return null;

    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
    geometry.setIndex(indices);

    // Use a single material for simplicity
    const material = new THREE.MeshLambertMaterial({ 
      vertexColors: false,
      side: THREE.FrontSide
    });

    const mesh = new THREE.Mesh(geometry, material);
    mesh.userData.chunkX = chunkX;
    mesh.userData.chunkZ = chunkZ;
    
    return mesh;
  }

  /**
   * Get block index in flat array
   */
  getBlockIndex(x, y, z) {
    return y * (16 * 16) + z * 16 + x;
  }

  /**
   * Determine which faces are visible
   */
  getVisibleFaces(blockId, adjacent) {
    const faces = [];
    const isTransparent = (id) => id === 0 || id === 5; // Air or water

    if (isTransparent(adjacent.top)) faces.push('top');
    if (isTransparent(adjacent.bottom)) faces.push('bottom');
    if (isTransparent(adjacent.north)) faces.push('north');
    if (isTransparent(adjacent.south)) faces.push('south');
    if (isTransparent(adjacent.east)) faces.push('east');
    if (isTransparent(adjacent.west)) faces.push('west');

    return faces;
  }

  /**
   * Add a block face to geometry
   */
  addBlockFace(vertices, normals, indices, colors, x, y, z, face, blockId, vertexOffset) {
    const faceVertices = this.getFaceVertices(face);
    const faceNormal = this.getFaceNormal(face);
    const color = this.getBlockColor(blockId, face);

    // Add vertices
    for (const vertex of faceVertices) {
      vertices.push(x + vertex[0], y + vertex[1], z + vertex[2]);
      normals.push(faceNormal[0], faceNormal[1], faceNormal[2]);
      colors.push(color.r, color.g, color.b);
    }

    // Add indices (two triangles per face)
    indices.push(
      vertexOffset, vertexOffset + 1, vertexOffset + 2,
      vertexOffset, vertexOffset + 2, vertexOffset + 3
    );
  }

  /**
   * Get vertices for a cube face
   */
  getFaceVertices(face) {
    switch (face) {
      case 'top':
        return [[0, 1, 0], [1, 1, 0], [1, 1, 1], [0, 1, 1]];
      case 'bottom':
        return [[0, 0, 0], [0, 0, 1], [1, 0, 1], [1, 0, 0]];
      case 'north':
        return [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
      case 'south':
        return [[0, 0, 1], [0, 1, 1], [1, 1, 1], [1, 0, 1]];
      case 'east':
        return [[1, 0, 0], [1, 0, 1], [1, 1, 1], [1, 1, 0]];
      case 'west':
        return [[0, 0, 0], [0, 1, 0], [0, 1, 1], [0, 0, 1]];
      default:
        return [];
    }
  }

  /**
   * Get normal vector for a face
   */
  getFaceNormal(face) {
    switch (face) {
      case 'top': return [0, 1, 0];
      case 'bottom': return [0, -1, 0];
      case 'north': return [0, 0, -1];
      case 'south': return [0, 0, 1];
      case 'east': return [1, 0, 0];
      case 'west': return [-1, 0, 0];
      default: return [0, 0, 0];
    }
  }

  /**
   * Get color for a block face
   */
  getBlockColor(blockId, face) {
    const colors = {
      1: { r: 0.5, g: 0.5, b: 0.5 }, // Stone
      2: { r: 0.545, g: 0.271, b: 0.075 }, // Dirt
      3: face === 'top' 
        ? { r: 0.0, g: 1.0, b: 0.0 } 
        : { r: 0.545, g: 0.271, b: 0.075 }, // Grass
      4: { r: 0.957, g: 0.643, b: 0.376 }, // Sand
      5: { r: 0.118, g: 0.565, b: 1.0 }, // Water
      6: { r: 0.329, g: 0.329, b: 0.329 }, // Bedrock
      7: { r: 0.396, g: 0.263, b: 0.129 }, // Oak log
      8: { r: 0.133, g: 0.545, b: 0.133 }, // Oak leaves
    };

    return colors[blockId] || { r: 1, g: 0, b: 1 }; // Magenta for unknown
  }

  /**
   * Update a single block
   */
  updateBlock(x, y, z, blockId) {
    const chunkX = Math.floor(x / 16);
    const chunkZ = Math.floor(z / 16);
    const chunkKey = `${chunkX},${chunkZ}`;
    
    const chunk = this.chunks.get(chunkKey);
    if (!chunk) return;

    // Update block data
    const localX = ((x % 16) + 16) % 16;
    const localZ = ((z % 16) + 16) % 16;
    const index = this.getBlockIndex(localX, y, localZ);
    chunk.blocks[index] = blockId;

    // Rebuild chunk mesh
    this.removeChunk(chunkX, chunkZ);
    const mesh = this.createChunkMesh(chunkX, chunkZ, chunk.blocks);
    
    if (mesh) {
      this.scene.add(mesh);
      chunk.mesh = mesh;
      this.meshes.push(mesh);
    }
  }

  /**
   * Remove a chunk
   */
  removeChunk(chunkX, chunkZ) {
    const chunkKey = `${chunkX},${chunkZ}`;
    const chunk = this.chunks.get(chunkKey);
    
    if (chunk) {
      this.scene.remove(chunk.mesh);
      chunk.mesh.geometry.dispose();
      
      const meshIndex = this.meshes.indexOf(chunk.mesh);
      if (meshIndex > -1) {
        this.meshes.splice(meshIndex, 1);
      }
      
      this.chunks.delete(chunkKey);
    }
  }
}
