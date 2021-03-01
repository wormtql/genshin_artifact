<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">冰4件套折合暴击率加成</h3>
            <el-input v-model="ice4Crit"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">天赋“唯此一心”折合暴击率加成</h3>
            <el-input v-model="talent1Crit"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">天赋“天地交泰”折合冰伤加成</h3>
            <el-input v-model="talent2Bonus"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">元素共鸣-双冰</h3>
            <el-switch v-model="ele2I" active-text="是" inactive-text="否"></el-switch>
            <p>双冰折合暴击率</p>
            <el-input v-model="ele2ICrit" :disabled="!ele2I"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "GanyuConfig",
    data: function () {
        return {
            ice4Crit: "0.2",
            talent1Crit: "0.2",
            talent2Bonus: "0.2",
            ele2I: false,
            ele2ICrit: "0.1",
        }
    },
    methods: {
        compact: function () {
            return {
                ice4Crit: parseFloat(this.ice4Crit) || 0.2,
                talent1Crit: parseFloat(this.talent1Crit) || 0.2,
                talent2Bonus: parseFloat(this.talent2Bonus) || 0.2,
                ele2I: this.ele2I,
                ele2ICrit: parseFloat(this.ele2ICrit) || 0.1,
            }
        }
    }
}
</script>