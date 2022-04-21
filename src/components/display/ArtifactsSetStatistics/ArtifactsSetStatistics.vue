<template>
    <div class="root">
        <div class="chart-div">
            <v-chart :option="chartOptionForEChart" :autoresize="true"></v-chart>
        </div>
        <div class="table-div">
            <el-table
                :data="tableDataForElementUI"
                size="mini"
                @selection-change="selection = $event"
                ref="table"
            >
                <el-table-column
                    type="selection"
                    width="48"
                ></el-table-column>
                <el-table-column
                    label="词条"
                >
                    <template v-slot="{ row }">
                        {{ row.chs }}
                        <!--                    <el-checkbox>{{ row.chs }}</el-checkbox>-->
                    </template>
                </el-table-column>
                <el-table-column
                    label="值"
                >
                    <template #default="{ row }">
                        <template v-if="row.percentage">
                            {{ (row.value * 100).toFixed(1) }}%
                        </template>
                        <template v-else>
                            {{ row.value.toFixed(0) }}
                        </template>
                    </template>
                </el-table-column>
                <el-table-column
                    label="有效词条数"
                >
                    <template #default="scope">
                        {{ scope.row.eff.toFixed(3) }}
                    </template>
                </el-table-column>
                <el-table-column
                    label="强化次数"
                >
                    <template slot-scope="scope">
                        [{{ scope.row.upgradeCount[0] }}, {{ scope.row.upgradeCount[1] }}]
                    </template>
                </el-table-column>
            </el-table>
            <p style="font-size: 12px">选中：{{ selectedEff.toFixed(1) }}</p>
        </div>
    </div>
</template>

<script>
import { howManyUpgradeCount } from "@util/artifacts"
import { artifactTags, artifactEff } from "@const/artifact"

export default {
    name: "ArtifactsSetStatistics",
    components: {
    },
    props: ["artifactIds"],
    // mounted() {
    //     this.setSelection("critical", true)
    //     this.setSelection("criticalDamage", true)
    //     this.setSelection("attackPercentage", true)
    // },
    data() {
        return {
            selection: []
        }
    },
    methods: {
        setSelection(tagName, value) {
            const component = this.$refs["table"]
            if (!component) {
                return
            }
            for (let row of this.tableDataForElementUI) {
                if (row.name === tagName) {
                    component.toggleRowSelection(row, value)
                    break
                }
            }
        }
    },

    computed: {
        artifacts() {
            const artifactsById = this.$store.getters["artifacts/artifactsById"]
            let arr = []
            for (let id of this.artifactIds) {
                let maybeArtifact = artifactsById[id]
                if (maybeArtifact) {
                    arr.push(maybeArtifact)
                }
            }

            return arr
        },

        data() {
            let result = {}

            if (!this.artifacts) {
                return {}
            }

            for (let artifact of this.artifacts) {
                const star = artifact.star ?? 5
                if (star <= 3) {
                    continue
                }

                for (let stat of artifact.normalTags) {
                    const name = stat.name
                    const value = stat.value

                    const [min, max] = howManyUpgradeCount(value, name, star)
                    const eff = value / artifactEff[star][name][3]

                    if (!Object.prototype.hasOwnProperty.call(result, name)) {
                        result[name] = {
                            name,
                            upgradeCount: [min, max],
                            value,
                            chs: artifactTags[name].chs,
                            eff,
                            percentage: artifactTags[name].percentage
                        }
                    } else {
                        let temp = result[name]
                        temp.upgradeCount[0] += min
                        temp.upgradeCount[1] += max
                        temp.value += value
                        temp.eff += eff
                    }
                }
            }

            return result
        },

        selectedEff() {
            let sum = 0
            for (let row of this.selection) {
                sum += row.eff
            }
            return sum
        },

        tableDataForElementUI() {
            let data = []

            for (let key in this.data) {
                data.push(this.data[key])
            }

            data.sort((a, b) => b.eff - a.eff)
            // data.sort((a, b) => a.chs.localeCompare(b.chs))

            return data
        },

        chartOptionForEChart() {
            // const sumOfEff = Object.values(this.data).reduce((a, b) => a + b.eff, 0)

            let data = [];
            for (let name in this.data) {
                data.push({
                    value: this.data[name].eff,
                    name: this.data[name].chs
                })
            }
            data.sort((a, b) => a.value - b.value)
            // console.log(sumOfEff)

            return {
                tooltip: {
                    trigger: "item",
                    formatter: (params) => {
                        // console.log(params)
                        const value = params.value
                        const p = params.percent

                        return `${value.toFixed(1)}/${p.toFixed(0)}%`
                    },
                    // backgroundColor: 'rgb(236, 245, 255)'
                },
                visualMap: {
                    show: false,
                    min: 0,
                    max: 25,
                    inRange: {
                        colorLightness: [0.3, 1]
                    }
                },
                // roseType: 'area',
                // radius: "55%",
                // legend: {
                //     orient: "vertical",
                //     left: "left",
                // },
                series: [
                    {
                        name: "有效词条分布",
                        type: "pie",
                        // radius: ["40%", "70%"],
                        label: {
                            // show: false
                        },
                        // itemStyle: {
                        //     borderRadius: 5,
                        //     borderColor: '#fff',
                        //     borderWidth: 2
                        // },
                        itemStyle: {
                            color: '#c23531',
                            // shadowBlur: 200,
                            // shadowColor: 'rgba(0, 0, 0, 0.5)'
                        },
                        data,
                    }
                ]
            }
        },
    }
}
</script>

<style scoped lang="scss">
@media only screen and (min-width: 992px) {
    .root {
        display: flex;
        gap: 20px;

        .table-div {
            flex-grow: 1;
        }

        .chart-div {
            width: 400px;
            height: 300px;
        }
    }
}

@media only screen and (max-width: 992px) {
    .root {
        .table-div {
            //flex-grow: 1;
        }

        .chart-div {
            width: 100%;
            height: 200px;
        }
    }
}

</style>