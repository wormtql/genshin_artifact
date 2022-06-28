<template>
    <div class="root">
        <div>
            <el-icon :size="30"><component :is="icon"></component></el-icon>
        </div>

        <div>
            <span class="description">{{ description }}</span>
        </div>
    </div>
</template>

<script>
export default {
    name: "ButtonItem",
    props: ["icon", "description"]
}
</script>

<style scoped lang="scss">
.root {
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>