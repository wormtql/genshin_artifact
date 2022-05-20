<template>
    <main-page></main-page>
</template>

<script>
import MainPage from "@page/MainPage";

export default {
    name: 'App',
    components: {
        MainPage,
    },
}
</script>

<style lang="scss">
.el-scrollbar {
    .el-scrollbar__bar {
        opacity: 1!important;
    }
}
</style>