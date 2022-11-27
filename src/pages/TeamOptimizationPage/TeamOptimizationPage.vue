<template>
    <div>
        <el-drawer
            :title="t('misc.panel')"
            v-model="showAttributeDrawer"
            :size="deviceIsPC ? '30%' : '100%'"
        >
            <template v-if="!wasmAttribute">
                <el-empty>In theory, you should not see this</el-empty>
            </template>
            <template v-else>
                <div style="padding: 0 20px">
                    <attribute-panel
                        :attribute="wasmAttribute"
                    ></attribute-panel>
                </div>
            </template>
        </el-drawer>

        <el-row :gutter="16">
            <el-col
                :md="6"
                :sm="24"
                class="mona-scroll-hidden left"
            >
                <div style="margin-bottom: 12px">
                    <el-button type="primary" :icon="IconEpCpu" @click="handleClickStart">{{ t("teamPage.start") }}</el-button>
                    <el-button :icon="IconEpPlus" @click="handleClickAddMember">{{ t("teamPage.add") }}</el-button>
                </div>

                <div
                    v-for="(presetName, index) in presetNames"
                    :key="index"
                    class="member-item"
                >
                    <div style="display: flex; justify-content: space-between; align-items: center" class="member-header">
                        <p class="team-title">{{ t("teamPage.member") }}{{ index + 1 }}</p>
                        <div>
                            <el-button
                                circle
                                text
                                :icon="IconEpDelete"
                                @click="handleDeleteMember(index)"
                                style="color: white"
                            ></el-button>
                        </div>
                    </div>


                    <p class="common-title2">{{ t("misc.preset") }}</p>
                    <select-preset
                        v-model="presetNames[index]"
                    ></select-preset>

                    <p class="common-title2">{{ t("teamPage.weight") }}</p>
                    <el-slider
                        v-model="weights[index]"
                        :min="0"
                        :max="1"
                        :step="0.01"
                        :show-input="true"
                        style="padding-left: 8px"
                    ></el-slider>

<!--                    <el-divider v-if="index < presetNames.length - 1"></el-divider>-->
                </div>
            </el-col>

            <el-col
                :md="18"
                :sm="24"
                class="mona-scroll-hidden right"
            >
                <template v-if="currentResultEntry">
                    <el-input-number
                        :model-value="resultIndex + 1"
                        @update:modelValue="handleChangeResultIndex"
                        :min="1"
                        :max="results.length"
                        style="margin-bottom: 12px"
                    ></el-input-number>
                    <div
                        v-for="(presetName, index) in currentResultEntry"
                        :key="index"
                        class="result-item"
                    >
                        <div class="result-item-top">
                            <div>
<!--                                <span class="result-item-title">{{ characterChs[index] }}</span>-->
                                <el-button
                                    :icon="IconEpHistogram"
                                    circle
                                    text
                                    :title="t('teamPage.showStat')"
                                    @click="handleClickDisplayAttributePanel(index)"
                                ></el-button>
                            </div>

                            <div class="result-item-buttons">
                            </div>
                        </div>
                        <div class="result-item-content">
                            <artifact-display
                                v-for="artifactId in currentResultEntry[index]"
                                :key="artifactId"
                                :item="artifactStore.artifacts.value.get(artifactId)"
                                :buttons="true"
                                :lock-button="true"
                                :delete-button="false"
                                :edit-button="false"
                                @toggle="handleToggleArtifact(artifactId)"
                            ></artifact-display>
                        </div>
                    </div>
                </template>
                <template v-else>
                    <div
                        style="display: flex; justify-content: center;"
                    >
                        <el-empty></el-empty>
                    </div>
                </template>
            </el-col>
        </el-row>
    </div>
</template>

<script setup lang="ts">
import {ref} from "vue"

import {convertArtifact} from "@/utils/converter"
import {team_optimize} from "@/wasm"
import {convertPresetToWasmInterface} from "@/utils/preset"
import {deviceIsPC} from "@/utils/device"
// @ts-ignore
import ArtifactDisplay from "@/components/display/ArtifactDisplay.vue"
import SelectPreset from "@/components/select/SelectPreset.vue"
import AttributePanel from "@/components/display/AttributePanel"
import {useArtifactStore} from "@/store/pinia/artifact"
import type {IArtifact} from "@/types/artifact"
import {usePresetStore} from "@/store/pinia/preset"
import {useAccountStore} from "@/store/pinia/account"
import {useMona} from "@/wasm/mona"
import {deepCopy} from "@/utils/common"

import IconEpCpu from "~icons/ep/cpu"
import IconEpPlus from "~icons/ep/plus"
import IconEpDelete from "~icons/ep/delete"
import IconEpHistogram from "~icons/ep/histogram"
import {useI18n} from "@/i18n/i18n";

const { t } = useI18n()


const artifactStore = useArtifactStore()
const presetStore = usePresetStore()
const accountStore = useAccountStore()

const mona = await useMona()

// members
type MemberPresetName = string | null
const presetNames = ref([null] as MemberPresetName[])
const weights = ref([0] as number[])
const MAX_MEMBERS = 8

function handleClickAddMember() {
    if (presetNames.value.length === MAX_MEMBERS) {
        ElMessage({
            message: "最多支持8个成员",
            type: "error"
        })
        return
    }
    presetNames.value.push(null)
    weights.value.push(0)
}

