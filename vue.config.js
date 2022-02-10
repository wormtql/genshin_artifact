const path = require("path");
const fs = require("fs");
const packageJson = fs.readFileSync("./package.json");
const version = JSON.parse(packageJson).version || "no version";
const webpack = require("webpack");


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

let singleFile = false;
if ("MONA_SINGLE_FILE" in process.env) {
    singleFile = !!process.env.MONA_SINGLE_FILE;
    routeMode = "hash";
    process.env.PublicPath = "./";
}

console.info(`
building with:
title = ${title}
needBeian = ${needBeian}
beianNumber = ${beianNumber}
needMigrate = ${needMigrate}
routeMode = ${routeMode}
singleFile = ${singleFile}
`);


let now = new Date();
let buildDate = now.toString();
process.env.VUE_APP_SINGLEFILE = singleFile
module.exports = {
    publicPath: process.env.PublicPath || '/',
    css: {
        extract: !singleFile,
    },
    filenameHashing: !singleFile,
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
                "@enemy": path.resolve(__dirname, "src/enemies"),
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
            })
        ],
        externals: (singleFile||process.env.NODE_ENV==='development')?{}:{
            vue: "Vue",
            "vue-router": "VueRouter",
            vuex: "Vuex",
            "element-ui": "ELEMENT",
            "vue-echarts": "VueECharts",
        }
    },
    chainWebpack: config => {
        if(singleFile){
            config.output.filename("[name].js");
            config.plugins.delete('prefetch')
            config.plugins.delete('preload')
            config.optimization.splitChunks({
                chunks: 'all',//split both async chunks and initial chunks(entrypoints)
                cacheGroups: {
                    default: false,
                    vendors: false
                }
            })
            config.plugin('limitchunk').use(
                new webpack.optimize.LimitChunkCountPlugin({
                    maxChunks: 1
                })
            )
            config.module
                .rule('images')
                .use('url-loader')
                .loader('url-loader')
                .tap((options) => Object.assign(options, { limit: 2000000 }))
            config.module
                .rule('fonts')
                .use('url-loader')
                .loader('url-loader')
                .tap((options) => Object.assign(options, { limit: 2000000 }))
            config.plugin('FaviconBase64Plugin').use({
                pluginName: "FaviconBase64Plugin",
                iconMatch: [],
                apply: function (compiler) {
                    console.log("Apply start");
                    compiler.hooks.compilation.tap(this.pluginName, (compilation) => {
                        compilation.hooks.htmlWebpackPluginAfterHtmlProcessing.tapAsync(
                            this.pluginName,
                            (htmlPluginData, cb) => {
                                const iconMatch = htmlPluginData.html.match(/<link rel="icon" href="([^"]+)"\/?>/);
                                if (iconMatch[0] && iconMatch[1]) {
                                    this.iconMatch.push({
                                        html: htmlPluginData.outputName,
                                        match: iconMatch,
                                    });
                                }
                                cb(null, htmlPluginData);
                            }
                        );
                    });
                    compiler.hooks.emit.tapAsync(this.pluginName, (compilation, cb) => {
                        if (this.iconMatch.length > 0) {
                            this.iconMatch.forEach((iconMatch) => {
                                const iconPath = iconMatch.match[1].replace(compilation.options.output.publicPath, "");
                                if (compilation.assets[iconPath] && compilation.assets[iconMatch.html]) {
                                    const iconData = compilation.assets[iconPath].source();
                                    const iconExt = iconPath.split(".").pop();
                                    const mimeType = iconExt === "ico" ? "image/x-icon" : `image/${iconExt}`;
                                    const iconBase64 = `data:${mimeType};base64,${iconData.toString("base64")}`;
                                    let htmlData = compilation.assets[iconMatch.html].source().toString();
                                    htmlData = htmlData.replace(
                                        iconMatch.match[0],
                                        `<link rel="icon" href="${iconBase64}" />`
                                    );
                                    compilation.assets[iconMatch.html] = {
                                        source: () => htmlData,
                                        size: () => htmlData.length,
                                    };
                                    delete compilation.assets[iconPath];
                                }
                            });
                        }
                        cb();
                    });
                },
            })
            config
                .plugin('ScriptExtHtmlWebpackPlugin')
                .before('copy')
                .use('script-ext-html-webpack-plugin', [
                    {
                        inline: [/app\.js$/],
                    },
                ])
        }
    },
    productionSourceMap: false,
}