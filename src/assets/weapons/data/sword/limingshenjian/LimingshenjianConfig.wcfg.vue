<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">生命值高于90%</h3>
            <el-switch v-model="value.lifeAbove90" active-text="是" inactive-text="否"></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "LimingshenjianConfig.wcfg",
    props: {
        value: {
            type: Object,
        }
    },

    first() {
        return {
            lifeAbove90: true,
        }
    }
}
</script>