<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">敌方火元素覆盖率</h3>
            <el-input
                v-model="fireFreq"
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
            <h3 class="config-title">平A附魔比例</h3>
            <el-input
                v-model="aEle"
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
            <h3 class="config-title">重击附魔比例</h3>
            <el-input
                v-model="bEle"
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

        <div class="config-item">
            <h3 class="config-title">蒸发占比（0-1）</h3>
            <el-input
                v-model="evaporate"
                size="small"
            ></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">融化占比（0-1）（请确保与蒸发占比之和小于1）</h3>
            <el-input
                v-model="melt"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "DilukeNormal.tcfg",
    data() {
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
    methods: {
        setData(d) {
            this.fireFreq = d.fireFreq.toString();
            this.aFreq = d.aFreq.toString();
            this.aEle = d.aEle.toString();
            this.bFreq = d.bFreq.toString();
            this.bEle = d.bEle.toString();
            this.eFreq = d.eFreq.toString();
            this.qFreq = d.qFreq.toString();

            this.evaporate = d.evaporate.toString();
            this.melt = d.melt.toString();
        },

        compact() {
            let fireFreq = parseFloat(this.fireFreq) ?? 0.3;
            let aFreq = parseFloat(this.aFreq) ?? 0.55;
            let bFreq = parseFloat(this.bFreq) ?? 0;
            let eFreq = parseFloat(this.eFreq) ?? 0.4;
            let qFreq = parseFloat(this.qFreq) ?? 0.05;

            let aEle = parseFloat(this.aEle) ?? 0.5;
            let bEle = parseFloat(this.bEle) ?? 0.5;

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

                evaporate: parseFloat(this.evaporate) ?? 0,
                melt: parseFloat(this.melt) ?? 0
            }
        },
    }
    
}
</script>