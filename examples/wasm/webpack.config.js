const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const path = require('path');
const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: "./index.js",
  experiments: {
    syncWebAssembly: true,
  },
  output: {
    path: dist,
    filename: "index.js",
  },
  mode: "development",
  plugins: [
    new HtmlWebpackPlugin({
      title: "Freeverb",
      template: "./index.html",
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  devServer: {
    static: dist,
  }
};
