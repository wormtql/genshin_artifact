import IntroPage from "@/pages/intro_page/IntroPage";
import ArtifactsPage from "@/pages/artifacts_page/ArtifactsPage";
// import CharacterPage from "@/pages/character_page/CharacterPage";
import CalculatePage from "@/pages/calculate_page/CalculatePage";
import CustomCharacterPage from "@/pages/custom_character_page/CustomCharacterPage";
import CustomWeaponPage from "@/pages/custom_weapon_page/CustomWeaponPage";
import CustomTargetPage from "@/pages/custom_target_page/CustomTargetPage";
import PanelCalculatePage from "@/pages/panel_calculate_page/PanelCalculatePage";
import UseCasePage from "@/pages/use_case_page/UseCasePage";
import AlgPage from "@/pages/alg_page/AlgPage";

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