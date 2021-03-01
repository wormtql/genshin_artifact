<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">敌方雷元素覆盖率</h3>
            <el-input v-model="thunderFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">平A频率</h3>
            <el-input v-model="aFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击频率</h3>
            <el-input v-model="bFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">E频率</h3>
            <el-input v-model="eFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q频率</h3>
            <el-input v-model="qFreq"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "KeqingPhysicalConfig",
    data: function () {
        return {
            thunderFreq: "0.5",
            aFreq: "0.3",
            bFreq: "0.5",
            eFreq: "0.15",
            qFreq: "0.05",
        }
    },
    methods: {
        compact() {
            let aFreq = parseFloat(this.aFreq) || 0.3;
            let bFreq = parseFloat(this.bFreq) || 0.5;
            let eFreq = parseFloat(this.eFreq) || 0.15;
            let qFreq = parseFloat(this.qFreq) || 0.05;

            let sum = aFreq + bFreq + eFreq + qFreq;
            if (sum === 0) {
                aFreq = 0.3;
                bFreq = 0.5;
                eFreq = 0.15;
                qFreq = 0.05
            } else {
                aFreq /= sum;
                bFreq /= sum;
                eFreq /= sum;
                qFreq /= sum;
            }

            return {
                thunderFreq: parseFloat(this.thunderFreq) || 0.5,

                aFreq,
                bFreq,
                eFreq,
                qFreq,
            }
        },
    }
}
</script>