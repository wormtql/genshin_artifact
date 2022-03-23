<template>
    <div>
        <el-container id="container">
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

            <el-aside
                width="15%"
                style="border-right: 1px solid #eee; min-width: 200px"
                class="hidden-sm-and-down"
            >
                <side-bar></side-bar>
            </el-aside>

            <el-container
                
            >
                <el-header class="hidden-md-and-up" style="background: #409EFF">
                    <div class="flex-row" style="height: 100%">
                        <el-button
                            icon="el-icon-menu"
                            type="text"
                            @click="drawerVisible = true"
                            style="color: white"
                        >

                        </el-button>
                    </div>
                </el-header>
                <el-main class="main" style="position: relative">
                    <keep-alive>
                        <router-view v-if="$route.meta.keepAlive" class="router-view"></router-view>
                    </keep-alive>
                    <router-view v-if="!$route.meta.keepAlive" class="router-view"></router-view>

                    <beian v-if="needBeian"></beian>
<!--                    <p>123</p>-->
                </el-main>
            </el-container>
        </el-container>
    </div>
</template>

<script>
import SideBar from "./SideBar.vue"
import Beian from "./Beian.vue";

export default {
    name: 'App',
    components: {
        SideBar,
        Beian,
    },
    data: function () {
        return {
            drawerVisible: false,

            needBeian: process.env.NEED_BEIAN,
        }
    }
}
</script>

<style>
/* body {
    overflow: auto;
} */

.router-view {
    /* min-height: 100vh; */
    /* height: 100vh; */
    height: 100%;
}

#container {
    height: 100vh;
}

.main::-webkit-scrollbar {
    display: none;
}

.main {
    -ms-overflow-style: none;
    scrollbar-width: none;
}

/*.main {*/
/*    overflow-x: visible;*/
/*}*/
</style>
