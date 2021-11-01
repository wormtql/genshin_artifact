<template>
    <a class="noknok" :href="link" target="_blank">
        <!-- <img :src="background" class="background" > -->
        <img src="./banner-1950x180-logo.png" class="logo">
        <img src="./banner-1950x180-qrcode.png" class="qrcode hidden-sm-and-down">
        <img src="./banner-1950x180-text.png" class="text hidden-sm-and-down">
        <img src="./banner-1950x180-text.png" class="text2 hidden-md-and-up">
    </a>
</template>

<script>

export default {
    name: "NokNok",
    data: function () {
        return {
            link: "https://www.noknok.cn/act/share_group_20210625/index.html?uid=100000041&gid=117757&_sd=eyJwcmVmaXgiOiJub2tub2siLCJzaGFyZXVpZCI6IjEwMDAwMDA0MSJ9"
        }
    }
}
</script>

<style scoped lang="scss">
.noknok {
    background-image: url("./banner-1950x180-bg.jpg");
    // background-position: -150px 0;
    height: 180px;
    display: block;
    position: relative;
    overflow: hidden;

    .logo {
        position: absolute;
        margin-left: 8px;
        margin-top: 8px;;
        // left: 60%;
        // top: 50%;
    }

    .qrcode {
        position: absolute;
        right: 8px;
        bottom: 8px;
        height: 80px;
    }

    .text {
        position: absolute;
        // width: calc(100% - 236px - 120px);
        width: 50%;
        max-height: 100%;
        right: 120px;
        // height: 160px;
        // max-width: 100%;
        top: 50%;
        transform: translateY(-50%);
    }

    .text2 {
        position: absolute;

        bottom: 8px;
        // height: 80px;
        width: 100%;
        // left: -32px;
    }
}
</style>