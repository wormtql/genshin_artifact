<template>
    <el-container id="app">
        <el-aside width="300px" style="border-right: 1px solid #eee">
            <side-bar></side-bar>
        </el-aside>

        <el-main style="">
            <keep-alive>
                <router-view v-if="$route.meta.keepAlive"></router-view>
            </keep-alive>
            <router-view v-if="!$route.meta.keepAlive"></router-view>
        </el-main>
    </el-container>
</template>

<script>
// import HeadBar from "./pages/main_page/HeadBar"
import SideBar from "./pages/main_page/SideBar"

export default {
    name: 'App',
    components: {
        // HeadBar,
        SideBar,
    },
    data: function() {
        return {
            // items: [
            //     { name: "intro", text: "首页", icon: "el-icon-menu" },
            //     { name: "artifacts", text: "圣遗物", icon: "el-icon-s-help" },
            //     { name: "characters", text: "角色", icon: "el-icon-user-solid" },
            //     { name: "calculate", text: "计算", icon: "el-icon-cpu" }
            // ]
        }
    }
}
</script>

<style>
#app {
    height: 100vh;
}
</style>
