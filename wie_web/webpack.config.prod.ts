import webpack from "webpack";
import merge from "webpack-merge";

import commonConfig from "./webpack.config.common";

const config: webpack.Configuration = merge(commonConfig, {
  mode: "production",
  devtool: false,
});

export default config;
