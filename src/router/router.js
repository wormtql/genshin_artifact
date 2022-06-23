import { defineAsyncComponent } from "vue"

import LoadingComponent from "@c/LoadingComponent"
import ErrorComponent from "@c/ErrorComponent"
// import NewArtifactPlanPage from "@page/NewArtifactPlanPage"
// import TeamOptimizationPage from "@page/TeamOptimizationPage"
import NewArtifactPotentialPage from "@page/NewArtifactPotentialPage"
import CharacterDBPage from "@page/CharacterDBPage"
import CharacterInfo from "@page/CharacterDBPage/CharacterInfo"
import MonaPlaygroundPage from "@page/MonaPlaygroundPage"
import ArtifactsPage from "@page/ArtifactsPage"

import { createRouter, createWebHashHistory, createWebHistory } from "vue-router"

const IntroPage = defineAsyncComponent({
    loader: () => import(/* webpackChunkName: "intro-page" */ "@page/about/IntroPage"),
    loadingComponent: LoadingComponent,
    errorComponent: ErrorComponent,
})
// const ArtifactsPage = defineAsyncComponent({
//     loader: () => import(/* webpackChunkName: "artifacts-page" */"@page/ArtifactsPage"),
//     loading: LoadingComponent,
//     error: ErrorComponent,
// });
const ExternalLinkPage = () => ({
    component: import(/* webpackChunkName: "about-page" */ "@page/about/ExternalLinkPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
// const CharacterPresetsPage = () => (
//     {
//         component: import(/* webpackChunkName: "character-presets-page" */ "@page/CharacterPresetsPage"),
//         loading: LoadingComponent,
//         error: ErrorComponent,
//     }
// );
const CharacterPresetsPage = () => import(/* webpackChunkName: "character-presets-page" */ "@page/CharacterPresetsPage")
const FAQPage = () => import(/* webpackChunkName: "help-page" */ "@page/helps/FAQPage");
const ExportToolPage = () => import(/* webpackChunkName: "help-page" */ "@page/helps/ExportToolPage");
const KumiPage = () => import (/* webpackChunkName: "kumi-page" */ "@page/KumiPage");
// const MonaPlaygroundPage = () => import(/* webpackChunkName: "playground-page" */ "@page/MonaPlaygroundPage");


const webName = process.env.MONA_TITLE;

const routes = [
    {
        path: "/playground",
        component: MonaPlaygroundPage,
        meta: {
            title: "Playground",
            keepAlive: true
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
    // {
    //     path: "/team-optimization",
    //     component: TeamOptimizationPage,
    //     meta: {
    //         title: "整队优化",
    //         keepAlive: true,
    //     }
    // },
    {
        path: "/artifacts-kumi",
        component: KumiPage,
        meta: {
            title: "圣遗物套装",
            keepAlive: true,
        }
    },
    // {
    //     path: "/artifacts-statistics",
    //     component: ArtifactsStatisticsPage,
    //     meta: {
    //         title: "圣遗物统计 | " + webName,
    //         keepAlive: true,
    //     }
    // },
    {
        path: "/help/export-tools",
        component: ExportToolPage,
        meta: {
            title: "导出工具汇总",
        }
    },
    {
        path: "/help/faq",
        component: FAQPage,
        meta: {
            title: "FAQ",
        }
    },
    {
        path: "/intro",
        component: () => Promise.resolve(IntroPage),
        alias: "/",
        meta: {
            title: "首页",
        }
    },
    {
        path: "/artifacts",
        component: ArtifactsPage,
        meta: {
            keepAlive: true,
            title: "圣遗物",
        }
    },
    // {
    //     path: "/calculate",
    //     name: "calculate",
    //     component: NewArtifactPlanPage,
    //     meta: {
    //         keepAlive: true,
    //         title: "星命定轨",
    //     }
    // },
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
});

export default router;