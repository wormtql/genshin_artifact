<template>
    <div>
        <apply-preset-dialog ref="applyPresetDialog" @selected="addPreset"></apply-preset-dialog>

        <el-dialog title="选择圣遗物" :width="deviceIsPC ? '80%' : '90%'" v-model="showSelectArtifactDialog">
            <select-artifact :position="selectArtifactSlot" @select="handleSelectArtifact"></select-artifact>
        </el-dialog>

        <el-row style="margin-bottom: 12px">
            <el-col :span="12">
                <el-button size="default" :icon="IconEpFolderOpened" @click="handleClickImportSequence">导入序列</el-button>
                <el-button type="primary" size="default" :icon="IconEpFolderChecked" :disabled="!sequenceDirty"
                    @click="handleClickSaveSequence">保存序列</el-button>
                <el-divider direction="vertical"></el-divider>
                <el-button v-if="!cancelOptimizeArtifact" type="primary" size="default" :icon="IconEpCpu"
                    @click="handleClickStart">开始计算</el-button>
                <el-button v-if="cancelOptimizeArtifact" type="danger" size="default" :icon="IconEpWarningOutline"
                    @click="cancelOptimizeArtifact">中止计算</el-button>
            </el-col>
            <el-col :span="12">
                <div style="float: right">
                    <el-button-group>
                        <el-button type="primary" size="default" :icon="IconEpStarOn"
                            @click="handleClickSaveToDirectory">
                            存至收藏夹
                        </el-button>
                        <el-dropdown trigger="click" @command="handleClickImportFromDirectory">
                            <el-button size="default" :icon="IconEpDownload">导入</el-button>
                            <template #dropdown>
                                <el-dropdown-menu>
                                    <el-dropdown-item v-for="item in directories" :key="item.id" :command="item.id">
                                        {{ item.title }}
                                    </el-dropdown-item>
                                </el-dropdown-menu>
                            </template>
                        </el-dropdown>
                        <el-dropdown trigger="click" @command="handleClickCompareWithDirectory">
                            <el-button size="default" :icon="IconEpDocumentCopy">
                                对比{{ oldDirectoryId !== null ? '中' : ''}}
                            </el-button>
                            <template #dropdown>
                                <el-dropdown-menu>
                                    <el-dropdown-item v-for="item in directories" :key="item.id" :command="item.id"
                                        :class="{ 'directory-active': oldDirectoryId == item.id }">
                                        {{ item.title }}
                                    </el-dropdown-item>
                                    <el-dropdown-item v-if="oldDirectoryId !== null" divided command="cancel">
                                        取消
                                    </el-dropdown-item>
                                </el-dropdown-menu>
                            </template>
                        </el-dropdown>
                    </el-button-group>
                </div>
            </el-col>
        </el-row>

        <!-- https://github.com/SortableJS/vue.draggable.next/issues/159 -->
        <!-- don't add tag="transition-group" -->
        <draggable :component-data="{
            type: 'transition-group',
            name: !dragging ? 'flip-list' : null
        }" :list="sequenceData" handle=".member-item" :animation="300" :disabled="false" ghostClass="ghost"
            @start="dragging = true" @end="dragging = false" item-key="id">
            <template #item="{ element: { id, name, arts }, index }">
                <el-row :gutter="16">
                    <el-col :md="6" :sm="24" class="mona-scroll-hidden left member-item">
                        <div style="display: flex; justify-content: space-between; align-items: center"
                            class="member-header">
                            <p class="team-title">成员{{ index + 1 }}</p>
                            <div>
                                <el-button circle size="small" link :icon="IconEpArrowUp" :disabled="index === 0"
                                    @click="handleUpMember(index)" style="color: white"></el-button>
                                <el-button circle size="small" link :icon="IconEpArrowDown"
                                    :disabled="index === sequenceData.length - 1" @click="handleDownMember(index)"
                                    style="color: white"></el-button>
                                <el-button circle size="small" link :icon="IconEpDelete"
                                    @click="handleDeleteMember(index)" style="color: white"></el-button>
                            </div>
                        </div>

                        <div style="display: flex; justify-content: space-between; align-items: center">
                            <preset-item :toolbar="false" :item="presets[index].item" :name="name" style="width: 100%;">
                            </preset-item>
                        </div>
                    </el-col>

                    <el-col :md="18" :sm="24" :id="id" class="mona-scroll-hidden right result-item">
                        <div class="result-item-top">
                            <div>
                                <el-button size="small" :icon="IconEpView" @click="handleRedirectToCalculator(index)">
                                    角色详情</el-button>
                                <el-button size="small" :icon="IconEpSmoking"
                                    @click="handleStartCompute(index, index + 1)">计算这个</el-button>
                                <el-button size="small" :icon="IconEpCaretBottom" @click="handleStartCompute(index)">
                                    计算这个及以下</el-button>
                            </div>
                            <div class="result-item-buttons">
                            </div>
                        </div>
                        <div class="result-item-content">
                            <div v-for="(artId, artIndex) in arts" :key="artIndex" class="artifact-item-or-button">
                                <artifact-display v-if="artifactsById.get(artId)" :item="artifactsById.get(artId)"
                                    selectable :buttons="true" :delete-button="true"
                                    @delete="handleRemoveArtifact(index, artIndex)"
                                    @toggle="handleToggleArtifact(artId)"
                                    @click="handleGotoSelectArtifact(index, artIndex)" class="artifact-display"
                                    :class="{ differ: oldDirectory.has(name) && artId !== oldDirectory.get(name)[artIndex] }">
                                </artifact-display>
                                <add-button v-else @click="handleGotoSelectArtifact(index, artIndex)"
                                    class="add-button"></add-button>
                            </div>
                        </div>
                    </el-col>
                </el-row>
            </template>
        </draggable>
        <el-row :key="'this-is-a-unique-key-for-add-button!!!!'" :gutter="16">
            <el-col :md="6" :sm="24" class="mona-scroll-hidden left member-item">
                <add-button msg="添加成员" @click="applyPresetDialog!.open()" style="height: 7vw; width: 100%">
                </add-button>
            </el-col>
        </el-row>
    </div>
