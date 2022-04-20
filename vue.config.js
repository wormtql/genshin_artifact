/* eslint-disable */

const path = require("path")
const { readFileSync, existsSync } = require("fs")
// const webpack = require("webpack")
const { execSync } = require("child_process")
const yaml = require("js-yaml")

// const BEIAN_CODE = "浙ICP备2021004987号";

const revision = execSync("git rev-parse HEAD").toString().trim().substring(0, 6)
console.log("revision: ", revision)

const version = JSON.parse(readFileSync("./package.json").toString()).version || "no version"

function getEnv() {
    let filename = ".env.development.yaml"
    if (process.env.NODE_ENV === "production") {
        filename = ".env.production.yaml"
    }

    let result = {}
    if (existsSync(filename)) {
        const content = readFileSync(filename).toString()
        const parsed = yaml.load(content)
        // console.log(parsed)
        result = parsed
    }

    return result
}

const now = new Date()
const year = now.getFullYear()
const month = now.getMonth()
const date = now.getDate()
const buildDate = `${year}/${month + 1}/${date}`

module.exports = {
    publicPath: process.env.PublicPath || '/',
    devServer: {
        proxy: {
            "^/api/compute_result/analysis": {
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
        externals: {
            vue: "Vue",
            "vue-router": "VueRouter",
            vuex: "Vuex",
            "element-ui": "ELEMENT",
            "vue-echarts": "VueECharts",
            "fuse.js": "Fuse",
            "@vue/composition-api": "VueCompositionAPI",
        },
        experiments: {
            asyncWebAssembly: true
        }
    },
    chainWebpack: config => {
        config.plugin("define").tap(definitions => {
            const customEnv = getEnv()

            definitions[0]["process.env"] = Object.assign(definitions[0]["process.env"], customEnv, {
                MONA_VERSION: `"${version}"`,
                MONA_BUILD_DATE: `"${buildDate}"`,
                MONA_REVISION: `"${revision}"`
            })
            // console.log(definitions)
            return definitions
        })
    },
    productionSourceMap: false,
}