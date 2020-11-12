<template>
    <v-chart :options="options"></v-chart>
</template>

<script>
import ECharts from 'vue-echarts'
import 'echarts/lib/chart/line'
import 'echarts/lib/component/polar'

export default {
    name: "Deritive",
    components: {
        "v-chart": ECharts,
    },
    props: {
        data: {
            type: Array,
            default: () => [
                {
                    "name": "name",
                    "chs": "暴击率",
                    "d": 1000
                },
                {
                    "name": "name1",
                    "chs": "暴击伤害",
                    "d": 900,
                }
            ]
        }
    },
    computed: {
        options() {
            return {
                series: {
                    type: "pie",
                    data: this.data.map(item => ({
                        name: item.chs,
                        value: item.d
                    })),
                }
            }
        }
    }
}
</script>