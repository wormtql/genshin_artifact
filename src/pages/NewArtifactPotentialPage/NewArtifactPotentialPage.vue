<template>
    <div class="root">
        <el-row>
            <el-col
                :md="6"
                :sm="24"
                class="mona-scroll-hidden left"
            >
                <select-potential-function-name v-model="potentialFunctionName"></select-potential-function-name>

                <div class="pf-detail">
                    <img :src="pfImage" class="pf-image" alt="图" >

                    <div>
                        <p class="pf-title">{{ pfChs }}</p>
                        <p class="pf-description">{{ pfDescription }}</p>
                    </div>
                </div>

                <div class="pf-config">
                    <item-config
                        v-model="potentialFunctionConfig"
                        :item-name="potentialFunctionName"
                        :configs="pfConfigConfigs"
                    ></item-config>
                </div>
            </el-col>
            <el-col
                :md="18"
                :sm="24"
                class="col-right mona-scroll-hidden"
            >
                <div>
                    <el-button
                        type="primary"
                        :icon="IconEpCpu"
                        @click="handleClickStart"
                    >开始计算</el-button>
                </div>

                <div class="filter">
                    <span class="filter-item">
                        <span class="filter-title">位置</span>
                        <select-artifact-slot
                            v-model="filterSlots"
                            :multiple="true"
                        ></select-artifact-slot>
                    </span>

                    <span class="filter-item">
                        <span class="filter-title">套装</span>
                        <select-artifact-set
                            v-model="filterSetNames"
                            :multiple="true"
                            :multiple-limit="5"
                        ></select-artifact-set>
                    </span>

                    <span class="filter-item">
                        <span class="filter-title">主词条</span>
                        <select-artifact-main-stat
                            v-model="filterMainStatNames"
                            :multiple="true"
                            :include-any="false"
                        ></select-artifact-main-stat>
                    </span>
                </div>

                <div style="margin: 0 0 20px 0">
                    <span class="filter-title">等级</span>
                    <div class="filter-level">
                        <el-input-number
                            :min="0"
                            :max="filterLevel[1]"
                            v-model="filterLevel[0]"
                        ></el-input-number>
                        ~
                        <el-input-number
                            :min="filterLevel[0]"
                            :max="20"
                            v-model="filterLevel[1]"
                        ></el-input-number>
                    </div>
                </div>

                <div class="artifacts-div">
<!--                    <artifact-display-by-id-->
<!--                        v-for="result in resultsToBeDisplay"-->
<!--                        :artifact-id="result[0]"-->
<!--                        :extra="result[1].toFixed(2)"-->
<!--                        :show-back="true"-->
<!--                        :back-value="result[1] / results[0][1]"-->
<!--                    ></artifact-display-by-id>-->
                    <artifact-display
                        v-for="result in artifactsToBeDisplayed"
                        :item="result[0]"
                        :extra="result[1].toFixed(2)"
                        :show-back="true"
                        :back-value="result[1] / maxScore"
                    ></artifact-display>
                </div>

                <div class="pager">
                    <el-pagination
                        :total="filteredResults.length"
                        :page-size="20"
                        v-model:current-page="currentPage"
                        hide-on-single-page
                        layout="prev, pager, next"
                        :small="!deviceIsPC"
                    ></el-pagination>
                </div>

            </el-col>
        </el-row>
    </div>
</template>

<script setup lang="ts">
import {potentialFunctionData} from "@potentialFunction"
import {getPotentialFunctionDefaultConfig} from "@util/potentialFunction"
import {getArtifactsWasm} from "@/utils/artifacts"
import {wasmComputeArtifactPotential} from "@wasm"
import {deviceIsPC} from "@/utils/device"

import SelectPotentialFunctionName from "@c/select/SelectPotentialFunctionName"
import SelectArtifactSlot from "@c/select/SelectArtifactSlot"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"
import ItemConfig from "@c/config/ItemConfig"
import ArtifactDisplay from "@/components/display/ArtifactDisplay.vue"
import type {ArtifactPosition, ArtifactSetName, ArtifactStatName, IArtifact} from "@/types/artifact"
import {useArtifactStore} from "@/store/pinia/artifact"
// import {ElLoading, ElMessage} from "element-plus"
// import "element-plus/es/components/message/style/css"
import IconEpCpu from "~icons/ep/cpu"


const artifactStore = useArtifactStore()

// potential function name and corresponding data
const potentialFunctionName = ref("ArtifactEff")

const pfImage = computed(() => {
    const item = potentialFunctionData[potentialFunctionName.value]
    return item?.badge
})

const pfChs = computed(() => {
    const item = potentialFunctionData[potentialFunctionName.value]
    return item?.chs
})

const pfDescription = computed(() => {
    const item = potentialFunctionData[potentialFunctionName.value]
    return item?.description
})

const pfConfigConfigs = computed(() => {
    const item = potentialFunctionData[potentialFunctionName.value]
    if (item) {
        return item.config
    } else {
        return []
    }
})

// potential function config
const potentialFunctionConfig = ref(getPotentialFunctionDefaultConfig("ArtifactEff"))

