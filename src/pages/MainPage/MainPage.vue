<template>
    <div class="root-div">
        <aside class="nav-bar mona-scroll">
            <side-bar></side-bar>
        </aside>

        <el-drawer
            title="导航"
            :visible.sync="drawerVisible"
            direction="ltr"
            size="80%"
        >
            <div style="height: 100%; overflow: auto">
                <side-bar></side-bar>
            </div>
        </el-drawer>

        <div class="header">
            <div class="flex-row" style="height: 100%">
                <el-button
                    icon="el-icon-menu"
                    type="text"
                    @click="drawerVisible = true"
                    style="color: white"
                ></el-button>
                <span class="header-title">{{ $route.meta.title }}</span>
            </div>
        </div>

        <div class="main mona-scroll">
            <keep-alive>
                <router-view v-if="$route.meta.keepAlive" class="router-view"></router-view>
            </keep-alive>
            <router-view v-if="!$route.meta.keepAlive" class="router-view"></router-view>

            <mona-footer></mona-footer>
        </div>

<!--        <el-container id="container">-->
<!--            <el-drawer-->
<!--                title="导航"-->
<!--                :visible.sync="drawerVisible"-->
<!--                direction="ltr"-->
<!--                size="80%"-->
<!--            >-->
<!--                <div style="height: 100%; overflow: auto">-->
<!--                    <side-bar></side-bar>-->
<!--                </div>-->
<!--            </el-drawer>-->

<!--            <el-aside-->
<!--                width="15%"-->
<!--                style="border-right: 1px solid #eee; min-width: 200px"-->
<!--                class="hidden-sm-and-down aside"-->
<!--            >-->
<!--                <side-bar></side-bar>-->
<!--            </el-aside>-->

<!--            <el-container-->
<!--                -->
<!--            >-->
<!--                <el-header class="hidden-md-and-up" style="background: #409EFF">-->
<!--                    <div class="flex-row" style="height: 100%">-->
<!--                        <el-button-->
<!--                            icon="el-icon-menu"-->
<!--                            type="text"-->
<!--                            @click="drawerVisible = true"-->
<!--                            style="color: white"-->
<!--                        >-->

<!--                        </el-button>-->
<!--                    </div>-->
<!--                </el-header>-->
<!--                <el-main class="main" style="position: relative">-->
<!--                    <keep-alive>-->
<!--                        <router-view v-if="$route.meta.keepAlive" class="router-view"></router-view>-->
<!--                    </keep-alive>-->
<!--                    <router-view v-if="!$route.meta.keepAlive" class="router-view"></router-view>-->

<!--                    <mona-footer></mona-footer>-->
<!--&lt;!&ndash;                    <p>123</p>&ndash;&gt;-->
<!--                </el-main>-->

<!--            </el-container>-->
<!--        </el-container>-->
    </div>
</template>

<script>
import SideBar from "./SideBar.vue"
import Footer from "./Footer.vue";

export default {
    name: 'App',
    components: {
        SideBar,
        MonaFooter: Footer,
    },
    data: function () {
        return {
            drawerVisible: false,
        }
    }
}
</script>

<style scoped lang="scss">
$contentPadding: 24px;

@media only screen and (min-width: 992px) {
    .root-div {
        display: flex;
        align-items: flex-start;
        //justify-content: flex-start;
    }

    .header {
        display: none;
    }

    .nav-bar {
        width: 15%;
        height: 100vh;
    }

    .main {
        flex: 1;
        height: 100vh;
    }

    .router-view {
        min-height: 100%;
        padding: $contentPadding;
        box-sizing: border-box;
    }
}

@media only screen and (max-width: 992px) {
    .header {
        height: 48px;
        background-color: #409EFF;
        padding-left: 16px;
        position: fixed;
        top: 0;
        width: 100vw;
        z-index: 1000;

        .header-title {
            color: white;
            margin-left: 16px;
        }
    }

    .root-div {
        //position: relative;
    }

    .main {
        margin-top: 48px;
    }

    .nav-bar {
        display: none;
    }

    .router-view {
        min-height: calc(100% - 48px);
        padding: $contentPadding;
        box-sizing: border-box;
    }
}

.main {



}


/* body {
    overflow: auto;
} */

//@media only screen and (min-width: 992px) {
//    .router-view {
//         min-height: 100vh;
//        /* height: 100vh; */
//        /*height: 100%;*/
//    }
//}


#container {
    /*height: 100vh;*/
    /*min-height: 100vh;*/
}

//.main::-webkit-scrollbar {
//    display: none;
//}

//.main {
//    -ms-overflow-style: none;
//    scrollbar-width: none;
//}

//.aside {
//    height: 100vh;
//}

/*.main {*/
/*    overflow-x: visible;*/
/*}*/
</style>
