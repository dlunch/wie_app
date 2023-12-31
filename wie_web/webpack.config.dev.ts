import path from "path";

import webpack from "webpack";
import merge from "webpack-merge";

import "webpack-dev-server";

import commonConfig from "./webpack.config.common";

const config: webpack.Configuration = merge(commonConfig, {
  mode: "development",
  devServer: {
    static: path.join(__dirname, "dist"),
    watchFiles: {
      paths: ["src/**/*.*"],
      options: {
        usePolling: true,
      },
    },
  },
});

export default config;
