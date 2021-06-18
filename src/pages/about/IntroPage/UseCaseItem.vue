<template>
    <el-card
        shadow="hover"
        :body-style="{textAlign: 'center'}"
        class="root"
    >
        <font-awesome-icon :icon="['fas', icon]" class="icon"></font-awesome-icon>
        <font-awesome-icon :icon="['fas', icon]" class="icon2"></font-awesome-icon>
        <p class="text">{{ text }}</p>
    </el-card>
</template>

<script>
export default {
    name: "UseCaseItem",
    props: ["text", "icon"],
}
</script>

<style lang="scss" scoped>
.root {
    position: relative;

    .text {
        margin: 16px 0 0 0;
    }

    .icon2 {
        position: absolute;
        left: 0;
        bottom: 0;
        font-size: 4rem;
        opacity: 0.3;
        transform: rotateZ(-30deg);
    }
}


</style>