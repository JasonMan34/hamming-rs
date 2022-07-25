const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/index.ts",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  output: {
    filename: "bootstrap.js",
    path: path.resolve(__dirname, "dist"),
  },
  mode: "development",
  plugins: [new CopyWebpackPlugin(["./src/index.html"])],
  experiments: {
    asyncWebAssembly: true,
  },
};