</template>

<script setup lang="ts">
import { ElMessage, ElMessageBox } from "element-plus"
import draggable from "vuedraggable"
import IconEpFolderOpened from "~icons/ep/folder-opened"
import IconEpFolderChecked from "~icons/ep/folder-checked"
import IconEpCpu from "~icons/ep/cpu"
import IconEpWarningOutline from "~icons/ep/warning"
import IconEpStarOn from "~icons/ep/star"
import IconEpDownload from "~icons/ep/download"
import IconEpDocumentCopy from "~icons/ep/document-copy"
import IconEpArrowUp from "~icons/ep/arrow-up"
import IconEpArrowDown from "~icons/ep/arrow-down"
import IconEpDelete from "~icons/ep/delete"
import IconEpView from "~icons/ep/view"
import IconEpSmoking from "~icons/ep/smoking"
import IconEpCaretBottom from "~icons/ep/caret-bottom"
import ArtifactDisplay from "@/components/display/ArtifactDisplay.vue"
import PresetItem from "@/components/display/PresetItem.vue"
import AddButton from "@/components/misc/AddButton.vue"
import ApplyPresetDialog from "../NewArtifactPlanPage/ApplyPresetDialog.vue"
import SelectArtifact from "@/components/select/SelectArtifact.vue"

import objectHash from "object-hash"
import { useMona } from "@/wasm/mona"
import { convertArtifact } from "@/utils/converter"
import { deepCopy } from "@/utils/common"
import { wasmSingleOptimize } from "@/wasm/single_optimize"
import { convertPresetToWasmInterface } from "@/utils/preset"
import { deviceIsPC } from "@/utils/device"
import { usePresetStore } from "@/store/pinia/preset"
import type { ArtifactPosition } from "@/types/artifact"
import { useArtifactStore } from "@/store/pinia/artifact"
import { useKumiStore } from "@/store/pinia/kumi"
import { useSequenceStore } from "@/store/pinia/sequence"
import { useAccountStore } from "@/store/pinia/account"
import { useRouter } from "vue-router"

const artifactStore = useArtifactStore()
const artifactsById = artifactStore.artifacts
const presetStore = usePresetStore()
const kumiStore = useKumiStore()
const sequenceStore = useSequenceStore()
const accountStore = useAccountStore()
const router = useRouter()

// sequence management
interface SequenceItem {
    name: string,
    id: string,
    arts: number[],
}

const sequenceData = reactive<SequenceItem[]>([])

function genUniqueId() {
    return 'arts' + String(Math.floor(Math.random() * 1e8))
}

const applyPresetDialog = ref<InstanceType<typeof ApplyPresetDialog> | null>(null)

function defaultSeqItem(name: string) {
    return {
        name,
        id: genUniqueId(),
        arts: [-1, -1, -1, -1, -1],
    }
}

function addPreset(name: string) {
    sequenceData.push(defaultSeqItem(name))
}

function swap(arr: any[], i: number, j: number) {
    const temp = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
}

function handleUpMember(index: number) {
    swap(sequenceData, index, index - 1)
}

function handleDownMember(index: number) {
    swap(sequenceData, index, index + 1)
}

function handleDeleteMember(index: number) {
    sequenceData.splice(index, 1)
}

