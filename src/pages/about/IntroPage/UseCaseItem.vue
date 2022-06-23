<template>
    <div class="root"
        @click="handleClick"
    >
        <el-icon><slot name="icon"></slot></el-icon>
        <p class="text">{{ text }}</p>
    </div>
</template>

<script>
export default {
    name: "UseCaseItem",
    props: ["text", "url"],
    methods: {
        handleClick() {
            console.log(this.url)
            if (this.url) {
                this.$router.replace(this.url)
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.root {
    position: relative;
    text-align: center;
    border: 1px solid #00000011;
    height: 100px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    cursor: pointer;

    &:hover {
        background-color: rgb(251, 249, 255);

        .icon2 {
            transform: none;
        }
    }

    .text {
        margin: 16px 0 0 0;
        font-size: 0.9em;
    }

    .icon2 {
        position: absolute;
        left: 0;
        bottom: 0;
        font-size: 4rem;
        opacity: 0.2;
        color: rgb(86,72,132);
        transform: rotateZ(-30deg);
        transition: 300ms;
    }
}


</style>