<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">冰4件套折合暴击率加成</h3>
            <el-input
                v-model="ice4Crit"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">双冰辅助产球 (填 0/1) </h3>
            <el-input
                v-model="doubleIceRecharge"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "shenlilinghuaDps.tcfg",
    data() {
        return {
            ice4Crit: "0.3",
            doubleIceRecharge: "1"
        }
    },
    methods: {
        compact() {
            return {
                ice4Crit: Math.max(0.0, parseFloat(this.ice4Crit) ?? 0.3),
                doubleIceRecharge: parseInt(this.doubleIceRecharge) ?? 1
            }
        },
        setData(d) {
            this.ice4Crit = d.ice4Crit.toString();
            this.doubleIceRecharge = d.doubleIceRecharge.toString();
        }
    }
    
}
</script>