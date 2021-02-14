const IntroPage = () => import(/* webpackChunkName: "intro-page" */ "@page/IntroPage");
const ArtifactsPage = () => import(/* webpackChunkName: "artifacts-page" */"@page/ArtifactsPage");
const ArtifactsPlanPage = () => import(/* webpackChunkName: "artifacts-plan-page" */ "@page/ArtifactsPlanPage");

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