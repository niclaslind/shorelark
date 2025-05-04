const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true, // <- Enable WASM support
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async", // <- Required for async WASM loading
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" }
      ]
    })
  ],
};