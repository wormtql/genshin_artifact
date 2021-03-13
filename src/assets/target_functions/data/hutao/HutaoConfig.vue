<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">计算模式</h3>
            <el-radio-group v-model="mode">
                <el-radio-button label="expect">期望伤害</el-radio-button>
                <el-radio-button label="max">最大平A伤害</el-radio-button>
                <el-radio-button label="max-reaction">最大平A反应伤害</el-radio-button>
            </el-radio-group>
        </div>

        <div class="config-item">
            <h3 class="config-title">生命值低于50%</h3>
            <el-switch v-model="hpBelow50" active-text="是" inactive-text="否"></el-switch>
        </div>

        <div class="config-item" v-if="mode === 'max' || mode === 'max-reaction'">
            <h3 class="config-title">是否应用渡火4</h3>
            <el-switch v-model="lw4" active-text="是" inactive-text="否"></el-switch>
        </div>

        <div class="config-item" v-if="mode === 'expect'">
            <h3 class="config-title">敌方火元素覆盖率</h3>
            <el-input v-model="pyroRate"></el-input>
        </div>

        <div class="config-item" v-if="mode === 'expect'">
            <h3 class="config-title">蒸发占比（0-1）</h3>
            <el-input v-model="evaporate"></el-input>
        </div>

        <div class="config-item" v-if="mode === 'expect'">
            <h3 class="config-title">融化占比（0-1）（请确保与蒸发占比之和小于1）</h3>
            <el-input v-model="melt"></el-input>
        </div>

        <div class="config-item" v-if="mode === 'expect'">
            <h3 class="config-title">重击频率</h3>
            <el-input v-model="bFreq"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "HutaoConfig",
    data: function () {
        return {
            hpBelow50: false,
            pyroRate: "0.9",
            evaporate: "0.5",
            melt: "0",
            bFreq: "0.7",
            mode: "expect",
            lw4: false,
        }
    },
    methods: {
        compact() {
            return {
                hpBelow50: this.hpBelow50,
                pyroRate: parseFloat(this.pyroRate),
                evaporate: parseFloat(this.evaporate),
                melt: parseFloat(this.melt),
                bFreq: parseFloat(this.bFreq),
                mode: this.mode,
            };
        }
    }
}
</script>