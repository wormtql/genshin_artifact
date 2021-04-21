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
                width="300px"
                style="border-right: 1px solid #eee"
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
                <el-main style="position: relative">
                    <keep-alive>
                        <router-view v-if="$route.meta.keepAlive" class="router-view"></router-view>
                    </keep-alive>
                    <router-view v-if="!$route.meta.keepAlive" class="router-view"></router-view>

                    <beian></beian>
                </el-main>
            </el-container>
        </el-container>
    </div>
</template>

<script>
import SideBar from "./SideBar"
import Beian from "./Beian";

export default {
    name: 'App',
    components: {
        SideBar,
        Beian,
    },
    data: function () {
        return {
            drawerVisible: false,
        }
    }
}
</script>

<style>
/* body {
    overflow: auto;
} */

.router-view {
    min-height: 100vh;
}

#container {
    height: 100vh;
}
</style>