const presetNames = computed(() => sequenceData.map(item => item.name))
const presets = computed(() => presetNames.value.map(name => presetStore.presets.value[name]))

const presetValid = computed(() => presets.value.every(preset => preset))
watch(() => presetValid.value, valid => {
    if (!valid) {
        for (let i = sequenceData.length - 1; i >= 0; i--) {
            if (!presets.value[i]) {
                sequenceData.splice(i, 1)
            }
        }
    }
})

// sequence management: save or restore the sequence data
const savedSequenceHash = ref<string | null>(null)
const sequenceDirty = computed(() => {
    const hash = objectHash(presetNames.value)
    return hash !== savedSequenceHash.value
})

function handleClickSaveSequence() {
    sequenceStore.sequence.value = presetNames.value.slice()
    savedSequenceHash.value = objectHash(presetNames.value)
    ElMessage.info('保存序列成功')
}

function handleClickImportSequence() {
    const newData = sequenceStore.sequence.value.map(defaultSeqItem)
    sequenceData.splice(0, sequenceData.length, ...newData)
    savedSequenceHash.value = objectHash(presetNames.value)
}

// watch: {
//     "$store.state.accounts.currentAccountId"() {
//         this.sequenceData = []
//     },
// },

// dragging
const dragging = ref(false)

// optimization
const cancelOptimizeArtifact = ref<(() => void) | null>(null)

function getFilteredArtifactsWasm(excludeResults: number[][]) {
    let results = []
    for (let artifact of artifactStore.artifacts.value.values()) {
        if (artifact.level >= 16) {
            results.push(artifact)
        }
    }
    let used = new Set()
    for (let artIds of excludeResults) {
        for (let artId of artIds) {
            used.add(artId)
        }
    }
    return results.filter(a => !a.omit && !used.has(a.id)).map(convertArtifact)
}

async function handleStartCompute(start: number, end?: number) {
    if (end === undefined) {
        end = presets.value.length
    }
    const canStart = presets.value.slice(start, end).every(x => x)
    if (!canStart) {
        ElMessage.error("计算范围内有计算预设已被删除")
        return
    }

    for (let i = start; i < end; i++) {
        const item = sequenceData[i]
        item.arts = [-1, -1, -1, -1, -1]
        const singleInterface = deepCopy({
            ...convertPresetToWasmInterface(presets.value[i].item),
            // max_result_num: 1,
        })
        const loading = ElLoading.service({
            target: '#' + item.id,
            lock: true,
            text: "莫娜占卜中",
            // background: 'rgba(0, 0, 0, 0.7)',
        })
        const usedArts = sequenceData.slice(0, i).map(({ arts }) => arts)
        const availableArts = getFilteredArtifactsWasm(usedArts)
        const [promise, cancel] = wasmSingleOptimize(singleInterface, availableArts, 120000, true)
        cancelOptimizeArtifact.value = cancel
        let results
        try {
            results = await promise;
        } catch (e) {
            ElMessage.error(e as string)
            break
        } finally {
            cancelOptimizeArtifact.value = null
            loading.close()
        }
        if (results.length === 0) {
            ElMessage.error("没有符合条件的圣遗物")
            break
        }
        item.arts = artifactObjectToArray(results[0])
    }
}

async function handleClickStart() {
    if (sequenceDirty.value) {
        try {
            await ElMessageBox.confirm("是否保存序列？", "警告", {
                confirmButtonText: "确定",
                cancelButtonText: "取消",
                type: 'warning'
            })
            handleClickSaveSequence()
        } catch (e) {
        }
    }
    await handleStartCompute(0)
}

function handleRedirectToCalculator(index: number) {
    const item = sequenceData[index]
    if (oldDirectory.value.has(item.name)) {
        router.push({
            name: "calculate",
            query: {
                presetName: item.name,
                artifactGroups: JSON.stringify([item.arts, oldDirectory.value.get(item.name)])
            }
        })
    } else {
        router.push({
            name: "calculate",
            query: {
                presetName: item.name,
                artifacts: JSON.stringify(item.arts),
            }
        })
    }
}

// select artifact
const showSelectArtifactDialog = ref(false)
const selectArtifactSlot = ref<ArtifactPosition>("flower")
const handleSelectArtifact = ref<((artId: number) => void)>(() => { })

function handleRemoveArtifact(index: number, artIndex: number) {
    sequenceData[index].arts[artIndex] = -1
}

function handleToggleArtifact(id: number) {
    artifactStore.toggleArtifact(id)
}

const index2SlotName: ArtifactPosition[] = ["flower", "feather", "sand", "cup", "head"]
function handleGotoSelectArtifact(index: number, artIndex: number) {
    const slotName = index2SlotName[artIndex]
    selectArtifactSlot.value = slotName
    handleSelectArtifact.value = id => {
        sequenceData[index].arts[artIndex] = id
        showSelectArtifactDialog.value = false
        handleSelectArtifact.value = () => { }
    }
    showSelectArtifactDialog.value = true
}

