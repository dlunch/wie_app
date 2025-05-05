import path from "path";

import webpack from "webpack";
import { merge } from "webpack-merge";

import "webpack-dev-server";

// @ts-ignore: allowImportingTsExtensions
import commonConfig from "./webpack.config.common.ts";

const config: webpack.Configuration = merge(commonConfig, {
  mode: "development",
  devServer: {
    static: path.join(import.meta.dirname, "dist"),
    watchFiles: {
      paths: ["src/**/*.*"],
      options: {
        usePolling: true,
      },
    },
  },
});

export default config;
