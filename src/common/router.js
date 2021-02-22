import LoadingComponent from "@c/LoadingComponent";
import ErrorComponent from "@c/ErrorComponent";

const IntroPage = () => ({
    component: import(/* webpackChunkName: "intro-page" */ "@page/IntroPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const ArtifactsPage = () => ({
    component: import(/* webpackChunkName: "artifacts-page" */"@page/ArtifactsPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const ArtifactsPlanPage = () => ({
    component: import(/* webpackChunkName: "artifacts-plan-page" */ "@page/ArtifactsPlanPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const ChangeLogPage = () => ({
    component: import(/* webpackChunkName: "about-page" */ "@page/ChangeLogPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});

import VueRouter from "vue-router";

const webName = "莫娜占卜铺"

const routes = [
    {
        path: "/intro",
        component: IntroPage,
        alias: "/",
        meta: {
            title: "首页 | " + webName,
        }
    },
    {
        path: "/artifacts",
        component: ArtifactsPage,
        meta: {
            keepAlive: true,
            title: "圣遗物 | " + webName,
        }
    },
    {
        path: "/calculate",
        component: ArtifactsPlanPage,
        meta: {
            keepAlive: true,
            title: "星命定轨 | " + webName,
        }
    },
    {
        path: "/changelog",
        component: ChangeLogPage,
        meta: {
            title: "更新记录有 | " + webName,
        }
    }
]

const router = new VueRouter({
    routes
});

router.beforeEach((to, from, next) => {
    if (to.meta.title) {
        document.title = to.meta.title;
    }
    next();
});

export default router;