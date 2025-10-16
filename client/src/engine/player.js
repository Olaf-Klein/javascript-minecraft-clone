/**
 * Player controller
 */

import * as THREE from 'three';

export class Player {
  constructor() {
    this.position = new THREE.Vector3(0, 80, 0);
    this.velocity = new THREE.Vector3();
    this.onGround = false;
    this.moved = false;
    
    // Movement
    this.moveSpeed = 5.0;
    this.jumpVelocity = 8.0;
    this.gravity = 32.0;
    
    // Input state
    this.input = {
      forward: false,
      backward: false,
      left: false,
      right: false,
      jump: false,
      sprint: false
    };
    
    this.initInput();
  }

  initInput() {
    document.addEventListener('keydown', (e) => {
      switch (e.code) {
        case 'KeyW': this.input.forward = true; break;
        case 'KeyS': this.input.backward = true; break;
        case 'KeyA': this.input.left = true; break;
        case 'KeyD': this.input.right = true; break;
        case 'Space': this.input.jump = true; break;
        case 'ShiftLeft': this.input.sprint = true; break;
      }
    });

    document.addEventListener('keyup', (e) => {
      switch (e.code) {
        case 'KeyW': this.input.forward = false; break;
        case 'KeyS': this.input.backward = false; break;
        case 'KeyA': this.input.left = false; break;
        case 'KeyD': this.input.right = false; break;
        case 'Space': this.input.jump = false; break;
        case 'ShiftLeft': this.input.sprint = false; break;
      }
    });
  }

  update(deltaTime, controls) {
    const speed = this.input.sprint ? this.moveSpeed * 1.5 : this.moveSpeed;
    
    // Movement direction
    const direction = new THREE.Vector3();
    const forward = new THREE.Vector3();
    const right = new THREE.Vector3();
    
    controls.getDirection(forward);
    forward.y = 0;
    forward.normalize();
    
    right.crossVectors(forward, new THREE.Vector3(0, 1, 0)).normalize();
    
    if (this.input.forward) direction.add(forward);
    if (this.input.backward) direction.sub(forward);
    if (this.input.right) direction.add(right);
    if (this.input.left) direction.sub(right);
    
    if (direction.length() > 0) {
      direction.normalize();
      this.velocity.x = direction.x * speed;
      this.velocity.z = direction.z * speed;
      this.moved = true;
    } else {
      this.velocity.x = 0;
      this.velocity.z = 0;
    }
    
    // Jumping
    if (this.input.jump && this.onGround) {
      this.velocity.y = this.jumpVelocity;
      this.onGround = false;
      this.moved = true;
    }
    
    // Apply gravity
    if (!this.onGround) {
      this.velocity.y -= this.gravity * deltaTime;
    }
    
    // Update position
    this.position.x += this.velocity.x * deltaTime;
    this.position.y += this.velocity.y * deltaTime;
    this.position.z += this.velocity.z * deltaTime;
    
    // Simple ground check (Y = 64 for now)
    if (this.position.y <= 64) {
      this.position.y = 64;
      this.velocity.y = 0;
      this.onGround = true;
    }
  }
}
