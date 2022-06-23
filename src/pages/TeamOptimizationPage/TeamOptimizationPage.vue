<template>
    <div>
        <el-drawer
            title="面板"
            :visible.sync="showAttributeDrawer"
            :size="deviceIsPC ? '30%' : '100%'"
        >
            <template v-if="!wasmAttribute">
                <el-empty>In theory, in should not see this</el-empty>
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
                    <el-button type="primary" size="mini" icon="el-icon-cpu" @click="handleClickStart">开始计算</el-button>
                    <el-button size="mini" icon="el-icon-plus" @click="handleClickAddMember">添加成员</el-button>
                </div>

                <div
                    v-for="(presetName, index) in presetNames"
                    :key="index"
                    class="member-item"
                >
                    <div style="display: flex; justify-content: space-between; align-items: center" class="member-header">
                        <p class="team-title">成员{{ index + 1 }}</p>
                        <div>
                            <el-button
                                circle
                                size="mini"
                                type="text"
                                icon="el-icon-delete"
                                @click="handleDeleteMember(index)"
                                style="color: white"
                            ></el-button>
                        </div>
                    </div>


                    <p class="common-title2">计算预设</p>
                    <select-preset
                        v-model="presetNames[index]"
                    ></select-preset>

                    <p class="common-title2">权重</p>
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
                        :value="resultIndex + 1"
                        @input="handleChangeResultIndex"
                        :min="1"
                        :max="results.length"
                        size="small"
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
                                    icon="el-icon-s-data"
                                    circle
                                    size="mini"
                                    type="text"
                                    title="查看面板"
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
                                :item="artifactsById[artifactId]"
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
import { reactive, ref } from "vue"

import {convertArtifact} from "@/utils/converter"
import {team_optimize, wasmGetAttribute} from "@/wasm"
import {convertPresetToWasmInterface, getPresetEntryByName} from "@/utils/preset"
import {toggleArtifact} from "@/utils/artifacts"
import {deviceIsPC} from "@/utils/device"

import SelectCharacter from "@/components/select/SelectCharacter"
import SelectWeapon from "@/components/select/SelectWeapon"
import ItemConfig from "@/components/config/ItemConfig"
import ArtifactDisplay from "@/components/display/ArtifactDisplay"
import MyButton1 from "@/components/button/MyButton1"
import PresetItem from "@/components/display/PresetItem"
import SelectPreset from "@/components/select/SelectPreset"
import AttributePanel from "@/components/display/AttributePanel"
import {ElMessage} from "element-plus"


type MemberPresetName = string | null
const presetNames = reactive([null] as MemberPresetName[])
const weights = reactive([0] as number[])
const MAX_MEMBERS = 8

function handleClickAddMember() {
    if (presetNames.length === MAX_MEMBERS) {
        ElMessage({
            message: "最多支持8个成员",
            type: "error"
        })
        return
    }
    presetNames.push(null)
    weights.push(0)
}

function handleDeleteMember(index: number) {
    if (presetNames.length === 1) {
        return
    }
    presetNames.splice(index, 1)
    weights.splice(index, 1)
}


const showAttributeDrawer = ref(false)
const wasmAttribute = ref(null as any)

function wasmGetAttributeInterface(index: number) {
    let artifacts = []
    if (this.currentResultEntry) {
        const artifactIds = Object.values(this.currentResultEntry[index])
        const artifactsOldFormat = artifactIds.map(x => this.artifactsById[x]).filter(x => x)
        artifacts = artifactsOldFormat.map(a => convertArtifact(a))
    }
    // console.log(this.presets[index])

    return {
        character: this.presets[index].item.character,
        weapon: this.presets[index].item.weapon,
        buffs: this.presets[index].item.buffs,
        artifacts,
    }
},

async function handleClickDisplayAttributePanel(index: number) {
    const input = wasmGetAttributeInterface(index)
    // console.log(input)
    const result = await wasmGetAttribute(input)
    this.wasmAttribute = result

    this.showAttributeDrawer = true
    // console.log(result)
},