////////////////////////////////////
// save, import and compare kumis

function artifactObjectToArray(art: Record<string, number>) {
    return [
        art.flower || -1,
        art.feather || -1,
        art.sand || -1,
        art.goblet || art.cup || -1,
        art.head || -1,
    ]
}

function getArtifactsFromDirectory(dirId: number | null) {
    //preset in page includes name artID & arts,
    //while preset in dir use id, title & artifactIds
    //get array of kumisID by dirID
    if (dirId === null) {
        return new Map()
    }
    const kumiArr = kumiStore.kumisByDirId.value[dirId]
    //order arts and transform kumisID to map
    const orderedArtsMap = new Map()
    for (const kumi of kumiArr) {
        const tempOrderdArtsSeq: Record<string, number> = {}
        for (const artId of kumi.artifactIds!) {
            let art = artId && artifactStore.artifacts.value.get(artId)
            if (art) {
                tempOrderdArtsSeq[art.position] = art.id
            }
        }
        orderedArtsMap.set(kumi.title, artifactObjectToArray(tempOrderdArtsSeq))
    }

    return orderedArtsMap
}

function handleClickSaveToDirectory() {
    const dirName = new Date().toLocaleString()
    const dirId = kumiStore.createDir(dirName)
    for (const item of sequenceData) {
        const kumiId = kumiStore.createKumi(dirId, item.name)!
        for (const artifactId of item.arts) {
            if (artifactId >= 0) {
                kumiStore.addArtifact(kumiId, artifactId)
            }
        }
    }
    ElMessage.success({ message: `已保存到"${dirName}"收藏夹` })
}

const oldDirectoryId = ref<number | null>(null)
const oldDirectory = computed(() => getArtifactsFromDirectory(oldDirectoryId.value))

const directories = kumiStore.dirs
function handleClickImportFromDirectory(dirId: number) {
    const artsMap = getArtifactsFromDirectory(dirId)
    //iterate sequenceData
    for (const item of sequenceData) {
        if (artsMap.has(item.name)) {
            item.arts = artsMap.get(item.name)
        }
    }
}

function handleClickCompareWithDirectory(dirId: number | 'cancel') {
    oldDirectoryId.value = dirId === 'cancel' ? null : dirId
}

// watch account change
watch(() => accountStore.currentAccountId.value, () => {
    sequenceData.splice(0, sequenceData.length)
})
</script>

<style scoped lang="scss">
@media (min-width: 992px) {
    // .left, .right {
    //     height: calc(100vh - 2 * 24px);
    // }
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
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));

        // .artifact-item-or-button {
        //     .add-button {
        //         width: 100%;
        //         height: 100%;
        //     }
        //     .artifact-display {
        //         width: 100%;
        //         box-sizing: border-box;
        //         &.differ {
        //             box-shadow: 2px 2px 3px #888888;
        //             :first-child {
        //                 background-color: rgb(217, 236, 255);
        //             }
        //         }
        //     }
        // }
        .artifact-item-or-button {
            .add-button {
                width: 100%;
                height: 100%;
                min-height: 7vw;
            }

            .artifact-display {
                overflow: hidden;
                // border-radius: 5px;
                width: 100%;
                box-sizing: border-box;
                z-index: 0;

                &.differ {
                    @keyframes hint {
                        0% {
                            opacity: 0;
                            transform: none;
                        }

                        50% {
                            opacity: 1;
                            transform: none;
                        }

                        100% {
                            opacity: 0;
                            transform: none;
                        }
                    }

                    &::before {
                        content: '';
                        position: absolute;
                        z-index: -2;
                        left: -50%;
                        top: -50%;
                        width: 200%;
                        height: 200%;
                        background-color: rgb(148, 199, 251);
                        background-repeat: no-repeat;
                        background-position: 0 0;
                        animation: hint 1.5s linear infinite;
                    }

                    &::after {
                        content: '';
                        position: absolute;
                        z-index: -1;
                        left: 2px;
                        top: 2px;
                        width: calc(100% - 4px);
                        height: calc(100% - 4px);
                        background: rgb(255, 255, 255);
                    }
                }

            }
        }
    }
}

.flip-list-move {
    transition: transform 0.3s;
}

.flip-list-enter-from,
.flip-list-leave-to {
    opacity: 0;
    transform: translateY(30px);
    transition: transform 0.3s;
}

.flip-list-leave-active {
    position: absolute;
}

.ghost {
    opacity: 0.5;
    background: #c8ebfb;
}

.directory-active {
    color: #66b1ff;
}
</style>
