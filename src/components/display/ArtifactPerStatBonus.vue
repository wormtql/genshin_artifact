<template>
    <div class="root">
        <el-row :gutter="deviceIsPC ? 16 : 0">
            <el-col :md="16" :sm="24" class="line-chart">
                <v-chart
                    :option="optionsForECharts"
                    :autoresize="true"
                ></v-chart>
            </el-col>
            <el-col :md="8" :sm="24" class="pie-chart">
                <v-chart
                    :option="optionsForPieChart"
                    :autoresize="true"
                ></v-chart>
            </el-col>
        </el-row>

    </div>
</template>

<script>
/// #if !USE_CDN
import { use } from "echarts/core"
import { LineChart, PieChart } from "echarts/charts"
import {
    TooltipComponent,
    // VisualMapComponent,
    ToolboxComponent,
    LegendComponent,
    GridComponent,
    TitleComponent,
} from "echarts/components"
import { CanvasRenderer } from "echarts/renderers"

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
/// #else
import _e from "echarts"
/// #endif
import { artifactTags } from "@const/artifact"
import { deviceIsPC } from "@util/device"
import VChart from "vue-echarts"

export default {
    name: "ArtifactPerStatBonus",
    props: ["data"],
    components: {VChart},
    data() {
        return {
            deviceIsPC
        }
    },
    computed: {
        seriesAndLegend() {
            const keysMap = {
                "atk": "attackStatic",
                "atk_percentage": "attackPercentage",
                "def": "defendStatic",
                "def_percentage": "defendPercentage",
                "hp": "lifeStatic",
                "hp_percentage": "lifePercentage",
                "critical_rate": "critical",
                "critical_damage": "criticalDamage",
                "recharge": "recharge",
                "elemental_mastery": "elementalMastery"
            }

            // console.log(this.data)
            let data = []
            let legend = []
            for (let key in keysMap) {
                // console.log(key)
                if (this.data[key].length > 0) {
                    let chs = artifactTags[keysMap[key]].chs
                    data.push({
                        name: chs,
                        type: "line",
                        data: this.data[key]
                    })
                    legend.push(chs)
                }
            }

            // console.log(data)
            return {
                data, legend
            }
        },

        optionsForPieChart() {
            const { data } = this.seriesAndLegend
            let dataSingle = []

            for (let item of data) {
                dataSingle.push({
                    value: item.data[0],
                    name: item.name
                })
            }

            return {
                title: {
                    text: "1词条收益",
                    left: "center",
                },
                tooltip: {
                    trigger: "item"
                },
                series: [
                    {
                        name: "1词条收益",
                        type: "pie",
                        radius: "50%",
                        data: dataSingle
                    }
                ]
            }
        },

        optionsForECharts() {
            const { data, legend } = this.seriesAndLegend
            const option = {
                tooltip: {
                    trigger: 'axis'
                },
                toolbox: deviceIsPC ? {
                    feature: {
                        saveAsImage: {}
                    }
                } : {},
                legend: {
                    data: legend
                },
                xAxis: {
                    type: "category",
                    // min: 1,
                    name: "词条数",
                    axisLabel: {
                        formatter: (value, index) => {
                            return `${index + 1}词条`
                        }
                    }

                },
                yAxis: {
                    name: "相对提升幅度",
                    axisLabel: {
                        formatter: (value, index) => {
                            return `${(value * 100).toFixed(1)}%`
                        }
                    }
                },
                series: data,
            }
            // console.log(option)
            return option
        }
    }
}
</script>

<style scoped lang="scss">
canvas {
    overflow: visible;
}
@media (min-width: 992px) {
    .root {
        height: 400px;
        overflow: visible;
    }

    .line-chart {
        height: 400px
    }

    .pie-chart {
        height: 400px;
    }
}

@media (max-width: 992px) {
    .root {
        overflow: visible;
    }
    .line-chart {
        height: 300px;
    }
    .pie-chart {
        height: 300px;
    }
}

</style>