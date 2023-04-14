<template>
    <div class="root">
        <div class="pie-chart">
            <v-chart :option="optionsForPieChart" :autoresize="true"></v-chart>
        </div>
        <div class="line-chart" :style="{ height: data.length * 40 + 130 + 'px' }">
            <v-chart :option="optionsForECharts" :autoresize="true"></v-chart>
        </div>
    </div>
</template>

<script>
import { useI18n } from "@/i18n/i18n";
import { use } from "echarts/core"
import { LineChart, PieChart } from "echarts/charts"
import {
    TooltipComponent,
    ToolboxComponent,
    LegendComponent,
    GridComponent,
    TitleComponent,
} from "echarts/components"
import { CanvasRenderer } from "echarts/renderers"
import { deviceIsPC } from "@util/device"
import VChart from "vue-echarts"

use([
    CanvasRenderer,
    LineChart,
    PieChart,
    TooltipComponent,
    ToolboxComponent,
    LegendComponent,
    TitleComponent,
    GridComponent
])

export default defineComponent({
    name: "BuffGraph",
    props: ["data"],
    components: { VChart },
    computed: {
        optionsForPieChart() {
            let dataSingle = []
            let factor = 1
            let ln = 0
            for (let i of this.data.slice(1)) {
                factor *= (i.data + 100) / 100
                ln += Math.log((i.data + 100) / 100)
            }
            let ratio = (factor - 1) / factor
            dataSingle.push({
                value: 1 / factor * 100,
                name: this.data[0].name
            })
            for (let i of this.data.slice(1)) {
                dataSingle.push({
                    value: Math.log((i.data + 100) / 100) / ln * ratio * 100,
                    name: i.name
                })
            }
            return {
                title: {
                    text: "伤害占比",
                    left: "center",
                },
                tooltip: {
                    trigger: "item",
                    formatter: (params) => {
                        return `${params.name} ${params.value.toFixed(2)}%`
                    }
                },
                series: [
                    {
                        type: "pie",
                        radius: "60%",
                        center: ["50%", "50%"],
                        data: dataSingle,
                    }
                ]
            }
        },

        optionsForECharts() {
            const option = {
                title: {
                    text: "等效独立增伤",
                    left: "center",
                },
                tooltip: {
                    trigger: 'axis',
                    formatter: (params) => {
                        return `${params[0].name} ${params[0].value.toFixed(2)}%`
                    }
                },
                toolbox: deviceIsPC ? {
                    feature: {
                        saveAsImage: {}
                    }
                } : {},
                yAxis: {
                    type: "category",
                    data: this.data.map(i => i.name),
                    show: false,
                },
                xAxis: {
                    axisLabel: {
                        formatter: (value, index) => {
                            return `${value.toFixed(0)}%`
                        }
                    }
                },
                series: {
                    data: this.data.map(i => i.data),
                    type: 'bar',
                    itemStyle: {
                        color: function (params) {
                            const colors = ['#5470c6', '#91cc75', '#fac858', '#ee6666', '#73c0de', '#3ba272', '#fc8452', '#9a60b4', '#ff88e0'];
                            return colors[params.dataIndex % colors.length];
                        }
                    }
                },
            }
            return option
        }
    },
    data() {
        return {
            deviceIsPC,
        }
    },
    methods: {

    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
})
</script>
    
<style scoped lang="scss">
canvas {
    overflow: visible;
}

.root {
    overflow: visible;
    height: fit-content;
}

.line-chart {
    width: 100%;
}

.pie-chart {
    width: 100%;
    height: 400px;
}
</style>