export default {
    name: "TeamOptimizationPage",
    components: {
        SelectCharacter,
        SelectWeapon,
        ItemConfig,
        ArtifactDisplay,
        MyButton1,
        PresetItem,
        SelectPreset,
        AttributePanel,
    },
    data() {
        return {
            results: [],    // a 3d array
            resultIndex: 0,

            showAttributeDrawer: false,
            wasmAttribute: null,

            deviceIsPC
        }
    },
    methods: {
        handleClickStart() {
            const canStart = this.presets.length === this.presetNames.length
            if (!canStart) {
                this.$message.error("请选择计算预设")
                return
            }

            const interfaceWasm = this.optimizeTeamWasmInterface
            const artifacts = this.filteredArtifactsWasm
            // console.log(artifacts)
            // console.log(interfaceWasm)

            const loading = this.$loading({
                lock: true,
                text: "莫娜占卜中（可能需要数分钟）"
            })

            team_optimize(interfaceWasm, artifacts).then(result => {
                // console.log(result)
                this.results = result.artifacts
                this.resultIndex = 0
            }).catch(e => {
                console.log(e)
            }).finally(() => {
                loading.close()
            })
        },

        handleChangeResultIndex(index) {
            this.resultIndex = index - 1
        },

        handleClickDisplayAttributePanel: async function (index) {
            const input = this.wasmGetAttributeInterface(index)
            // console.log(input)
            const result = await wasmGetAttribute(input)
            this.wasmAttribute = result

            this.showAttributeDrawer = true
            // console.log(result)
        },

        wasmGetAttributeInterface(index) {
            let artifacts = []
            if (this.currentResultEntry) {
                const artifactIds = Object.values(this.currentResultEntry[index])
                const artifactsOldFormat = artifactIds.map(x => this.artifactsById[x]).filter(x => x)
                artifacts = artifactsOldFormat.map(a => convertArtifact(a))
            }
            // console.log(this.presets[index])

            return {
                character: this.presets[index].item.character,
                weapon: this.presets[index].item.weapon,
                buffs: this.presets[index].item.buffs,
                artifacts,
            }
        },

        // not used, todo
        handleClickSaveAsKumi(index) {
            let artifacts = []
            if (this.currentResultEntry) {
                const artifactIds = Object.values(this.currentResultEntry[index])
                artifacts = artifactIds.map(x => this.artifactsById[x]).filter(x => x)
            }
        },

        handleToggleArtifact(artifactId) {
            toggleArtifact(artifactId)
        }
    },
    computed: {
        ...mapGetters("artifacts", {
            artifactsFlat: "allFlat",
            artifactsById: "artifactsById",
        }),

        singleInterfaces() {
            return this.presets.map(x => convertPresetToWasmInterface(x.item))
        },

        currentResultEntry() {
            if (this.results.length === 0) {
                return null
            } else {
                return this.results[this.resultIndex]
            }
        },

        filteredArtifacts() {
            let results = []
            for (let artifact of this.artifactsFlat) {
                if (artifact.level >= 16) {
                    results.push(artifact)
                }
            }
            return results.filter(a => !a.omit)
        },

        filteredArtifactsWasm() {
            return this.filteredArtifacts.map(convertArtifact)
        },

        presets() {
            let results = []
            for (let name of this.presetNames) {
                if (name) {
                    results.push(getPresetEntryByName(name))
                }
            }
            return results
        },

        optimizeTeamHyperParamInterface() {
            // todo
            return {
                mva_step: 5,
                work_space: 1000,
                max_re_optimize: 5,
                max_search: 1000000,
                count: 1000,
            }
        },

        optimizeTeamWasmInterface() {
            return {
                // single_interfaces: interfaces,
                // weights: weights,
                single_interfaces: this.singleInterfaces,
                weights: this.weights,
                hyper_param: this.optimizeTeamHyperParamInterface
            }
        }
    },
}
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