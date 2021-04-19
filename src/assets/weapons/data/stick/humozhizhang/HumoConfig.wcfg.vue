<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">生命值低于50%</h3>
            <el-switch
                v-model="lifeBelow50"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "HumoConfig.wcfg",
    data() {
        return {
            lifeBelow50: false,
        }
    },
    methods: {
        setData(d) {
            this.lifeBelow50 = d.lifeBelow50;
        }
    }
}
</script>