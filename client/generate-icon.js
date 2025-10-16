const fs = require('fs');
const path = require('path');
const sharp = require('sharp');

async function createIcons() {
  const assetsDir = path.join(__dirname, 'assets');
  const svgPath = path.join(assetsDir, 'icon.svg');

  if (!fs.existsSync(svgPath)) {
    console.log('SVG icon not found. Please ensure assets/icon.svg exists.');
    return;
  }

  try {
    // Create PNG icon (512x512 for Linux)
    await sharp(svgPath)
      .png()
      .resize(512, 512)
      .toFile(path.join(assetsDir, 'icon.png'));
    console.log('Generated icon.png');

    // Create ICO icon (256x256 for Windows)
    await sharp(svgPath)
      .png()
      .resize(256, 256)
      .toFile(path.join(assetsDir, 'icon-256.png'));

    // For ICNS (macOS), electron-builder can handle SVG directly
    // but we'll create a PNG as backup
    await sharp(svgPath)
      .png()
      .resize(512, 512)
      .toFile(path.join(assetsDir, 'icon-512.png'));
    console.log('Generated icon-512.png');

    console.log('All icons generated successfully!');
  } catch (error) {
    console.error('Error generating icons:', error);
    console.log('Falling back to SVG icons (electron-builder can handle SVG)');
  }
}

if (require.main === module) {
  createIcons();
}

module.exports = { createIcons };