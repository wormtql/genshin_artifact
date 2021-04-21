const path = require("path");
const fs = require("fs");
const packageJson = fs.readFileSync("./package.json");
const version = JSON.parse(packageJson).version || "no version";
const webpack = require("webpack");
const WorkerPlugin = require("worker-plugin");


module.exports = {
    publicPath: process.env.PublicPath || '/',
    configureWebpack: {
        resolve: {
            alias: {
                "@c": path.resolve(__dirname, "src/components"),
                "@asset": path.resolve(__dirname, "src/assets"),
                "@util": path.resolve(__dirname, "src/utils"),
                "@alg": path.resolve(__dirname, "src/algorithms"),
                "@page": path.resolve(__dirname, "src/pages"),
                "@worker": path.resolve(__dirname, "src/workers"),
                "@const": path.resolve(__dirname, "src/constants"),
                // "genshin_panel": path.resolve(__dirname, "../../ts/genshin/dist"),
            }
        },
        plugins: [
            new webpack.DefinePlugin({
                "process.env": {
                    VERSION: `"${version}"`,
                    // WEB_TITLE: `"原神圣遗物工具"`,
                    WEB_TITLE: `"莫娜占卜铺"`,
                }
            }),
            new WorkerPlugin({
                globalObject: "self",
            }),
        ],
        // entry: {
        //     "compute-worker": "./src/workers/compute.worker.js",
        //     "potential-worker": "./src/workers/compute_potential.worker.js",
        // },
        // module: {
        //     rules: [
        //         {
        //             test: /\.worker\.js$/,
        //             use: [
        //                 {
        //                     loader: "worker-loader",
        //                     // options: {
        //                     //     filename: "js/[contenthash].[name].js",
        //                     // }
        //                 },
        //                 // "babel-loader"
        //             ],
        //         }
        //     ]
        // },
        externals: {
            vue: "Vue",
            "vue-router": "VueRouter",
            vuex: "Vuex",
            "element-ui": "ELEMENT",
        }
    },
    // chainWebpack: config => {
    //     config.module
    //         .rule("worker")
    //             .test(/\.worker\.js$/)
    //             .use("babel")
    //                 .loader("babel-loader")

    // },
    productionSourceMap: false,
}