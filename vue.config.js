/* eslint-disable */

const path = require("path")
const fs = require("fs")
const packageJson = fs.readFileSync("./package.json")
const version = JSON.parse(packageJson).version || "no version"
const webpack = require("webpack")
const { execSync } = require("child_process")


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


let now = new Date();
let buildDate = now.toString();

module.exports = {
    publicPath: process.env.PublicPath || '/',
    configureWebpack: {
        resolve: {
            extensions: [".vue", ".png", ".jpg"],
            alias: {
                "@c": path.resolve(__dirname, "src/components"),
                "@asset": path.resolve(__dirname, "src/assets"),
                "@util": path.resolve(__dirname, "src/utils"),
                "@alg": path.resolve(__dirname, "src/algorithms"),
                "@page": path.resolve(__dirname, "src/pages"),
                "@worker": path.resolve(__dirname, "src/workers"),
                "@const": path.resolve(__dirname, "src/constants"),
                "@enemy": path.resolve(__dirname, "src/enemies"),
                "mona": path.resolve(__dirname, "mona/pkg"),

                "@wasm": path.resolve(__dirname, "src/wasm"),
                "@character": path.resolve(__dirname, "src/assets/character"),
                "@weapon": path.resolve(__dirname, "src/assets/weapon"),
                "@targetFunction": path.resolve(__dirname, "src/assets/target_function"),
                "@potentialFunction": path.resolve(__dirname, "src/assets/potential_function"),
                "@buff": path.resolve(__dirname, "src/assets/buff"),
                "@image": path.resolve(__dirname, "src/images"),
                "@artifact": path.resolve(__dirname, "src/assets/artifacts"),
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
                    BUILD_DATE: `"${buildDate}"`,
                }
            }),
            // new WorkerPlugin({
            //     globalObject: "self",
            // }),
        ],
        // entry: {
        //     "compute-worker": "./src/workers/compute.worker.js",
        //     "potential-worker": "./src/workers/compute_potential.worker.js",
        // },
        module: {
            rules: [
                // {
                //     test: /\.ccfg\.yaml$/,
                //     use: [
                //         "vue-loader",
                //         {
                //             loader: path.resolve(loaderPath, "character_config_loader.js"),
                //             options: {
                //                 type: "character"
                //             }
                //         }
                //     ]
                // },
                // {
                //     test: /\.cscfg\.yaml$/,
                //     use: [
                //         "vue-loader",
                //         {
                //             loader: path.resolve(loaderPath, "character_config_loader.js"),
                //             options: {
                //                 type: "characterSkill"
                //             }
                //         }
                //     ]
                // },
                // {
                //     test: /\.wcfg\.yaml$/,
                //     use: [
                //         "vue-loader",
                //         path.resolve(loaderPath, "weapon_config_loader.js")
                //     ]
                // },
                // {
                //     test: /\.tfcfg\.yaml$/,
                //     use: [
                //         "vue-loader",
                //         path.resolve(loaderPath, "target_function_config_loader.js")
                //     ]
                // },
            ]
        },
        externals: {
            vue: "Vue",
            "vue-router": "VueRouter",
            vuex: "Vuex",
            "element-ui": "ELEMENT",
            "vue-echarts": "VueECharts",
            "fuse.js": "Fuse",
        },
        experiments: {
            asyncWebAssembly: true
        }
    },
    // chainWebpack: config => config.resolve.symlinks(false),
    // chainWebpack: config => {
    //     config.module
    //         .rule("worker")
    //             .test(/\.worker\.js$/)
    //             .use("babel")
    //                 .loader("babel-loader")

    // },
    productionSourceMap: false,
}