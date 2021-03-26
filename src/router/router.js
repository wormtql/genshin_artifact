import LoadingComponent from "@c/LoadingComponent";
import ErrorComponent from "@c/ErrorComponent";

const IntroPage = () => ({
    component: import(/* webpackChunkName: "intro-page" */ "@page/about/IntroPage"),
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
    component: import(/* webpackChunkName: "about-page" */ "@page/about/ChangeLogPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const AlgorithmPage = () => ({
    component: import(/* webpackChunkName: "about-page" */ "@page/about/AlgorithmPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const ExternalLinkPage = () => ({
    component: import(/* webpackChunkName: "about-page" */ "@page/about/ExternalLinkPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const ArtifactPotentialPage = () => ({
    component: import(/* webpackChunkName: "artifact-potential-page" */ "@page/ArtifactPotentialPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const PotentialFuncPage = () => ({
    component: import(/* webpackChunkName: "about-page" */ "@page/about/PotentialFuncPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const CharacterPresetsPage = () => (
    {
        component: import(/* webpackChunkName: "character-presets-page" */ "@page/CharacterPresetsPage"),
        loading: LoadingComponent,
        error: ErrorComponent,
    }
);
const HelpBasicPage = () => ({
    component: import(/* webpackChunkName: "help-page" */ "@page/helps/HelpBasicPage"),
    loading: LoadingComponent,
    error: ErrorComponent,
});
const FAQPage = () => import(/* webpackChunkName: "help-page" */ "@page/helps/FAQPage");
const TargetFuncExplanationPage = () => import(/* webpackChunkName: "help-page" */ "@page/helps/TargetFuncExplanationPage");

import VueRouter from "vue-router";

const webName = "莫娜占卜铺"

const routes = [
    {
        path: "/help/target-func-explanation",
        component: TargetFuncExplanationPage,
        meta: {
            title: "目标函数参数说明 | 帮助 | " + webName,
        }
    },
    {
        path: "/help/basic",
        component: HelpBasicPage,
        meta: {
            title: "基本使用 | 帮助 | " + webName,
        }
    },
    {
        path: "/help/faq",
        component: FAQPage,
        meta: {
            title: "FAQ | 帮助 | " + webName,
        }
    },
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
            title: "更新记录 | " + webName,
        }
    },
    {
        path: "/algorithm-target",
        component: AlgorithmPage,
        meta: {
            title: "目标函数 | " + webName,
        }
    },
    {
        path: "/algorithm-potential",
        component: PotentialFuncPage,
        meta: {
            title: "潜力函数 | " + webName,
        }
    },
    {
        path: "/tomodachi",
        component: ExternalLinkPage,
        meta: {
            title: "友情链接 | " + webName,
        }
    },
    {
        path: "/potential",
        component: ArtifactPotentialPage,
        meta: {
            keepAlive: true,
            title: "圣遗物潜力 | " + webName,
        }
    },
    {
        path: "/character-presets",
        component: CharacterPresetsPage,
        meta: {
            keepAlive: true,
            title: "角色预设 | " + webName,
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