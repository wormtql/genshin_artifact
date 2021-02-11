const IntroPage = () => import("@page/IntroPage");
const ArtifactsPage = () => import("@page/ArtifactsPage");

// const UseCasePage = () => import("@/pages/use_case_page/UseCasePage");
// const AlgPage = () => import("@/pages/alg_page/AlgPage");

const ArtifactsPlanPage = () => import(/* webpackChunkName: "artifacts-plan-page" */ "@page/ArtifactsPlanPage");

import VueRouter from "vue-router";

const routes = [
    {
        path: "/intro",
        component: IntroPage,
        alias: "/",
    },
    {
        path: "/artifacts",
        component: ArtifactsPage,
        meta: {
            keepAlive: true,
        }
    },
    {
        path: "/calculate",
        component: ArtifactsPlanPage,
        meta: {
            keepAlive: true,
        }
    },
]

const router = new VueRouter({
    routes
})

export default router;