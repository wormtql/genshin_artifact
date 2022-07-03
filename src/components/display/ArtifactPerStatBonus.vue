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
/// #endif
import { artifactTags } from "@const/artifact"
import { deviceIsPC } from "@util/device"
import VChart from "vue-echarts"
import {useI18n} from "../../i18n/i18n";


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
                    // let chs = artifactTags[keysMap[key]].chs
                    const title = this.t("stat", keysMap[key])
                    data.push({
                        // name: chs,
                        name: title,
                        type: "line",
                        // data: parseFloat(this.data[key].toFixed(1))
                        data: this.data[key]
                    })
                    // legend.push(chs)
                    legend.push(title)
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
                    text: this.t("calcPage.bonus1"),
                    left: "center",
                },
                tooltip: {
                    trigger: "item"
                },
                series: [
                    {
                        name: this.t("calcPage.bonus1"),
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
                    name: this.t("calcPage.statCount"),
                    axisLabel: {
                        formatter: (value, index) => {
                            // return `${index + 1}${this.t('misc.stat')}`
                            return `${index + 1}`
                        }
                    }

                },
                yAxis: {
                    name: this.t("calcPage.gain"),
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
    },

    setup() {
        const { t } = useI18n()

        return {
            t
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