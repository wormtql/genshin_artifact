<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">敌方火元素覆盖率</h3>
            <el-input v-model="value.fireFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">平A频率</h3>
            <el-input v-model="value.aFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">平A附魔比例</h3>
            <el-input v-model="value.aEle"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击频率</h3>
            <el-input v-model="value.bFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">重击附魔比例</h3>
            <el-input v-model="value.bEle"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">E频率</h3>
            <el-input v-model="value.eFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q频率</h3>
            <el-input v-model="value.qFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">蒸发占比（0-1）</h3>
            <el-input v-model="value.evaporate"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">融化占比（0-1）（请确保与蒸发占比之和小于1）</h3>
            <el-input v-model="value.melt"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "DilukeNormal.tcfg",
    props: ["value"],
    first() {
        return {
            fireFreq: "0.3",
            aFreq: "0.55",
            aEle: "0.5",
            bFreq: "0",
            bEle: "0.5",
            eFreq: "0.4",
            qFreq: "0.05",

            evaporate: "0",
            melt: "0",
        }
    },
    compact(obj) {
        let fireFreq = parseFloat(obj.fireFreq) ?? 0.3;
        let aFreq = parseFloat(obj.aFreq) ?? 0.55;
        let bFreq = parseFloat(obj.bFreq) ?? 0;
        let eFreq = parseFloat(obj.eFreq) ?? 0.4;
        let qFreq = parseFloat(obj.qFreq) ?? 0.05;

        let aEle = parseFloat(obj.aEle) ?? 0.5;
        let bEle = parseFloat(obj.bEle) ?? 0.5;

        let sum = aFreq + bFreq + eFreq + qFreq;
        if (sum === 0) {
            aFreq = 0.55
            bFreq = 0.0;
            eFreq = 0.4;
            qFreq = 0.05
        } else {
            aFreq /= sum;
            bFreq /= sum;
            eFreq /= sum;
            qFreq /= sum;
        }

        return {
            fireFreq,

            aFreq,
            aEle,
            bFreq,
            bEle,
            eFreq,
            qFreq,

            evaporate: parseFloat(obj.evaporate) ?? 0,
            melt: parseFloat(obj.melt) ?? 0
        }
    },
}
</script>