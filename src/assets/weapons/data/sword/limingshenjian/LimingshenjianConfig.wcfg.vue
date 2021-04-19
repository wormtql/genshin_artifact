<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">生命值高于90%</h3>
            <el-switch
                v-model="lifeAbove90"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "LimingshenjianConfig.wcfg",
    data() {
        return {
            lifeAbove90: true,
        }
    },
    methods: {
        setData(d) {
            this.lifeAbove90 = d.lifeAbove90;
        }
    }
}
</script>