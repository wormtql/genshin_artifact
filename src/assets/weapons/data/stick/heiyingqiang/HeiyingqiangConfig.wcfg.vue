<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">是否攻击史莱姆</h3>
            <el-switch v-model="value.slime" active-text="是" inactive-text="否"></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "HeiyingqiangConfig.wcfg",
    props: {
        value: {
            type: Object,
            required: true,
        },
    },
    
    first() {
        return {
            slime: true,
        }
    }
}
</script>