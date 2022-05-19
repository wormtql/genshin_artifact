/* eslint-disable */

const path = require("path")
const { readFileSync, existsSync } = require("fs")
// const webpack = require("webpack")
const { execSync } = require("child_process")
const yaml = require("js-yaml")

// const BEIAN_CODE = "浙ICP备2021004987号";

const revision = execSync("git rev-parse HEAD").toString().trim().substring(0, 7)
console.log("revision: ", revision)

const version = JSON.parse(readFileSync("./package.json").toString()).version || "no version"

function getEnv() {
    const filename = process.env.ENV_FILE

    let result = {}
    if (existsSync(filename)) {
        const content = readFileSync(filename).toString()
        const parsed = yaml.load(content)
        // console.log(parsed)
        result = parsed
    }

    return result
}

function convertKVToDefinePlugin(obj) {
    let result = {}
    for (const key in obj) {
        const value = obj[key]
        const valueType = typeof value

        if (valueType === "string") {
            result[key] = "\"" + value + "\""
        } else if (valueType === "number") {
            result[key] = value
        } else if (valueType === "boolean") {
            result[key] = value.toString()
        } else {
            throw new Error("wrong type")
        }
    }

    return result
}

const customEnv = getEnv()

const now = new Date()
const year = now.getFullYear()
const month = now.getMonth()
const date = now.getDate()
const buildDate = `${year}/${month + 1}/${date}`


const resources = {
    js: [
        { url: "https://npm.elemecdn.com/vue@2.6.11/dist/vue.min.js", global: "Vue", name: "vue" },
        { url: "https://npm.elemecdn.com/vue-router@3.4.8/dist/vue-router.min.js", global: "VueRouter", name: "vue-router" },
        { url: "https://npm.elemecdn.com/vuex@3.5.1/dist/vuex.min.js", global: "Vuex", name: "vuex" },
        { url: "https://npm.elemecdn.com/element-ui@2.15.6/lib/index.js", global: "ELEMENT", name: "element-ui" },
        { url: "https://npm.elemecdn.com/@vue/composition-api@1.6.1/dist/vue-composition-api.prod.js", global: "VueCompositionAPI", name: "@vue/composition-api" },
        { url: "https://npm.elemecdn.com/echarts@5.3.0/dist/echarts.min.js", global: "echarts", name: "echarts" },
        { url: "https://npm.elemecdn.com/vue-echarts@6.0.0-rc.4/dist/index.umd.min.js", global: "VueECharts", name: "vue-echarts" },
        { url: "https://npm.elemecdn.com/fuse.js@6.5.3/dist/fuse.min.js", global: "Fuse", name: "fuse.js" },
    ],
    css: [
        { url: "https://npm.elemecdn.com/element-ui@2.15.6/lib/theme-chalk/index.css" },
        { url: "https://npm.elemecdn.com/element-ui@2.15.6/lib/theme-chalk/display.css" },
    ]
}


module.exports = {
    publicPath: process.env.PublicPath || '/',
    devServer: {
        proxy: {
            "^/api/compute_result/analysis": {
                // target: "http://localhost:8000"
                target: "https://www.mona-uranai.com/"
            },
            "^/api": {
                target: "http://localhost:8000/",
                changeOrigin: true
            }
        }
    },
    configureWebpack: {
        resolve: {
            extensions: [".vue", ".png", ".jpg", ".webp"],
            alias: {
                "@c": path.resolve(__dirname, "src/components"),
                "@asset": path.resolve(__dirname, "src/assets"),
                "@util": path.resolve(__dirname, "src/utils"),
                "@alg": path.resolve(__dirname, "src/algorithms"),
                "@page": path.resolve(__dirname, "src/pages"),
                "@worker": path.resolve(__dirname, "src/workers"),
                "@const": path.resolve(__dirname, "src/constants"),
                "@enemy": path.resolve(__dirname, "src/enemies"),
                "mona": path.resolve(__dirname, "sub/mona_wasm/pkg"),
                // "monaBg": path.resolve(__dirname, "sub/mona_wasm/pkg")

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
        // externals: process.env.NODE_ENV === "development" ? {} : {
        //     vue: "Vue",
        //     "vue-router": "VueRouter",
        //     vuex: "Vuex",
        //     "element-ui": "ELEMENT",
        //     "vue-echarts": "VueECharts",
        //     "fuse.js": "Fuse",
        //     "@vue/composition-api": "VueCompositionAPI",
        // },
        experiments: {
            asyncWebAssembly: true
        },
    },
    chainWebpack: config => {
        // merge custom env
        config.plugin("define").tap(definitions => {
            const definePluginConverted = convertKVToDefinePlugin(customEnv)
            definitions[0]["process.env"] = Object.assign(definitions[0]["process.env"], definePluginConverted, {
                MONA_VERSION: `"${version}"`,
                MONA_BUILD_DATE: `"${buildDate}"`,
                MONA_REVISION: `"${revision}"`
            })
            console.log(definitions)
            return definitions
        })

        // set ifdef loader
        config.module.rule("js")
            .use("ifdef")
            .loader("ifdef-loader")
            .options({
                "USE_CDN": !!customEnv["USE_CDN"]
            })

        // if (process.env.NODE_ENV === "production") {
        if (true) {
            // console.log("externals")
            // config.set("externalsType", "script")
            config.plugin("html").tap(args => {
                args[0].cdn = resources
                return args
            })

            let externals = {}
            for (const item of resources.js) {
                externals[item.name] = item.global
            }
            // console.log(externals)
            config.externals(externals)
            // config.externals({
                // "vue": [
                //     "https://npm.elemecdn.com/vue@2.6.11/dist/vue.min.js",
                //     "Vue"
                // ],
                // "vue-router": [
                //     "https://npm.elemecdn.com/vue-router@3.4.8/dist/vue-router.min.js",
                //     "VueRouter"
                // ],
                // "vuex": [
                //     "https://npm.elemecdn.com/vuex@3.5.1/dist/vuex.min.js",
                //     "Vuex"
                // ],
                // "element-ui": [
                //     "https://npm.elemecdn.com/element-ui@2.15.6/lib/index.js",
                //     "ELEMENT"
                // ],
                // "@vue/composition-api": [
                //     "https://npm.elemecdn.com/@vue/composition-api@1.6.1/dist/vue-composition-api.prod.js",
                //     "VueCompositionAPI"
                // ],
                // "e-charts": [
                //     "https://npm.elemecdn.com/echarts@5.3.0/dist/echarts.min.js",
                //     "echarts"
                // ],
                // "vue-echarts": [
                //     "https://npm.elemecdn.com/vue-echarts@6.0.0-rc.4/dist/index.umd.min.js",
                //     "VueECharts"
                // ],
                // "fuse.js": [
                //     "https://npm.elemecdn.com/fuse.js@6.5.3/dist/fuse.min.js",
                //     "Fuse"
                // ]
                // vue: "Vue",
                // "vue-router": "VueRouter",
                // vuex: "Vuex",
                // "element-ui": "ELEMENT",
                // "vue-echarts": "VueECharts",
                // "fuse.js": "Fuse",
                // "@vue/composition-api": "VueCompositionAPI",
            // })
        }
    },
    productionSourceMap: false,
}