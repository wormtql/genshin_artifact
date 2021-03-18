<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">是否在冲刺后</h3>
            <el-switch v-model="value.afterDash" active-text="是" inactive-text="否"></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "Anxiangdejiuyushi.wcfg",
    props: ["value"],

    first() {
        return {
            afterDash: true,
        }
    }
}
</script>