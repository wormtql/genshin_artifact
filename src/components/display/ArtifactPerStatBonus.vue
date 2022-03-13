<template>
    <div class="root">
        <el-row :gutter="16">
            <el-col :span="16" style="height: 400px">
                <v-chart
                    :option="optionsForECharts"
                    :autoresize="true"
                ></v-chart>
            </el-col>
            <el-col :span="8" style="height: 400px">
                <v-chart
                    :option="optionsForPieChart"
                    :autoresize="true"
                ></v-chart>
            </el-col>
        </el-row>

    </div>
</template>

<script>
import { artifactTags } from "@const/artifact"

export default {
    name: "ArtifactPerStatBonus",
    props: ["data"],
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
                toolbox: {
                    feature: {
                        saveAsImage: {}
                    }
                },
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
.root {
    height: 400px;
    overflow: visible;
}
</style>