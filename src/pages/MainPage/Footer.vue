<template>
    <div>
        <div class="footer">
            <span class="footer-item">©{{ year }} 莫娜占卜铺</span>
            <span class="footer-item">#{{ revision }} {{ date }}</span>
<!--            <span class="footer-item">{{ date }}</span>-->
            <span class="footer-item"><a target="_blank" href="https://github.com/wormtql/genshin_artifact">Github</a></span>

        </div>
        <div
            style="text-align: center"
        >
            <a
                v-if="needBeian"
                target="_blank"
                href="https://beian.miit.gov.cn"
                class="fs-14"
                style="font-size: 0.8rem"
            >浙ICP备2021004987号-2</a>
        </div>
    </div>

</template>

<script>
export default {
    name: "Footer",
    data() {
        return {
            number: process.env.MONA_BEIAN_NUMBER,
            revision: process.env.MONA_REVISION,
            needBeian: process.env.MONA_NEED_BEIAN,
            date: process.env.MONA_BUILD_DATE,
            year: (new Date()).getFullYear(),
        }
    }
}
</script>

<style lang="scss" scoped>


.footer {
    display: flex;
    justify-content: center;
    align-items: baseline;
    //border-top: 1px solid #eee;
    // background: #123456;
    //padding-top: 20px;
    //margin-top: 20px;

    .footer-item {
        font-size: 0.8rem;
        color: #5e6d82;

        padding: 0 8px;
        border-right: 1px solid gray;

        &:last-of-type {
            //padding-right: 0;
            border-right: none;
        }

        a {
            text-decoration: none;
            color: #5e6d82
        }
    }

    //a {
    //    text-decoration: none;
    //    // color: white;
    //}
}
</style>