<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均层数</h3>
            <el-slider
                v-model="level"
                :min="0"
                :max="6"
                :step="0.1"
                show-input
            ></el-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">是否处于后台</h3>
            <el-switch
                v-model="isBackend"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "Xizai.wcfg",
    data() {
        return {
            isBackend: false,
            level: 3,
        }
    },
    methods: {
        setData(d) {
            this.isBackend = d.isBackend ?? false;
            this.level = d.level ?? 3;
        }
    }
}
</script>