// results
const results = ref<[number, number][]>([])     // [id, score][]

const maxScore = computed(() => {
    if (results.value.length > 0) {
        return results.value[0][1]
    } else {
        return Number.MAX_SAFE_INTEGER
    }
})

// filters
const filterSlots = ref<ArtifactPosition[]>([])
const filterSetNames = ref<ArtifactSetName[]>([])
const filterMainStatNames = ref<ArtifactStatName[]>([])
const filterLevel = ref<[number, number]>([0, 20])

const filteredResults = computed((): [number, number][] => {
    let r = []

    for (let item of results.value) {
        const artifact = artifactStore.artifacts.value.get(item[0])
        if (!artifact) {
            continue
        }

        if (filterSlots.value.length > 0) {
            if (filterSlots.value.indexOf(artifact.position) === -1) {
                continue
            }
        }

        if (filterSetNames.value.length > 0) {
            if (filterSetNames.value.indexOf(artifact.setName) === -1) {
                continue
            }
        }

        if (filterMainStatNames.value.length > 0) {
            if (filterMainStatNames.value.indexOf(artifact.mainTag.name) === -1) {
                continue
            }
        }

        const level = artifact.level
        if (level < filterLevel.value[0] || level > filterLevel.value[1]) {
            continue
        }

        r.push(item)
    }

    return r
})

// pager
const currentPage = ref(1)

const resultsToBeDisplay = computed((): [number, number][] => {
    const p = currentPage.value
    return filteredResults.value
        .slice((p - 1) * 20, Math.min(p * 20, filteredResults.value.length))
})

const artifactsToBeDisplayed = computed((): [IArtifact, number][] => {
    const r = <[IArtifact, number][]>[]
    for (let item of resultsToBeDisplay.value) {
        const [artifactId, score] = item
        const a = artifactStore.artifacts.value.get(artifactId)
        if (a) {
            r.push([a, score])
        }
    }
    return r
})

// compute
const potentialFunctionInterface = computed(() => {
    return {
        name: potentialFunctionName.value,
        config: potentialFunctionConfig.value
    }
})

function handleClickStart() {
    const artifactsWasm = getArtifactsWasm()
    const pfInterface = potentialFunctionInterface.value
    // console.log(pfInterface)

    const loading = ElLoading.service({
        text: "莫娜占卜中",
        lock: true,
        fullscreen: true
    })

    wasmComputeArtifactPotential(pfInterface, artifactsWasm).then((res: any) => {
        // console.log(results)
        results.value = res
    }).catch((e: any) => {
        console.log(e)
        ElMessage({
            message: e,
            type: "error"
        })
    }).finally(() => {
        loading.close()
    })
}

// export default {

//     watch: {
//         potentialFunctionName(newValue, oldValue) {
//             if (newValue === oldValue) {
//                 return
//             }
//
//             const item = potentialFunctionData[newValue]
//             const configConfigs = item.config ?? []
//             if (configConfigs.length === 0) {
//                 this.potentialFunctionConfig = "NoConfig"
//             } else {
//                 let defaultConfig = {}
//                 for (const config of configConfigs) {
//                     defaultConfig[config.name] = config.default
//                 }
//
//                 this.potentialFunctionConfig = {
//                     [newValue]: defaultConfig
//                 }
//             }
//         }
//     }
// }
</script>

<style scoped lang="scss">
@media only screen and (min-width: 992px) {
    .left, .col-right {
        height: calc(100vh - 2 * 24px);
    }

    .col-right {
        padding-left: 16px;
    }

    .filter-title {
        margin-bottom: 4px;
    }

    .filter {
        display: flex;
        align-items: center;
        gap: 4px;
        margin: 16px 0;

        .filter-item {
            flex: 1;


            .el-select {
                width: 100%;
            }
        }
    }
}

@media only screen and (max-width: 992px) {
    .left {
        margin-bottom: 16px;
    }
    .filter-title {
        margin: 4px 0;
        display: inline-block;
    }

    .filter {
        .filter-item {
            .el-select {
                width: 100%;
            }

            display: block;
        }
    }

    .filter-level {
        display: flex;
        align-items: center;
        gap: 8px;
        .el-input-number {
            flex: 1;
        }
    }
}

.filter-title {
    font-size: 0.8rem;
    color: #606266;
}

.pf-detail {
    margin-top: 20px;
    display: flex;
    align-items: flex-start;

    .pf-image {
        height: 64px;
        width: 64px;
        border-radius: 64px;
        margin-right: 8px;
    }

    .pf-title {
        font-weight: bold;
        font-size: 0.9rem;
        margin: 0 0 12px 0;
    }

    .pf-description {
        font-size: 0.8rem;
        margin: 0;
        color: #606266;
    }
}

.pf-config {
    margin-top: 20px;
}

.col-right {
    position: relative;
}

.pager {
    margin-top: 16px;

}

.artifacts-div {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 4px;

    //justify-content: space-between;
    //align-content: flex-start;
    //grid-gap: 12px;
}
</style>