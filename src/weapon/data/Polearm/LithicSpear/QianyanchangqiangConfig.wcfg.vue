<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">队伍中的璃月角色数量</h3>
            <el-input-number
                v-model="liyueCount"
                :min="0"
                :max="4"
                size="small"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "QianyanchangqiangConfig.wcfg",
    data() {
        return {
            liyueCount: 0,
        }
    },
    methods: {
        setData(d) {
            this.liyueCount = d.liyueCount;
        }
    }
}
</script>