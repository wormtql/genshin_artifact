<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">E技能刹那之花命中次数</h3>
            <el-input-number v-model="eCount" :min="0"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q技能生灭之花命中次数</h3>
            <el-input-number v-model="qCount" :min="0"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q技能施放频率（占E、Q总次数的百分比）（请输入0-1的数值）</h3>
            <el-input v-model="qFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">2命生灭计数（不足2命请忽略）</h3>
            <el-input-number v-model="c2Count" :min="0" :max="4"></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "AbeiduoDefaultConfig",
    data: function () {
        return {
            eCount: 4,
            qCount: 7,
            qFreq: "0.4",
            c2Count: 2,
        }
    },
    methods: {
        compact() {
            let freq = parseFloat(this.qFreq) || 0.4;

            return {
                eCount: this.eCount,
                qCount: this.qCount,
                qFreq: freq,
            };
        }
    }
}
</script>