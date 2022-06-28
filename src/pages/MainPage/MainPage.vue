<template>
<!--    <div class="root-div">-->
    <aside class="nav-bar mona-scroll">
        <side-bar></side-bar>
    </aside>

    <el-drawer
        title="导航"
        v-model="drawerVisible"
        direction="ltr"
        size="80%"
    >
        <div style="height: 100%; overflow: auto">
            <side-bar
                :do-route="false"
                @select="handleSelect"
            ></side-bar>
        </div>
    </el-drawer>

    <div class="header">
        <div class="flex-row" style="height: 100%">
            <el-button
                :icon="IconEpMenu"
                @click="drawerVisible = true"
                style="color: white"
                link
            ></el-button>
            <span class="header-title">{{ $route.meta.title }}</span>
        </div>
    </div>

    <div class="main mona-scroll">
        <div class="main-view">
            <router-view v-slot="{ Component }">
                <template v-if="Component">
                    <transition mode="out-in" name="el-fade-in-linear">
                        <keep-alive>
                            <component :is="Component"></component>
                        </keep-alive>
<!--                        <keep-alive>-->
<!--                            <suspense>-->
<!--                                <component :is="Component"></component>-->

<!--                                <template #fallback>-->
<!--                                    <div class="loading-container">-->
<!--                                        <simple-loading></simple-loading>-->
<!--                                    </div>-->

<!--                                </template>-->
<!--                            </suspense>-->
<!--                        </keep-alive>-->
                    </transition>
                </template>
            </router-view>
        </div>


<!--            <keep-alive>-->
<!--                <router-view v-if="$route.meta.keepAlive" class="router-view"></router-view>-->
<!--            </keep-alive>-->
<!--            <router-view v-if="!$route.meta.keepAlive" class="router-view"></router-view>-->
<!--            <router-view class="router-view"></router-view>-->

        <mona-footer style="margin-top: 24px"></mona-footer>
    </div>
<!--    </div>-->
</template>

<script setup lang="ts">
import SideBar from "./SideBar.vue"
import { default as MonaFooter } from "./Footer.vue"
import SimpleLoading from "@/components/loading/SimpleLoading.vue"

import IconEpMenu from "~icons/ep/menu"
import {useRouter} from "vue-router"

const drawerVisible = ref(false)

const router = useRouter()

function handleSelect(index: string) {
    drawerVisible.value = false
    router.push(index)
}
</script>

<style scoped lang="scss">
$contentPadding: 24px;
$side-bar-width: 15%;
$header-height: 48px;


@media only screen and (min-width: 992px) {
    //.root-div {
    //    display: flex;
    //    align-items: flex-start;
    //    //justify-content: flex-start;
    //}
    .main-view {
        min-height: calc(100vh - 2 * #{$contentPadding});
    }

    .header {
        display: none;
    }

    .nav-bar {
        width: $side-bar-width;
        height: 100vh;
        position: fixed;
        top: 0;
        bottom: 0;
        left: 0;
    }

    .main {
        //flex: 1;
        height: 100vh;
        margin-left: $side-bar-width;
        //min-height: 100vh;
        padding: $contentPadding;
        box-sizing: border-box;
    }

    .loading-container {
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    //.router-view {
    //    min-height: 100%;
    //    //padding: $contentPadding;
    //    box-sizing: border-box;
    //}
}

@media only screen and (max-width: 992px) {
    .header {
        height: $header-height;
        background-color: #409EFF;
        padding-left: 16px;
        position: fixed;
        top: 0;
        width: 100vw;
        z-index: 2000;

        .header-title {
            color: white;
            margin-left: 16px;
        }
    }

    .main {
        margin-top: 48px;
        padding: $contentPadding;
    }

    .main-view {
        min-height: calc(100vh - #{$header-height} - 2 * #{$contentPadding});
    }

    .nav-bar {
        display: none;
    }

    .router-view {
        min-height: calc(100% - 48px);
        padding: $contentPadding;
        box-sizing: border-box;
        overflow-x: hidden;
    }
}

</style>
