<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">敌方雷元素覆盖率</h3>
            <el-input v-model="value.thunderFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">平A频率</h3>
            <el-input v-model="value.aFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击频率</h3>
            <el-input v-model="value.bFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">E频率</h3>
            <el-input v-model="value.eFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q频率</h3>
            <el-input v-model="value.qFreq"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "KeqingPhysical.tcfg",
    props: ["value"],
    first() {
        return {
            thunderFreq: "0.5",
            aFreq: "0.3",
            bFreq: "0.5",
            eFreq: "0.15",
            qFreq: "0.05",
        }
    },
    compact(obj) {
        let aFreq = parseFloat(obj.aFreq);
        let bFreq = parseFloat(obj.bFreq);
        let eFreq = parseFloat(obj.eFreq);
        let qFreq = parseFloat(obj.qFreq);

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
            thunderFreq: parseFloat(obj.thunderFreq) ?? 0.5,

            aFreq,
            bFreq,
            eFreq,
            qFreq,
        }
    },
}
</script>