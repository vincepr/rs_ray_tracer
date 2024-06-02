const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const crypto = require("crypto");
const crypto_orig_createHash = crypto.createHash;
crypto.createHash = algorithm => crypto_orig_createHash(algorithm == "md4" ? "sha256" : algorithm);

const dist = path.resolve(__dirname, "dist");

const appConfig = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),
  ]
};
const workerConfig = {
  mode: "production",
  entry: "./js/render.worker.js",
  target: "webworker",
  resolve: {
    extensions: [".js", ".wasm"]
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    })
  ],
  output: {
    path: dist,
    filename: "worker.js"
  }
};

module.exports = [appConfig, workerConfig]