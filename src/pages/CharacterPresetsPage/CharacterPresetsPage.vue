<template>
    <div>
        <el-dialog
            v-model="showImportDialog"
            :title="t('misc.import')"
            :width="deviceIsPC ? '60%' : '90%'"
        >

            <import-block ref="fileImportBlock" accept="application/json,text/plain"></import-block>

            <template #footer>
                <el-button @click="showImportDialog = false">{{ t("misc.cancel") }}</el-button>
                <el-button type="primary" @click="handleImport">{{ t("misc.confirm") }}</el-button>
            </template>
        </el-dialog>

<!--        <el-drawer-->
<!--            :visible.sync="showArtifactsScoreDrawer"-->
<!--            title="推荐圣遗物"-->
<!--            :size="deviceIsPC ? '30%' : '100%'"-->
<!--        >-->
<!--            <el-empty v-if="artifactScoreList.length === 0"></el-empty>-->
<!--            <div v-else class="artifact-score-list-div">-->
<!--                <div-->
<!--                    v-for="(item, index) in artifactsScoreListCut"-->
<!--                    :key="index"-->
<!--                >-->
<!--                    <artifact-display-->
<!--                        :item="artifactsById[item[0]]"-->
<!--                        :show-back="true"-->
<!--                        :back-value="item[1]"-->
<!--                        class="artifact-score-item"-->
<!--                    ></artifact-display>-->
<!--                </div>-->
<!--            </div>-->
<!--        </el-drawer>-->

        <div class="toolbar">
            <el-button-group>
                <el-button
                    :icon="IconFa6SolidDownload"
                    @click="handleExportAll"
                >{{ t("presetPage.exportAll") }}</el-button>
                <el-button
                    :icon="IconFa6SolidUpload"
                    @click="handleClickImport"
                >{{ t("misc.import") }}</el-button>
            </el-button-group>
        </div>

        <div class="body" v-if="presetsLength > 0">
            <preset-item
                v-for="entry in allPresetsFlat"
                :item="entry.item"
                :name="entry.name"
                :calculate-icon="false"
                @delete="handleDeletePreset(entry.name)"
                @download="handleDownload(entry.name)"
                class="item"
                @click="handleClickPreset(entry.name)"
            ></preset-item>
        </div>
        <template v-else>
            <el-empty>
                <p
                    style="font-size: 0.9em; color: #606266"
                >
                    {{ t("presetPage.go") }}<span class="route-item" @click="$router.replace('/calculate')">{{ t("misc.calculator") }}</span>{{ t("presetPage.toCalc") }}
                </p>
            </el-empty>
        </template>
    </div>
</template>

<script setup lang="ts">
import {computed, ref, Ref} from "vue"
import {useRouter} from "vue-router"
import {checkImportFormat} from "@/utils/preset"
import {downloadString} from "@/utils/common"
import {deviceIsPC} from "@/utils/device"

import PresetItem from "@/components/display/PresetItem.vue"
import ImportBlock from "@/components/misc/ImportBlock"
// import {ElMessage} from "element-plus"
import {usePresetStore} from "@/store/pinia/preset"

import IconFa6SolidUpload from "~icons/fa6-solid/upload"
import IconFa6SolidDownload from "~icons/fa6-solid/download"
import {useI18n} from "@/i18n/i18n";


// i18n
const { t } = useI18n()

// store
const presetStore = usePresetStore()

// router
const router = useRouter()


// import and export
const showImportDialog = ref(false)
const fileImportBlock: Ref<null | InstanceType<typeof ImportBlock>> = ref(null)

function handleClickImport() {
    showImportDialog.value = true
}

function checkImportType(obj: any) {
    if (Object.prototype.hasOwnProperty.call(obj, "data")) {
        return "multi";
    } else {
        return "single"
    }
}

async function handleImport() {
    if (!fileImportBlock.value) {
        return
    }

    const text = await fileImportBlock.value.getReadPromise()

    let obj = null
    try {
        obj = JSON.parse(text)
    } catch {
        obj = null
    }

    const checkObj = checkImportFormat(obj)

    if (!checkObj) {
        ElMessage({
            message: t("presetPage.wrongFormat"),
            type: "error"
        })
    } else {
        for (let entry of obj) {
            presetStore.addOrOverwrite(entry.name, entry.item)
        }

        showImportDialog.value = false
    }
}

function handleDownload(name: string) {
    const entry = presetStore.presets.value[name]
    const temp = [entry]
    const str = JSON.stringify(temp)

    downloadString(str, "application/json", name)
}

function handleExportAll() {
    const str = JSON.stringify(presetStore.allFlat.value)

    downloadString(str, "application/json", t("misc.preset"))
}


// preset crud
const allPresetsFlat = presetStore.allFlat

const presetsLength = computed(() => {
    return allPresetsFlat.value.length
})

function handleDeletePreset(name: string) {
    presetStore.deletePreset(name)
}


// todo quick calculate
function handleClickPreset(name: string) {
    router.push({
        name: "calculate",
        params: {
            presetName: name
        }
    })
    // this.$router.push({
    //     name: "calculate",
    //     params: {
    //         presetName: name
    //     }
    // })
}

// todo score
// artifactsScoreListCut() {
//     return this.artifactScoreList.slice(0, 20)
// },

// test(name) {
//     console.log(name)
//
//     const item = getPresetEntryByName(name).item
//     if (!item) {
//         return
//     }
//
//     const characterInterface = item.character
//     const weaponInterface = item.weapon
//     const tfInterface = item.targetFunction
//
//     const artifacts = this.allArtifactsFlat.map(a => convertArtifact(a))
//
//     wasmGetArtifactsRankByCharacter(characterInterface, weaponInterface, tfInterface, artifacts).then(result => {
//         result = result.filter(item => {
//             return this.artifactsById[item[0]].level === 0
//         })
//
//         this.artifactScoreList = result
//
//         const maxValue = result.map(x => x[1]).reduce((p, c) => Math.max(p, c), 0)
//         console.log(maxValue)
//         if (maxValue > 0) {
//             for (let i = 0; i < result.length; i++) {
//                 result[i][1] /= maxValue
//             }
//         }
//
//
//         this.showArtifactsScoreDrawer = true
//     })
// }



// export default {
//     data() {
//         return {
//             showArtifactsScoreDrawer: false,
//
//             artifactScoreList: [],
//         }
//     },
// }
</script>

<style scoped lang="scss">
.body {
    display: grid;
    gap: 4px;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    grid-auto-rows: min-content;

    .item {
        width: 100%;
    }
}

.toolbar {
    margin-bottom: 20px;
}

.artifact-score-list-div {
    padding: 0 20px;

    .artifact-score-item {
        width: 100%;
        margin-bottom: 12px;
    }
}

.route-item {
    padding: 4px;
    transition: 200ms;
    border-radius: 2px;
    cursor: pointer;
    color: rgb(86,69,137);

    &:hover {
        background-color: rgb(86,69,137);
        color: white;
    }
}
</style>