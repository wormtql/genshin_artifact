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
                <side-bar
                    :do-route="false"
                    @select="handleSelect"
                ></side-bar>
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

            <mona-footer style="margin-bottom: 24px"></mona-footer>
        </div>
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
    },
    methods: {
        handleSelect(index) {
            this.drawerVisible = false
            this.$router.push(index)
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
        z-index: 10000;

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
        overflow-x: hidden;
    }
}

</style>
