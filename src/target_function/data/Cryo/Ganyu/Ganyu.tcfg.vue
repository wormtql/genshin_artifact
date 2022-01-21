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
            <h3 class="config-title">天赋“唯此一心”折合暴击率加成</h3>
            <el-input
                v-model="talent1Crit"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">天赋“天地交泰”折合冰伤加成</h3>
            <el-input
                v-model="talent2Bonus"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "Ganyu.tcfg",
    data() {
        return {
            ice4Crit: "0.2",
            talent1Crit: "0.2",
            talent2Bonus: "0.2",
        }
    },
    methods: {
        compact() {
            return {
                ice4Crit: parseFloat(this.ice4Crit) ?? 0.2,
                talent1Crit: parseFloat(this.talent1Crit) ?? 0.2,
                talent2Bonus: parseFloat(this.talent2Bonus) ?? 0.2,
            }
        }
    }
    
}
</script>