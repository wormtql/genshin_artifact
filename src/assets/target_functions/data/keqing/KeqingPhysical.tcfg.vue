<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">敌方雷元素覆盖率</h3>
            <el-input
                v-model="thunderFreq"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">平A频率</h3>
            <el-input
                v-model="aFreq"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击频率</h3>
            <el-input
                v-model="bFreq"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">E频率</h3>
            <el-input
                v-model="eFreq"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q频率</h3>
            <el-input
                v-model="qFreq"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "KeqingPhysical.tcfg",
    data() {
        return {
            thunderFreq: "0.5",
            aFreq: "0.3",
            bFreq: "0.5",
            eFreq: "0.15",
            qFreq: "0.05",
        }
    },
    methods: {
        setData(d) {
            this.thunderFreq = d.thunderFreq.toString();
            this.aFreq = d.aFreq.toString();
            this.bFreq = d.bFreq.toString();
            this.eFreq = d.eFreq.toString();
            this.qFreq = d.qFreq.toString();
        },
        compact() {
            let aFreq = parseFloat(this.aFreq);
            let bFreq = parseFloat(this.bFreq);
            let eFreq = parseFloat(this.eFreq);
            let qFreq = parseFloat(this.qFreq);

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
                thunderFreq: parseFloat(this.thunderFreq) ?? 0.5,

                aFreq,
                bFreq,
                eFreq,
                qFreq,
            }
        },
    }
    
}
</script>