<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">计算模式</h3>
            <el-radio-group v-model="value.mode">
                <el-radio-button label="expect">期望伤害</el-radio-button>
                <el-radio-button label="max">最大伤害</el-radio-button>
                <el-radio-button label="max-reaction">最大反应伤害</el-radio-button>
            </el-radio-group>
        </div>

        <div class="config-item" v-if="value.mode === 'max' || value.mode === 'max-reaction'">
            <h3 class="config-title">技能类型</h3>
            <el-radio-group v-model="value.skill">
                <el-radio-button label="a">平A</el-radio-button>
                <el-radio-button label="b">重击</el-radio-button>
            </el-radio-group>
        </div>

        <div class="config-item">
            <h3 class="config-title">生命值低于50%</h3>
            <el-switch v-model="value.hpBelow50" active-text="是" inactive-text="否"></el-switch>
        </div>

        <div class="config-item" v-if="value.mode === 'max' || value.mode === 'max-reaction'">
            <h3 class="config-title">是否应用渡火4</h3>
            <el-switch v-model="value.lw4" active-text="是" inactive-text="否"></el-switch>
        </div>

        <div class="config-item" v-if="value.mode === 'expect'">
            <h3 class="config-title">敌方火元素覆盖率</h3>
            <el-input v-model="value.pyroRate"></el-input>
        </div>

        <div class="config-item" v-if="value.mode === 'expect'">
            <h3 class="config-title">蒸发占比（0-1）</h3>
            <el-input v-model="value.evaporate"></el-input>
        </div>

        <div class="config-item" v-if="value.mode === 'expect'">
            <h3 class="config-title">融化占比（0-1）（请确保与蒸发占比之和小于1）</h3>
            <el-input v-model="value.melt"></el-input>
        </div>

        <div class="config-item" v-if="value.mode === 'expect'">
            <h3 class="config-title">重击频率</h3>
            <el-input v-model="value.bFreq"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "Hutao.tcfg",
    props: ["value"],
    first() {
        return {
            hpBelow50: false,
            pyroRate: "0.9",
            evaporate: "0.5",
            melt: "0",
            bFreq: "0.7",
            mode: "expect",
            lw4: false,
            skill: "a",
        }
    },
    compact(obj) {
        return {
            hpBelow50: obj.hpBelow50,
            pyroRate: parseFloat(obj.pyroRate) ?? 0.9,
            evaporate: parseFloat(obj.evaporate) ?? 0.5,
            melt: parseFloat(obj.melt) ?? 0,
            bFreq: parseFloat(obj.bFreq) ?? 0.7,
            mode: obj.mode,
            lw4: obj.lw4,
            skill: obj.skill,
        };
    }
}
</script>