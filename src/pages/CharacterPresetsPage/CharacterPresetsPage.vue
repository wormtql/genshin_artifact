<template>
    <div>
        <el-dialog
            :visible.sync="showImportDialog"
            title="导入"
            width="60%"
        >
            <import-block ref="importBlock"></import-block>

            <template #footer>
                <el-button @click="showImportDialog = false">取消</el-button>
                <el-button type="primary" @click="handleImport">确定</el-button>
            </template>
        </el-dialog>

        <el-drawer
            :visible.sync="showArtifactsScoreDrawer"
            title="推荐圣遗物"
            :size="deviceIsPC ? '30%' : '100%'"
        >
            <el-empty v-if="artifactScoreList.length === 0"></el-empty>
            <div v-else class="artifact-score-list-div">
                <div
                    v-for="(item, index) in artifactsScoreListCut"
                    :key="index"
                >
                    <artifact-display
                        :item="artifactsById[item[0]]"
                        :show-back="true"
                        :back-value="item[1]"
                        class="artifact-score-item"
                    ></artifact-display>
                </div>
            </div>
        </el-drawer>

        <div class="toolbar">
            <el-button
                icon="el-icon-download"
                size="mini"
                @click="handleExportAll"
            >导出全部</el-button>
            <el-button
                icon="el-icon-upload2"
                size="mini"
                @click="handleClickImport"
            >导入</el-button>
        </div>

        <div class="body" v-if="presetsLength > 0">
            <preset-item
                v-for="entry in allFlat"
                :item="entry.item"
                :name="entry.name"
                @delete="handleDeletePreset(entry.name)"
                @download="handleDownload(entry.name)"
                @cpu="handleQuickCalculate(entry.name)"
                class="item"
                @click="test(entry.name)"
            ></preset-item>
        </div>
        <template v-else>

            <el-empty>
                <p
                    style="font-size: 0.9em; color: #606266"
                >
                    请前往<span class="route-item" @click="$router.replace('/calculate')">计算器</span>页面添加预设
                </p>
            </el-empty>
        </template>
    </div>
</template>

<script>
import { mapGetters } from "vuex"

import { deletePreset, getPresetEntryByName, checkImportFormat, createOrUpdatePreset, upgradePresetItem } from "@util/preset"
import { downloadString } from "@util/common"
import { convertArtifact } from "@util/converter"
import { wasmGetArtifactsRankByCharacter } from "@/wasm"
import { deviceIsPC } from "@util/device"

import PresetItem from "@c/display/PresetItem"
import ImportBlock from "@c/misc/ImportBlock"
import ArtifactDisplay from "@c/display/ArtifactDisplay"

export default {
    name: "CharacterPresetsPage",
    components: {
        PresetItem,
        ImportBlock,
        ArtifactDisplay,
    },
    created() {
    },
    data() {
        return {
            showImportDialog: false,
            showArtifactsScoreDrawer: false,

            artifactScoreList: [],

            deviceIsPC
        }
    },
    methods: {
        checkImportType(obj) {
            if (Object.prototype.hasOwnProperty.call(obj, "data")) {
                return "multi";
            } else {
                return "single"
            }
        },

        handleQuickCalculate(name) {
            this.$router.push({
                name: "calculate",
                params: {
                    presetName: name
                }
            })
        },

        async handleImport() {
            const component = this.$refs.importBlock
            if (!component) {
                return
            }

            const text = await component.getReadPromise()

            let obj = null
            try {
                obj = JSON.parse(text)
            } catch {
                obj = null
            }

            const checkObj = checkImportFormat(obj)

            if (!checkObj) {
                this.$message.error("导入格式错误")
            } else {
                for (let entry of obj) {
                    createOrUpdatePreset(entry.item, entry.name)
                }

                this.showImportDialog = false
            }
        },

        handleDeletePreset(name) {
            deletePreset(name)
        },

        handleDownload(name) {
            const entry = getPresetEntryByName(name)
            const temp = [entry]
            const str = JSON.stringify(temp)

            downloadString(str, "text/plain", name)
        },

        handleExportAll() {
            const str = JSON.stringify(this.allFlat)
            
            downloadString(str, "text/plain", "预设")
        },

        handleClickImport() {
            this.showImportDialog = true
        },

        test(name) {
            console.log(name)

            const item = getPresetEntryByName(name).item
            if (!item) {
                return
            }

            const characterInterface = item.character
            const weaponInterface = item.weapon
            const tfInterface = item.targetFunction

            const artifacts = this.allArtifactsFlat.map(a => convertArtifact(a))

            wasmGetArtifactsRankByCharacter(characterInterface, weaponInterface, tfInterface, artifacts).then(result => {
                result = result.filter(item => {
                    return this.artifactsById[item[0]].level === 0
                })

                this.artifactScoreList = result

                const maxValue = result.map(x => x[1]).reduce((p, c) => Math.max(p, c), 0)
                console.log(maxValue)
                if (maxValue > 0) {
                    for (let i = 0; i < result.length; i++) {
                        result[i][1] /= maxValue
                    }
                }


                this.showArtifactsScoreDrawer = true
            })
        }
    },
    computed: {
        ...mapGetters("presets", ["allFlat"]),
        ...mapGetters("artifacts", {
            "allArtifactsFlat": "allFlat",
            "artifactsById": "artifactsById"
        }),

        presetsLength() {
            return this.allFlat.length
        },

        artifactsScoreListCut() {
            return this.artifactScoreList.slice(0, 20)
        },
    }
}
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