function handleDeleteMember(index: number) {
    if (presetNames.value.length === 1) {
        return
    }
    presetNames.value.splice(index, 1)
    weights.value.splice(index, 1)
}

const presets = computed(() => {
    let results = []
    for (let name of presetNames.value.values()) {
        if (name) {
            const preset = presetStore.presets.value[name]
            if (preset) {
                results.push(preset)
            }
        }
    }
    return results
})


// results
const results = ref<Array<Array<Record<string, number>>>>([])    // a 3d array
const resultIndex = ref(0)
const currentResultEntry = computed((): Record<string, number>[] | null => {
    if (results.value.length === 0) {
        return null
    } else {
        return results.value[resultIndex.value]
    }
})

function handleChangeResultIndex(index: number) {
    resultIndex.value = index - 1
}


// attribute
const showAttributeDrawer = ref(false)
const wasmAttribute = ref(null as any)

function wasmGetAttributeInterface(index: number) {
    let artifacts: any[] = []
    if (currentResultEntry.value) {
        const artifactIds = Object.values(currentResultEntry.value[index])
        const artifactsOldFormat: IArtifact[] = []
        for (let artifactId of artifactIds) {
            const a = artifactStore.artifacts.value.get(artifactId)
            if (a) {
                artifactsOldFormat.push(a)
            }
        }
        artifacts = artifactsOldFormat.map(a => convertArtifact(a))
    }

    return {
        character: presets.value[index].item.character,
        weapon: presets.value[index].item.weapon,
        buffs: presets.value[index].item.buffs,
        artifacts,
    }
}

async function handleClickDisplayAttributePanel(index: number) {
    const input = wasmGetAttributeInterface(index)
    wasmAttribute.value = await mona.CommonInterface.get_attribute(input)

    showAttributeDrawer.value = true
}

// artifacts
function handleToggleArtifact(artifactId: number) {
    artifactStore.toggleArtifact(artifactId)
}

const filteredArtifacts = computed((): IArtifact[] => {
    let results: IArtifact[] = []
    for (let artifact of artifactStore.artifacts.value.values()) {
        if (artifact.level >= 16 && !artifact.omit) {
            results.push(artifact)
        }
    }
    return results
})

const filteredArtifactsWasm = computed(() => {
    return filteredArtifacts.value.map(convertArtifact)
})


// do calculation
const singleInterfaces = computed(() => {
    return presets.value.map(x => convertPresetToWasmInterface(x.item))
})

const optimizeTeamHyperParamInterface = {
    mva_step: 5,
    work_space: 1000,
    max_re_optimize: 5,
    max_search: 1000000,
    count: 1000,
}

const optimizeTeamWasmInterface = computed(() => {
    return {
        // single_interfaces: interfaces,
        // weights: weights,
        single_interfaces: singleInterfaces.value,
        weights: weights.value,
        hyper_param: optimizeTeamHyperParamInterface
    }
})

function handleClickStart() {
    const canStart = presets.value.length === presetNames.value.length
    if (!canStart) {
        ElMessage({
            message: "请选择计算预设",
            type: "error"
        })
        return
    }

    const interfaceWasm = deepCopy(optimizeTeamWasmInterface.value)
    // no need to deep copy because it's converted into wasm format, which is not reactive
    const artifacts = filteredArtifactsWasm.value

    const loading = ElLoading.service({
        text: "莫娜占卜中（可能需要数分钟）",
        lock: true,
        fullscreen: true
    })

    team_optimize(interfaceWasm, artifacts).then(result => {
        // console.log(result)
        results.value = result.artifacts
        resultIndex.value = 0
    }).catch(e => {
        ElMessage({
            message: `计算过程发生错误：${e.message ?? e}`,
            type: "error"
        })
        console.error(e)
    }).finally(() => {
        loading.close()
    })
}

watch(() => accountStore.currentAccountId.value, () => {
    presetNames.value = [null]
    weights.value = [0]
    results.value = []
    resultIndex.value = 0
})
</script>

<style scoped lang="scss">
@media (min-width: 992px) {
    .left, .right {
        height: calc(100vh - 2 * 24px);
    }
}

@media (max-width: 992px) {
    .el-select {
        width: 100%;
    }

    //.left {
    //    margin-bottom: 12px;
    //}
}

.member-item {
    margin-bottom: 16px;
    //box-shadow: 0 0 10px 1px #00000011;
    //padding: 12px;

    &:last-of-type {
        margin-bottom: 64px;
    }

    .member-header {
        //background-color: rgb(236, 245, 255);
        background-color: #409EFF;
        padding: 0 8px;
        height: 32px;
        //border-radius: 3px;
    }

    .team-title {
        font-size: 0.9rem;
        font-weight: bold;
        //color: #606166;
        color: white;
        margin: 0;

        //border-left: 2px solid #409EFF;
        //padding-left: 12px;
    }
}

.common-title2 {
    font-size: 12px;
    color: #666666;
}



.result-item {
    margin-bottom: 12px;

    &:last-of-type {
        margin-bottom: 0;
    }

    .result-item-top {
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 36px;
        border-bottom: 1px solid #00000011;

        .result-item-title {
            font-size: 12px;
        }

        .result-item-buttons {
            display: flex;
            align-items: center;
        }
    }

    .result-item-content {
        //padding-top: 12px;
        //display: flex;
        //flex-wrap: wrap;
        //gap: 12px;
        display: grid;
        gap: 4px;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    }
}
</style>
