const path = require('path');

module.exports = {
  entry: './constants.js',
  output: {
    filename: 'shared.bundle.js',
    path: path.resolve(__dirname, 'dist'),
    library: 'shared',
    libraryTarget: 'umd',
  },
  mode: 'development',
};