const path = require("path");
const fs = require("fs");
const packageJson = fs.readFileSync("./package.json");
const version = JSON.parse(packageJson).version || "no version";
const webpack = require("webpack");

module.exports = {
    configureWebpack: {
        resolve: {
            alias: {
                "@c": path.resolve(__dirname, "src/components"),
                "@asset": path.resolve(__dirname, "src/assets"),
                "@util": path.resolve(__dirname, "src/utils"),
                "@alg": path.resolve(__dirname, "src/algorithms"),
                "@page": path.resolve(__dirname, "src/pages"),
            }
        },
        plugins: [
            new webpack.DefinePlugin({
                "process.env": {
                    VERSION: `"${version}"`
                }
            })
        ]
    }
}