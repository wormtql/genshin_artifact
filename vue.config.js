const path = require("path");
const fs = require("fs");
const packageJson = fs.readFileSync("./package.json");
const version = JSON.parse(packageJson).version || "no version";
const webpack = require("webpack");
const WorkerPlugin = require("worker-plugin");


module.exports = {
    configureWebpack: {
        resolve: {
            alias: {
                "@c": path.resolve(__dirname, "src/components"),
                "@asset": path.resolve(__dirname, "src/assets"),
                "@util": path.resolve(__dirname, "src/utils"),
                "@alg": path.resolve(__dirname, "src/algorithms"),
                "@page": path.resolve(__dirname, "src/pages"),
                "@worker": path.resolve(__dirname, "src/workers"),
                // "genshin_panel": path.resolve(__dirname, "../../ts/genshin/dist"),
            }
        },
        plugins: [
            new webpack.DefinePlugin({
                "process.env": {
                    VERSION: `"${version}"`
                }
            }),
            new WorkerPlugin(),
        ],
        // module: {
        //     rules: [
        //         {
        //             test: /\.worker\.js$/,
        //             use: [
        //                 {
        //                     loader: "worker-loader",
        //                     options: {
        //                         filename: "[hash].worker.js",
        //                     }
        //                 },
        //                 "babel-loader"
        //             ],
        //         }
        //     ]
        // }
        externals: {
            vue: "Vue",
            "vue-router": "VueRouter",
            vuex: "Vuex",
            "element-ui": "ELEMENT",
        }
    },
    // chainWebpack: config => {
    //     config.plugin("")
    //     config.plugin("html").tag(args => {
    //         args[0].template = "";
    //         return args;
    //     })
    // },
    productionSourceMap: false,
}