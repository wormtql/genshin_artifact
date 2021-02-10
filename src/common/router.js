const IntroPage = () => import("@/pages/intro_page/IntroPage");
const ArtifactsPage = () => import("@/pages/artifacts_page/ArtifactsPage");

// const UseCasePage = () => import("@/pages/use_case_page/UseCasePage");
// const AlgPage = () => import("@/pages/alg_page/AlgPage");

const ArtifactsPlanPage = () => import(/* webpackChunkName: "artifacts-plan-page" */ "@/pages/artifacts_plan_page");

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