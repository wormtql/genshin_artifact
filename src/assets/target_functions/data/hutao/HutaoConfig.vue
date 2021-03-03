<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">生命值低于50%</h3>
            <el-switch v-model="hpBelow50" active-text="是" inactive-text="否"></el-switch>
        </div>

        <div class="config-item">
            <h3 class="config-title">敌方火元素覆盖率</h3>
            <el-input v-model="pyroRate"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">蒸发占比（0-1）</h3>
            <el-input v-model="evaporate"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">融化占比（0-1）（请确保与蒸发占比之和小于1）</h3>
            <el-input v-model="melt"></el-input>
        </div>

        <div class="config-item">
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
            };
        }
    }
}
</script>