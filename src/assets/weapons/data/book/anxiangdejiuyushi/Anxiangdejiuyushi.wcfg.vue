<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">是否在冲刺后</h3>
            <el-switch
                v-model="afterDash"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "Anxiangdejiuyushi.wcfg",
    data() {
        return {
            afterDash: true,
        }
    },
    methods: {
        setData(d) {
            this.afterDash = d.afterDash;
        }
    }
}
</script>