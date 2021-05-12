<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">计算模式</h3>
            <el-radio-group
                v-model="mode"
                size="small"
            >
                <el-radio-button label="expect">期望伤害</el-radio-button>
                <el-radio-button label="max">最大伤害</el-radio-button>
                <el-radio-button label="max-reaction">最大反应伤害</el-radio-button>
            </el-radio-group>
        </div>

        <div
            class="config-item"
            v-if="mode === 'max' || mode === 'max-reaction'"
        >
            <h3 class="config-title">技能类型</h3>
            <el-radio-group
                v-model="skill"
                size="small"
            >
                <el-radio-button label="a">平A</el-radio-button>
                <el-radio-button label="b">重击</el-radio-button>
            </el-radio-group>
        </div>

        <div
            class="config-item"
            v-if="mode === 'max' || mode === 'max-reaction'"
        >
            <h3 class="config-title">是否应用渡火4</h3>
            <el-switch
                v-model="lw4"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>

        <div
            class="config-item"
            v-if="mode === 'expect'"
        >
            <h3 class="config-title">敌方火元素覆盖率</h3>
            <el-input
                v-model="pyroRate"
                size="small"
            ></el-input>
        </div>

        <div
            class="config-item"
            v-if="mode === 'expect'"
        >
            <h3 class="config-title">蒸发占比（0-1）</h3>
            <el-input
                v-model="evaporate"
                size="small"
            ></el-input>
        </div>

        <div
            class="config-item"
            v-if="mode === 'expect'"
        >
            <h3 class="config-title">融化占比（0-1）（请确保与蒸发占比之和小于1）</h3>
            <el-input
                v-model="melt"
                size="small"
            ></el-input>
        </div>

        <div
            class="config-item"
            v-if="mode === 'expect'"
        >
            <h3 class="config-title">重击频率</h3>
            <el-input
                v-model="bFreq"
                size="small"
            ></el-input>
        </div>

        <div
            class="config-item"
            v-if="mode === 'expect'"
        >
            <h3 class="config-title">6命覆盖率（不足6命请忽略）</h3>
            <el-input
                v-model="conste6Rate"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "Hutao.tcfg",
    data() {
        return {
            pyroRate: "0.9",
            evaporate: "0.5",
            melt: "0",
            bFreq: "0.7",
            mode: "expect",
            lw4: false,
            skill: "a",
            conste6Rate: "0.05",
        }
    },
    methods: {
        compact() {
            return {
                pyroRate: parseFloat(this.pyroRate) ?? 0.9,
                evaporate: parseFloat(this.evaporate) ?? 0.5,
                melt: parseFloat(this.melt) ?? 0,
                bFreq: parseFloat(this.bFreq) ?? 0.7,
                mode: this.mode,
                lw4: this.lw4,
                skill: this.skill,
                conste6Rate: parseFloat(this.conste6Rate) ?? 0.05,
            };
        },
        setData(d) {
            this.pyroRate = d.pyroRate.toString();
            this.evaporate = d.evaporate.toString();
            this.melt = d.melt.toString();
            this.bFreq = d.bFreq.toString();
            this.mode = d.mode;
            this.lw4 = d.lw4;
            this.skill = d.skill;
            this.conste6Rate = d.conste6Rate ?? "0.05";
        }
    }
    
}
</script>