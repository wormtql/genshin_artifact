<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">队伍中的璃月角色数量</h3>
            <el-input-number v-model="value.liyueCount" :min="0" :max="4"></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "QianyanchangqiangConfig.wcfg",
    props: {
        value: {
            type: Object,
        }
    },

    first() {
        return {
            liyueCount: 0,
        }
    }
}
</script>