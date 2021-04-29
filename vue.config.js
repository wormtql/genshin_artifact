const path = require("path");
const fs = require("fs");
const packageJson = fs.readFileSync("./package.json");
const version = JSON.parse(packageJson).version || "no version";
const webpack = require("webpack");
const WorkerPlugin = require("worker-plugin");


const BEIAN_CODE = "浙ICP备2021004987号";

let title = "莫娜占卜铺";
if ("MONA_TITLE" in process.env) {
    title = process.env.MONA_TITLE;
}

let needBeian = true;
if ("MONA_NEED_BEIAN" in process.env) {
    needBeian = !!eval(process.env.MONA_NEED_BEIAN);
}

let beianNumber = "1";
if ("MONA_BEIAN_NUMBER" in process.env) {
    beianNumber = process.env.MONA_BEIAN_NUMBER;
}

let needMigrate = false;
if ("MONA_NEED_MIGRATE" in process.env) {
    needMigrate = !!eval(process.env.MONA_NEED_MIGRATE);
}

let routeMode = "hash";
if ("MONA_ROUTE_MODE" in process.env) {
    routeMode = process.env.MONA_ROUTE_MODE;
}

console.info(`
building with:
title = ${title}
needBeian = ${needBeian}
beianNumber = ${beianNumber}
needMigrate = ${needMigrate}
routeMode = ${routeMode}
`);


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
                    WEB_TITLE: `"${title}"`,
                    NEED_BEIAN: `${needBeian}`,
                    BEIAN_NUMBER: `"${BEIAN_CODE + '-' + beianNumber}"`,
                    NEED_MIGRATE: `${needMigrate}`,
                    ROUTE_MODE: `"${routeMode}"`,
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
            "vue-echarts": "VueECharts",
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