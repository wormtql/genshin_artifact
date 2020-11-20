// import IntroPage from "@/pages/intro_page/IntroPage";
// import ArtifactsPage from "@/pages/artifacts_page/ArtifactsPage";
// import CalculatePage from "@/pages/calculate_page/CalculatePage";
// import CustomCharacterPage from "@/pages/custom_character_page/CustomCharacterPage";
// import CustomWeaponPage from "@/pages/custom_weapon_page/CustomWeaponPage";
// import CustomTargetPage from "@/pages/custom_target_page/CustomTargetPage";
// import PanelCalculatePage from "@/pages/panel_calculate_page/PanelCalculatePage";
// import UseCasePage from "@/pages/use_case_page/UseCasePage";
// import AlgPage from "@/pages/alg_page/AlgPage";

// import LoadingComponent from "@/components/LoadingComponent";

const IntroPage = () => import("@/pages/intro_page/IntroPage");
const ArtifactsPage = () => import("@/pages/artifacts_page/ArtifactsPage");
const CalculatePage = () => import("@/pages/calculate_page/CalculatePage");
const CustomCharacterPage = () => import("@/pages/custom_character_page/CustomCharacterPage");
const CustomWeaponPage = () => import("@/pages/custom_weapon_page/CustomWeaponPage");
const CustomTargetPage = () => import("@/pages/custom_target_page/CustomTargetPage");
// const PanelCalculatePage = () => ({
//     component: import("@/pages/panel_calculate_page/PanelCalculatePage"),
//     loading: LoadingComponent,
//     error: LoadingComponent,
//     delay: 200,
//     timeout: 3000,
// });
const PanelCalculatePage = () => import("@/pages/panel_calculate_page/PanelCalculatePage");
const UseCasePage = () => import("@/pages/use_case_page/UseCasePage");
const AlgPage = () => import("@/pages/alg_page/AlgPage");

import VueRouter from "vue-router";

const routes = [
    {
        path: "/intro",
        component: IntroPage,
        alias: "/",
    },
    {
        path: "/artifacts",
        component: ArtifactsPage
    },
    {
        path: "/use-case",
        component: UseCasePage,
    },
    {
        path: "/calculate",
        component: CalculatePage,
        meta: {
            keepAlive: true,
        }
    },
    {
        path: "/custom-character",
        component: CustomCharacterPage,
    },
    {
        path: "/custom-weapon",
        component: CustomWeaponPage,
    },
    {
        path: "/custom-target",
        component: CustomTargetPage,
    },
    {
        path: "/panel-calculate",
        component: PanelCalculatePage,
        meta: {
            keepAlive: true,
        }
    },
    {
        path: "/alg",
        component: AlgPage,
    }
]

const router = new VueRouter({
    routes
})

export default router;