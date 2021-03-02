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
            <h3 class="config-title">平A附魔比例</h3>
            <el-input v-model="aEle"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击频率</h3>
            <el-input v-model="bFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击附魔比例</h3>
            <el-input v-model="bEle"></el-input>
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
    name: "KeqingNormalConfig",
    data: function () {
        return {
            thunderFreq: "0.9",
            aFreq: "0.4",
            bFreq: "0.4",
            eFreq: "0.15",
            qFreq: "0.05",
            aEle: "0.8",
            bEle: "0.8",
        }
    },
    methods: {
        compact() {
            let aFreq = parseFloat(this.aFreq);
            let bFreq = parseFloat(this.bFreq);
            let eFreq = parseFloat(this.eFreq);
            let qFreq = parseFloat(this.qFreq);

            let sum = aFreq + bFreq + eFreq + qFreq;
            if (sum === 0) {
                aFreq = 0.4;
                bFreq = 0.4;
                eFreq = 0.15;
                qFreq = 0.05
            } else {
                aFreq /= sum;
                bFreq /= sum;
                eFreq /= sum;
                qFreq /= sum;
            }

            return {
                thunderFreq: parseFloat(this.thunderFreq),

                aFreq,
                bFreq,
                eFreq,
                qFreq,

                aEle: parseFloat(this.aEle),
                bEle: parseFloat(this.bEle),
            }
        },
    }
}
</script>