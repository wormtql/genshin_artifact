<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">在0.3秒内击中敌人</h3>
            <el-switch
                v-model="within03"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "TangongConfig.wcfg",

    data() {
        return {
            within03: true,
        };
    },
    methods: {
        setData(d) {
            this.within03 = d.within03;
        }
    }
}
</script>