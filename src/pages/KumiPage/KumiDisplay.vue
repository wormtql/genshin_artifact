<template>
    <div>
        <div
            v-for="dir in dirs"
            :key="dir.id"
            class="item is-active"
            :class="{ active: value === dir.id }"
            @click="$emit('input', dir.id)"
        >
            {{ dir.label }}
        </div>
    </div>
</template>

<script>
export default {
    name: "KumiDisplay",
    props: ["value"],
    computed: {
        dirs() {
            let tree = this.$store.state.kumi["tree"];

            return (tree.children ?? [])
        }
    }
}
</script>

<style lang="scss" scoped>
$height: 40px;

.item {
    height: $height;
    padding-left: 20px;
    font-size: 14px;
    cursor: pointer;
    line-height: $height;
    color: #303133;
    transition: 200ms;

    &.active {
        color: #409EFF;
        background: #ecf5ff;
    }

    &:hover {
        background: #ecf5ff;
    }
}
</style>