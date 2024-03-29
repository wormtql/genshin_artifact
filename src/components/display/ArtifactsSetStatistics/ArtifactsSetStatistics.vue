<template>
    <div class="root">
        <div class="chart-div">
            <v-chart :option="chartOptionForEChart" :autoresize="true"></v-chart>
        </div>
        <div class="table-div">
            <el-table
                :data="tableDataForElementUI"
                @selection-change="selection = $event"
                ref="table"
            >
                <el-table-column
                    type="selection"
                    width="48"
                ></el-table-column>
                <el-table-column
                    :label="t('misc.stat')"
                >
                    <template #default="{ row }">
                        {{ row.title }}
                        <!--                    <el-checkbox>{{ row.chs }}</el-checkbox>-->
                    </template>
                </el-table-column>
                <el-table-column
                    :label="t('misc.value')"
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
                    :label="t('misc.stat1')"
                >
                    <template #default="scope">
                        {{ scope.row.eff.toFixed(3) }}
                    </template>
                </el-table-column>
                <el-table-column
                    :label="t('misc.rollCount')"
                >
                    <template #default="scope">
                        [{{ scope.row.upgradeCount[0] }}, {{ scope.row.upgradeCount[1] }}]
                    </template>
                </el-table-column>
            </el-table>
            <p style="font-size: 12px">{{ t("misc.selected") }}: {{ selectedEff.toFixed(1) }}</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { howManyUpgradeCount } from "@/utils/artifacts"
import { artifactTags, artifactEff } from "@/constants/artifact"
import {useArtifactStore} from "@/store/pinia/artifact"
import type {IArtifact} from "@/types/artifact"
import {useI18n} from "@/i18n/i18n"
/// #if !USE_CDN
import { use } from "echarts/core"
import { PieChart } from "echarts/charts"
import {
    TooltipComponent,
    VisualMapComponent,
    LegendComponent,
    TitleComponent,
} from "echarts/components"
import { CanvasRenderer } from "echarts/renderers"

use([
    CanvasRenderer,
    PieChart,
    VisualMapComponent,
    TooltipComponent,
    LegendComponent,
    TitleComponent,
])
/// #endif
import VChart from "vue-echarts"


const { t } = useI18n()

interface Props {
    artifactIds: number[]
}

const props = defineProps<Props>()


interface StatData {
    name: string,
    upgradeCount: [number, number],
    value: number,
    title: string,
    eff: number,
    percentage: boolean
}

const artifactStore = useArtifactStore()

const artifacts = computed((): IArtifact[] => {
    const arr = []
    for (let id of props.artifactIds) {
        let maybeArtifact = artifactStore.artifacts.value.get(id)
        if (maybeArtifact) {
            arr.push(maybeArtifact)
        }
    }

    return arr
})

const statData = computed((): Record<string, StatData> => {
    let result: Record<string, StatData> = {}

    for (let artifact of artifacts.value) {
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
                    // chs: artifactTags[name].chs,
                    title: t("stat", name),
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
})

const chartOptionForEChart = computed(() => {
    let data = [];
    for (let name in statData.value) {
        data.push({
            value: statData.value[name].eff,
            // name: statData.value[name].chs
            name: statData.value[name].title
        })
    }
    data.sort((a, b) => a.value - b.value)

    return {
        tooltip: {
            trigger: "item",
            formatter: (params: any) => {
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
})

const tableDataForElementUI = computed((): StatData[] => {
    let data = []

    for (let key in statData.value) {
        data.push(statData.value[key])
    }

    data.sort((a, b) => b.eff - a.eff)
    // data.sort((a, b) => a.chs.localeCompare(b.chs))

    return data
})

const selection = ref<StatData[]>([])

const selectedEff = computed(() => {
    let sum = 0
    for (let row of selection.value) {
        sum += row.eff
    }
    return sum
})


// export default {
//     methods: {
//         setSelection(tagName, value) {
//             const component = this.$refs["table"]
//             if (!component) {
//                 return
//             }
//             for (let row of this.tableDataForElementUI) {
//                 if (row.name === tagName) {
//                     component.toggleRowSelection(row, value)
//                     break
//                 }
//             }
//         }
//     },
//
//     computed: {
//
//
//
//
//
//     }
// }
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