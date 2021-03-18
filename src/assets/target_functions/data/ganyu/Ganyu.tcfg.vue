<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">冰4件套折合暴击率加成</h3>
            <el-input v-model="value.ice4Crit"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">天赋“唯此一心”折合暴击率加成</h3>
            <el-input v-model="value.talent1Crit"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">天赋“天地交泰”折合冰伤加成</h3>
            <el-input v-model="value.talent2Bonus"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">元素共鸣-双冰</h3>
            <el-switch v-model="value.ele2I" active-text="是" inactive-text="否"></el-switch>
            <p>双冰折合暴击率</p>
            <el-input v-model="value.ele2ICrit" :disabled="!value.ele2I"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "Ganyu.tcfg",
    props: ["value"],
    first() {
        return {
            ice4Crit: "0.2",
            talent1Crit: "0.2",
            talent2Bonus: "0.2",
            ele2I: false,
            ele2ICrit: "0.1",
        }
    },
    compact(obj) {
        return {
            ice4Crit: parseFloat(obj.ice4Crit) ?? 0.2,
            talent1Crit: parseFloat(obj.talent1Crit) ?? 0.2,
            talent2Bonus: parseFloat(obj.talent2Bonus) ?? 0.2,
            ele2I: obj.ele2I,
            ele2ICrit: parseFloat(obj.ele2ICrit) ?? 0.1,
        }
    }
}
</script>