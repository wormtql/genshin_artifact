import NewArtifactPlanPage from "@page/NewArtifactPlanPage"
import TeamOptimizationPage from "@page/TeamOptimizationPage"
import NewArtifactPotentialPage from "@page/NewArtifactPotentialPage"
import CharacterDBPage from "@page/CharacterDBPage"
import CharacterInfo from "@page/CharacterDBPage/CharacterInfo"
import MonaPlaygroundPage from "@page/MonaPlaygroundPage"
import ArtifactsPage from "@page/ArtifactsPage"
import CalcBestArtifactSetPage from "@/pages/CalcBestArtifactSetPage"
import AccountPage from "@page/AccountPage"
// import ExternalLinkPage from "@/pages/about/ExternalLinkPage"
import SetupPage from "@page/SetupPage"

import { createRouter, createWebHashHistory, createWebHistory } from "vue-router"
import {getRepo} from "@/api/repo"
import {ElMessageBox} from "element-plus"
import "element-plus/es/components/message-box/style/css"
import {importMonaJson} from "@/utils/artifacts"
import {useArtifactStore} from "@/store/pinia/artifact"

const IntroPage = () => import(/* webpackChunkName: "intro-page" */ "@page/about/IntroPage")
const CharacterPresetsPage = () => import(/* webpackChunkName: "character-presets-page" */ "@page/CharacterPresetsPage")
const ExportToolPage = () => import(/* webpackChunkName: "help-page" */ "@page/helps/ExportToolPage")
const KumiPage = () => import (/* webpackChunkName: "kumi-page" */ "@page/KumiPage")
const ExternalLinkPage = () => import(/* webpackChunkName: "about-page" */ "@page/about/ExternalLinkPage")


const webName = process.env.MONA_TITLE;

const routes = [
    {
        path: "/best-set",
        component: CalcBestArtifactSetPage,
        name: "calcBest",
        meta: {
            title: "套装优化",
        }
    },
    {
        path: "/setup",
        component: SetupPage,
        name: "setup",
        meta: {
            title: "设置"
        }
    },
    {
        path: "/playground",
        component: MonaPlaygroundPage,
        name: "playground",
        meta: {
            title: "Playground",
        }
    },
    {
        path: "/character",
        component: CharacterDBPage,
        meta: {
            title: "角色",
            keepAlive: true
        },
        children: [
            {
                path: ":name",
                component: CharacterInfo,
                meta: {
                    title: "角色"
                }
            },
        ]
    },
    {
        path: "/team-optimization",
        component: TeamOptimizationPage,
        meta: {
            title: "整队优化",
            keepAlive: true,
        }
    },
    {
        path: "/artifacts-kumi",
        component: KumiPage,
        meta: {
            title: "圣遗物套装",
            keepAlive: true,
        }
    },
    {
        path: "/help/export-tools",
        component: ExportToolPage,
        meta: {
            title: "导出工具汇总",
        }
    },
    {
        path: "/intro",
        component: IntroPage,
        alias: "/",
        name: "home",
        meta: {
            title: "首页",
        }
    },
    {
        path: "/account",
        component: AccountPage,
        name: "account",
        meta: {
            keepAlive: true,
            title: "账号",
        }
    },
    {
        path: "/artifacts",
        component: ArtifactsPage,
        name: "artifact",
        meta: {
            keepAlive: true,
            title: "圣遗物",
        }
    },
    {
        path: "/calculate",
        name: "calculate",
        component: NewArtifactPlanPage,
        meta: {
            keepAlive: true,
            title: "莫娜计算器",
        }
    },
    {
        path: "/tomodachi",
        component: ExternalLinkPage,
        meta: {
            title: "友情链接",
        }
    },
    {
        path: "/potential",
        component: NewArtifactPotentialPage,
        meta: {
            keepAlive: true,
            title: "圣遗物潜力",
        }
    },
    {
        path: "/presets",
        component: CharacterPresetsPage,
        meta: {
            keepAlive: true,
            title: "计算预设",
        }
    },
]

const router = createRouter({
    routes,
    history: process.env.MONA_ROUTE_MODE === "hash" ? createWebHashHistory() : createWebHistory()
})

router.beforeEach((to, from, next) => {
    if (to.meta.title) {
        document.title = to.meta.title;
    }

    const title = to.meta.title
    document.title = `${title} | ${webName}`

    next();
})

router.beforeEach(async (to, from, next) => {
    if (to.path === "/import") {
        const artifactStore = useArtifactStore()
        const type = to.query["type"]
        const code = to.query["code"]

        if (type === "artifact" && code) {
            try {
                const response = await getRepo(code)
                if (response.status === 200) {
                    const contentStr = response.data.content
                    const content = JSON.parse(contentStr)
                    console.log(content)

                    if (artifactStore.artifactsCount.value > 0) {
                        await ElMessageBox.confirm("检测到本地存在圣遗物，是否继续导入", "注意", {
                            confirmButtonText: "是",
                            cancelButtonText: "否",
                            type: "warning"
                        }).then(() => {
                            importMonaJson(content, true)
                        }).catch(() => {})
                    } else {
                        importMonaJson(content, true)
                    }
                }
            } catch (e) {
                console.log(e)
            }

            // console.log("123")
            // setImmediate(() => {
            // redirect("artifact")
            next({ name: "artifact" })
        } else {
            next({ name: "home" })
        }
    } else {
        next()
    }
})

export default router;
