/* eslint-disable */

const path = require("path")
const { readFileSync, existsSync } = require("fs")
// const webpack = require("webpack")
const { execSync } = require("child_process")
const yaml = require("js-yaml")
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin")
const AutoImport = require("unplugin-auto-import/webpack")
const Components = require("unplugin-vue-components/webpack")
const { ElementPlusResolver } = require("unplugin-vue-components/resolvers")
const IconsResolver = require("unplugin-icons/resolver")
const Icons = require("unplugin-icons/webpack")

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

const useCDN = !!customEnv["USE_CDN"]


// const resources = {
//     js: [
//         { url: "https://npm.elemecdn.com/vue@3.2.36/dist/vue.global.prod.js", global: "Vue", name: "vue" },
//         { url: "https://npm.elemecdn.com/vue-router@4.0.16/dist/vue-router.global.prod.js", global: "VueRouter", name: "vue-router" },
//         // { url: "https://npm.elemecdn.com/element-ui@2.15.6/lib/index.js", global: "ELEMENT", name: "element-ui" },
//         { url: "https://npm.elemecdn.com/echarts@5.3.0/dist/echarts.min.js", global: "echarts", name: "echarts" },
//         { url: "https://npm.elemecdn.com/vue-echarts@6.1.0/dist/index.umd.min.js", global: "VueECharts", name: "vue-echarts" },
//         { url: "https://npm.elemecdn.com/fuse.js@6.6.2/dist/fuse.min.js", global: "Fuse", name: "fuse.js" },
//         { url: "", global: "monaco", name: "monaco-editor" }
//     ],
//     css: [
//         // { url: "https://npm.elemecdn.com/element-ui@2.15.6/lib/theme-chalk/index.css" },
//         // { url: "https://npm.elemecdn.com/element-ui@2.15.6/lib/theme-chalk/display.css" },
//     ]
// }


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
        plugins: [
            AutoImport({
                imports: ["vue"],
                resolvers: [
                    ElementPlusResolver(),
                    IconsResolver({
                        prefix: "Icon"
                    })
                ],
                dts: path.resolve(__dirname, "src", 'auto-imports.d.ts'),
            }),
            Components({
                resolvers: [
                    ElementPlusResolver(),
                    IconsResolver({
                        enabledCollections: ["ep", "fa6-brands", "fa6-solid"]
                    })
                ],

                dts: path.resolve(__dirname, "src", 'components.d.ts'),
            }),
            Icons({
                autoInstall: true
            }),
        ],
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
                "mona": path.resolve(__dirname, "mona_wasm/pkg"),
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
        experiments: {
            asyncWebAssembly: true
        },
    },
    chainWebpack: config => {
        // use custom i18n loader
        config.module.rule("i18n-js")
            .test(/\.i18n$/)
            .use("i18n-loader")
                .loader("./loaders/i18n_loader.js")

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

        // use monaco webpack plugin if using non cdn mode
        if (!customEnv["USE_CDN"]) {
            config.plugin("monaco").use(new MonacoWebpackPlugin())
        }

        // set ifdef loader
        config.module.rule("js")
            .use("ifdef")
            .loader("ifdef-loader")
            .options({
                "USE_CDN": !!customEnv["USE_CDN"]
            })
        config.module.rule("vue")
            .use("ifdef")
            .loader("ifdef-loader")
            .options({
                "USE_CDN": !!customEnv["USE_CDN"]
            })

        if (useCDN) {
            const assets = {
                js: [
                    { url: "https://npm.elemecdn.com/vue@3.2.36/dist/vue.global.prod.js", name: "vue", global: "Vue" },
                    { url: "https://npm.elemecdn.com/vue-router@4.0.16/dist/vue-router.global.prod.js", name: "vue-router", global: "VueRouter" },
                    { url: "https://npm.elemecdn.com/echarts@5.3.3/dist/echarts.min.js", name: "echarts", global: "echarts" },
                    { url: "https://npm.elemecdn.com/vue-echarts@6.1.0/dist/index.umd.min.js", name: "vue-echarts", global: "VueECharts" },
                ],
                css: [
                    { url: "https://npm.elemecdn.com/element-plus@2.2.6/dist/index.css" },
                ]
            }

            const externals = {
                "monaco-editor": "monaco"
            }

            for (const item of assets.js) {
                externals[item.name] = item.global
            }

            config.externals(externals)

            config.plugin("html").tap(args => {
                args[0].cdn = assets
                return args
            })

            // config.externals({
            //     "monaco-editor": "var monaco",
            // })
            // config.set("externalsType", "script")
        }
    },
    productionSourceMap: false,
}