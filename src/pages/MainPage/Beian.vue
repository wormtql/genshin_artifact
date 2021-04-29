<template>
    <div class="beian">
        <a
            target="_blank"
            href="https://beian.miit.gov.cn/"
            class="fs-14"
        >{{ number }}</a>
    </div>
</template>

<script>
export default {
    name: "Beian",
    data() {
        return {
            number: process.env.BEIAN_NUMBER,
        }
    }
}
</script>

<style lang="scss" scoped>
.beian {
    display: flex;
    justify-content: center;
    border-top: 1px solid #eee;
    // background: #123456;
    padding-top: 20px;
    margin-top: 20px;

    a {
        text-decoration: none;
        // color: white;
    }
}
</style>