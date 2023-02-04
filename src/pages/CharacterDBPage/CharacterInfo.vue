<template>
    <div v-loading="!loaded">
        <el-empty v-if="!loaded"></el-empty>
        <div v-if="loaded && error">
            Error
        </div>
        <div v-if="loaded && !error && !hasCharacter">
            <el-empty :description="t('dbPage.tooSmall')"></el-empty>
        </div>
        <div v-if="loaded && !error && hasCharacter" class="tab-full-height mona-scroll-hidden content-div">
            <p class="analysis-item-title">{{ t("a", characterNameLocaleIndex) }}</p>
            <div class="character-splash-div">
                <img :src="characterSplash">
            </div>

            <p class="analysis-item-title">{{ t("dbPage.weapon") }}</p>
            <w-c-bar
                v-for="item in weaponUsage"
                :key="item[0]"
                :item="item"
                type="weapon"
            ></w-c-bar>

            <p class="analysis-item-title">{{ t("dbPage.art") }}</p>
            <div
                v-for="(item, index) in artifactSetUsage"
                :key="index"
                class="bar-item-artifact"
            >
                <artifact-bar :item="item"></artifact-bar>
            </div>

            <p class="analysis-item-title">{{ t("dbPage.mainStat") }}</p>
            <el-row>
                <el-col
                    :md="8"
                    :sm="24"
                    v-for="slotName in mainStats"
                >
                    <basic-pie-chart
                        :title="t('misc', slotName)"
                        :data="mainStatData[slotName]"
                    ></basic-pie-chart>
                </el-col>
            </el-row>

            <p class="analysis-item-title">{{ t("dbPage.subStat") }}</p>
            <v-chart :option="optionsForSubStatChart" style="height: 300px" :autoresize="true"></v-chart>
        </div>
    </div>
</template>

<script>
import {characterByElement, characterData} from "@character"
import {getComputeResultAnalysis} from "@/api/misc"

import ArtifactBar from "./ArtifactBar"
import WCBar from "./WCBar"
import BasicPieChart from "@/components/display/BasicPieChart"
import {statName2Locale} from "@util/artifacts"

/// #if !USE_CDN
import { use } from "echarts/core"
import { BarChart } from "echarts/charts"
import {
    TooltipComponent,
    GridComponent,
} from "echarts/components"
import { CanvasRenderer } from "echarts/renderers"

use([
    CanvasRenderer,
    BarChart,
    TooltipComponent,
    GridComponent,
])
/// #endif
import VChart from "vue-echarts"
import {useI18n} from "../../i18n/i18n";

export default {
    name: "MonaDBCharacter",
    components: {
        ArtifactBar,
        WCBar,
        BasicPieChart,
        VChart,
    },
    data() {
        return {
            analysisResult: null,
            characterName: "Amber",

            loaded: false,
            error: false,

            characterByElement,
            characterData,

            mainStats: ["Sand", "Goblet", "Head"],
            // slot2Chs: {
            //     "Sand": "时之沙",
            //     "Goblet": "空之杯",
            //     "Head": "理之冠"
            // }
        }
    },
    mounted() {
        this.characterName = this.$route.params.name ?? "Amber"
        this.refresh(this.characterName)
    },
    methods: {
        refresh(name) {
            this.loaded = false
            this.characterName = name

            getComputeResultAnalysis().then(result => {
                this.analysisResult = result
                this.error = false
            }).catch(e => {
                console.log(e)
                this.error = true
            }).finally(() => {
                this.loaded = true
            })
        },
    },
    computed: {
        characterNameLocaleIndex() {
            return characterData[this.characterName].nameLocale
        },

        hasCharacter() {
            return this.characterResult && this.characterResult[this.characterName]
        },

        characterSplash() {
            const data = characterData[this.characterName]
            if (!data) {
                return ""
            }

            return data.splash
        },

        characterResult() {
            if (this.analysisResult) {
                return this.analysisResult.character_result
            }

            return null
        },

        weaponUsage() {
            if (!this.characterResult) {
                return []
            }

            let temp = []
            for (const item of this.characterResult[this.characterName].weapon_usage) {
                if (item[1] >= 0.01) {
                    temp.push(item)
                }
            }
            return temp
        },

        artifactSetUsage() {
            if (!this.characterResult) {
                return []
            }

            let temp = []
            for (const item of this.characterResult[this.characterName].artifact_set_usage) {
                if (item[1] >= 0.01) {
                    temp.push(item)
                }
            }
            return temp
        },

        mainStatUsage() {
            if (this.characterResult) {
                return this.characterResult[this.characterName].main_stat_usage
            }
            return null
        },

        mainStatData() {
            let result = {}
            if (this.mainStatUsage) {
                for (const slot of this.mainStats) {
                    result[slot] = []
                    for (const statName in this.mainStatUsage[slot]) {
                        const value = this.mainStatUsage[slot][statName]
                        // const native = statName2Chs(statName)

                        if (value > 0.01) {
                            const title = statName2Locale(statName)

                            result[slot].push({
                                value,
                                // name: native
                                name: title,
                            })
                        }
                    }
                }
            }

            return result
        },

        subStatUsage() {
            if (this.characterResult) {
                return this.characterResult[this.characterName].artifact_sub_stat_statistics
            }
            return null
        },

        subStatLabelAndData() {
            if (!this.subStatUsage) {
                return [[], []]
            }

            let temp = []
            for (const statName in this.subStatUsage) {
                const locale = statName2Locale(statName)
                const value = parseFloat(this.subStatUsage[statName].toFixed(2))
                temp.push([locale, value])
            }

            temp.sort((a, b) => b[1] - a[1])
            // temp = temp.filter(a => a[1] > 0.01)

            const labels = temp.map(x => x[0])
            const data = temp.map(x => x[1])

            return [labels, data]
        },

        optionsForSubStatChart() {
            return {
                xAxis: {
                    type: 'category',
                    data: this.subStatLabelAndData[0],
                    axisLabel: {
                        rotate: 60
                    }
                },
                yAxis: {
                    type: 'value',
                    name: this.t("dbPage.count")
                },
                tooltip: {
                    trigger: "item"
                },
                series: [
                    {
                        data: this.subStatLabelAndData[1],
                        type: 'bar'
                    }
                ]
            }
        }
    },
    beforeRouteUpdate(to, f ,next) {
        // console.log(to.params.name)
        this.refresh(to.params.name)
        next()
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
.analysis-item-title {
    font-size: 25px;
    font-weight: bold;
    color: #525252;
    position: relative;

    &::before {
        content: "#"
    }
}

.character-splash-div {
    img {
        width: min(400px, 100%);
    }
}
</style>