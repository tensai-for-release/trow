const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const devMode = process.env.NODE_ENV !== "production";

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: devMode ? "[name].[fullhash].js" : "[name].[chunkhash].js",
  },
  experiments: {
    // syncWebAssembly: true,
    asyncWebAssembly: true,
  },
  plugins: [
    new webpack.ProgressPlugin(),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "html", "index.html"),
    }),
    new CleanWebpackPlugin(),
    new MiniCssExtractPlugin({
      filename: devMode ? "[name].[fullhash].css" : "[name].[chunkhash].css",
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
      args: "",
      // Default arguments are `--typescript --target browser --mode normal`.
      extraArgs: "",
    }),
    // Have this work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"],
    }),
  ],
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss$/,
        use: [
          devMode ? "style-loader" : MiniCssExtractPlugin.loader,
          "css-loader",
          "sass-loader",
        ],
      },
    ],
  },

  optimization: {
    minimizer: [new TerserPlugin()],

    splitChunks: {
      cacheGroups: {
        vendors: {
          priority: -10,
          test: /[\\/]node_modules[\\/]/,
        },
      },

      chunks: "async",
      minChunks: 1,
      minSize: 30000,
      name: false,
    },
  },
};
