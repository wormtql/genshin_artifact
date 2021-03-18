<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">E技能刹那之花命中次数</h3>
            <el-input-number v-model="value.eCount" :min="0"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q技能生灭之花命中次数</h3>
            <el-input-number v-model="value.qCount" :min="0"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">Q技能施放频率（占E、Q总次数的百分比）（请输入0-1的数值）</h3>
            <el-input v-model="value.qFreq"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">2命生灭计数（不足2命请忽略）</h3>
            <el-input-number v-model="value.c2Count" :min="0" :max="4"></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "AbeiduoDefault.tcfg",
    props: {
        value: {
            type: Object,
        }
    },

    first() {
        return {
            eCount: 4,
            qCount: 7,
            qFreq: "0.4",
            c2Count: 2,
        }
    },

    compact(obj) {
        let temp = Object.assign({}, obj);
        temp.qFreq = parseFloat(obj.qFreq) ?? 0.4;

        return temp;
    }
}
</script>