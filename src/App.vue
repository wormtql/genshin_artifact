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
import SideBar from "./pages/main_page/SideBar"

export default {
    name: 'App',
    components: {
        SideBar,
    },
}
</script>

<style>
#app {
    height: 100vh;
}
</style>
