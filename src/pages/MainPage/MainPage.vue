<template>
    <el-container id="container">
        <el-aside width="300px" style="border-right: 1px solid #eee">
            <side-bar></side-bar>
        </el-aside>

        <el-container>
            <el-main style="position: relative">
                <keep-alive>
                    <router-view v-if="$route.meta.keepAlive"></router-view>
                </keep-alive>
                <router-view v-if="!$route.meta.keepAlive"></router-view>
            </el-main>
        </el-container>
    </el-container>
</template>

<script>
import SideBar from "./SideBar"

export default {
    name: 'App',
    components: {
        SideBar,
    },
}
</script>

<style>
#container {
    height: 100vh;
}
</style>
