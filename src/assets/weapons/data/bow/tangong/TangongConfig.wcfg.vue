<template>
    <div>
        <div class="config-title">
            <h3 class="config-title">在0.3秒内击中敌人</h3>
            <el-switch v-model="within03" active-text="是" inactive-text="否"></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "TangongConfig.wcfg",
    data: function () {
        return {
            within03: true,
        }
    }
}
</script>