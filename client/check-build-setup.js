#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

console.log('🔍 Checking JavaScript Minecraft Clone build setup...\n');

// Check if we're in the right directory
const rootPackageJson = path.join(__dirname, '..', 'package.json');
const clientPackageJson = path.join(__dirname, 'package.json');
const clientAssetsDir = path.join(__dirname, 'assets');
const clientIconSvg = path.join(__dirname, 'assets', 'icon.svg');

let allGood = true;

function checkFile(filePath, description) {
  if (fs.existsSync(filePath)) {
    console.log(`✅ ${description}: Found`);
    return true;
  } else {
    console.log(`❌ ${description}: Missing`);
    allGood = false;
    return false;
  }
}

function checkDependency(packageJsonPath, dependency, description) {
  if (fs.existsSync(packageJsonPath)) {
    const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    const deps = { ...pkg.dependencies, ...pkg.devDependencies };
    if (deps[dependency]) {
      console.log(`✅ ${description}: ${deps[dependency]}`);
      return true;
    } else {
      console.log(`❌ ${description}: Not found in ${path.basename(packageJsonPath)}`);
      allGood = false;
      return false;
    }
  }
  return false;
}

// Check root package.json
checkFile(rootPackageJson, 'Root package.json');

// Check client package.json
checkFile(clientPackageJson, 'Client package.json');

// Check assets directory
checkFile(clientAssetsDir, 'Client assets directory');

// Check icon SVG
checkFile(clientIconSvg, 'Icon SVG file');

// Check key dependencies
checkDependency(clientPackageJson, 'electron', 'Electron');
checkDependency(clientPackageJson, 'electron-builder', 'electron-builder');
checkDependency(clientPackageJson, 'sharp', 'Sharp (for icon generation)');
checkDependency(clientPackageJson, 'three', 'Three.js');

console.log('\n' + '='.repeat(50));

if (allGood) {
  console.log('🎉 All checks passed! Ready to build executables.');
  console.log('\nTo build executables:');
  console.log('1. Run: npm run install:all');
  console.log('2. Run: cd client && npm run dist:win  (for Windows)');
  console.log('   Or:  cd client && npm run dist:mac  (for macOS)');
  console.log('   Or:  cd client && npm run dist:linux (for Linux)');
} else {
  console.log('⚠️  Some issues found. Please check the errors above.');
  console.log('Make sure to run: npm run install:all');
}

console.log('\nFor detailed instructions, see: docs/build-executables.md');