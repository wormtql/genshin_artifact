<template>
    <div id="mainPageHeadBar" :style="{background: color}">
        <span class="logo">
            原神圣遗物
        </span>
        <button v-for="item in items" :key="item.name">
            {{ item.text }}
        </button>
    </div>
</template>

<script>
export default {
    name: "HeadBar",
    props: {
        color: {
            type: String,
            default: "#345678"
        },
        items: {
            type: Array,
            default: () => []
        }
    }
}
</script>

<style scoped>
.logo {
    padding: 0 32px;
    color: white;
    line-height: 64px;
}

#mainPageHeadBar {
    height: 64px;
    position: fixed;
    width: 100vw;
    margin: 0;
}
</style>