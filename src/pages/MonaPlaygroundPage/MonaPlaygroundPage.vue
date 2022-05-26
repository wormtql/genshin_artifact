<template>
    <el-row>
        <el-dialog
            :visible.sync="showSelectBuffDialog"
            title="选择BUFF"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <select-buff
                @select="handleSelectBuff"
            ></select-buff>
        </el-dialog>

        <el-dialog
            title="选择圣遗物"
            :width="deviceIsPC ? '80%' : '90%'"
            :visible.sync="showSelectArtifactDialog"
        >
            <select-artifact
                :position="selectArtifactSlot"
                @select="handleSelectArtifact"
            ></select-artifact>
        </el-dialog>

        <el-col :sm="24" :md="6" class="left mona-scroll-hidden" :style="deviceIsPC ? { paddingRight: '16px' } : {}">
            <div class="config-character">
                <img :src="characterSplash" class="character-splash" alt="角色" />
                <div class="select-character">
                    <p class="common-title">角色</p>
                    <div style="display: flex; gap: 12px">
                        <select-character
                            v-model="characterName"
                            style="flex: 1"
                        ></select-character>
                        <select-character-level
                            v-model="characterLevel"
                            style="flex: 1"
                        ></select-character-level>
                    </div>

                    <div class="config-character-skill">
                        <h3 class="common-title2">技能</h3>
                        <div class="skill-div">
                            <el-input-number
                                size="mini"
                                controls-position="right"
                                v-model="characterSkill1"
                                :min="1"
                                :max="15"
                                style="flex: 1; display: block; width: unset"
                            ></el-input-number>
                            <el-input-number
                                size="mini"
                                controls-position="right"
                                v-model="characterSkill2"
                                :min="1"
                                :max="15"
                                style="flex: 1; display: block; width: unset"
                            ></el-input-number>
                            <el-input-number
                                size="mini"
                                controls-position="right"
                                v-model="characterSkill3"
                                :min="1"
                                :max="15"
                                style="flex: 1; display: block; width: unset"
                            ></el-input-number>
                        </div>
                    </div>

                    <div class="config-character-constellation">
                        <h3 class="common-title2">命之座</h3>
                        <el-input-number
                            size="mini"
                            controls-position="right"
                            v-model="characterConstellation"
                            :min="0"
                            :max="6"
                        ></el-input-number>
                    </div>

                    <div class="character-extra-config" v-if="characterNeedConfig">
                        <item-config
                            v-model="characterConfig"
                            :item-name="characterName"
                            :configs="characterConfigConfig"
                        ></item-config>
                    </div>
                </div>
            </div>

            <el-divider></el-divider>

            <div class="config-weapon">
                <!-- <img :src="weaponSplash" class="weapon-splash" /> -->
                <div class="select-weapon">
                    <p class="common-title">武器</p>

                    <div style="display: flex; gap: 12px; margin-bottom: 8px">
                        <select-weapon
                            :type="characterWeaponType"
                            v-model="weaponName"
                            style="flex: 1"
                        ></select-weapon>
                        <select-weapon-level
                            v-model="weaponLevel"
                            style="flex: 1"
                        ></select-weapon-level>
                    </div>

                    <weapon-display :weapon-name="weaponName"></weapon-display>

                    <div class="config-weapon-refine">
                        <h3 class="common-title2">精炼</h3>
                        <el-input-number
                            size="mini"
                            controls-position="right"
                            v-model="weaponRefine"
                            :min="1"
                            :max="5"
                        ></el-input-number>
                    </div>

                    <div class="weapon-extra-config" v-if="weaponNeedConfig">
                        <item-config
                            v-model="weaponConfig"
                            :item-name="weaponName"
                            :configs="weaponConfigConfig"
                        ></item-config>
                    </div>
                </div>
            </div>

            <el-divider></el-divider>

            <div class="config-buff">
                <p class="common-title">BUFF</p>
                <div class="buff-tool" style="margin-bottom: 12px">
                    <my-button-1 icon="el-icon-plus" title="添加BUFF"
                                 @click="handleClickAddBuff"
                    ></my-button-1>
                </div>
                <div class="buffs" v-if="buffs.length > 0">
                    <buff-item
                        v-for="buff in buffs"
                        :key="buff.id"
                        :buff="buff"
                        :buff-config.sync="buff.config"
                        @delete="handleClickDeleteBuff(buff.id)"
                        @toggle="handleClickToggleBuff(buff.id)"
                    ></buff-item>
                </div>
                <div v-else>
                    <el-empty description="无BUFF"></el-empty>
                </div>
            </div>

            <el-divider></el-divider>

            <div class="config-artifact">
                <p class="common-title">圣遗物</p>

                <div class="artifacts">
                    <div
                        v-for="(id, index) in artifactIds"
                        :key="index"
                        class="artifact-item-or-button"
                    >
                        <artifact-display
                            v-if="artifactItems[index]"
                            :item="artifactItems[index]"
                            selectable
                            :buttons="true"
                            :delete-button="true"
                            @delete="handleRemoveArtifact(index)"
                            @toggle="handleToggleArtifact(id)"
                            @click="handleGotoSelectArtifact(index)"
                            class="artifact-display"
                        ></artifact-display>
                        <add-button
                            v-else
                            @click="handleGotoSelectArtifact(index)"
                            class="add-button"
                        ></add-button>
                    </div>
                </div>

            </div>
        </el-col>
        <el-col :sm="24" :md="18" class="right mona-scroll-hidden">
            <div class="editor-container">
                <mona-monaco-editor ref="editor"></mona-monaco-editor>
            </div>

            <div class="bottom">
                <div class="tool-bar">
                    <my-button1 icon="el-icon-caret-right" title="运行" @click="handleClickRun()"></my-button1>
                    <my-button1 icon="el-icon-delete-solid" title="清除输出" @click="handleClickClearOutput"></my-button1>
                </div>

                <div class="output mona-scroll-hidden">
                    <p v-for="output in outputs" class="output-row">{{ output }}</p>
                </div>
            </div>
        </el-col>
    </el-row>
</template>

<script>
import MonaMonacoEditor from "@c/MonaMonacoEditor"
import ItemConfig from "@c/config/ItemConfig"
import SelectCharacter from "@c/select/SelectCharacter"
import SelectWeapon from "@c/select/SelectWeapon"
import SelectCharacterLevel from "@c/select/SelectCharacterLevel"
import SelectWeaponLevel from "@c/select/SelectWeaponLevel"
import WeaponDisplay from "@c/display/WeaponDisplay"
import MyButton1 from "@c/button/MyButton1"
import BuffItem from "@page/NewArtifactPlanPage/BuffItem"
import SelectBuff from "@c/select/SelectBuff"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import AddButton from "@c/misc/AddButton"
import SelectArtifact from "@c/select/SelectArtifact"

import {deviceIsPC} from "@util/device"
import {buffData} from "@buff";
import {characterData} from "@character"
import {weaponData} from "@weapon"
import {newDefaultArtifactConfigForWasm} from "@util/artifacts"
import { mapGetters } from "vuex"
import {artifactsData} from "@artifact";
import {convertArtifact, convertArtifactName} from "@util/converter";
import {toSnakeCase} from "@util/common";

// const original_source = `dmg x = KamisatoAyaka.Normal1({ after_dash: true })
// print(x.normal.e)`

export default {
    name: "MonaPlaygroundPage",
    components: {
        MonaMonacoEditor,
        ItemConfig,
        SelectCharacter,
        SelectWeapon,
        SelectCharacterLevel,
        SelectWeaponLevel,
        WeaponDisplay,
        MyButton1,
        BuffItem,
        SelectBuff,
        ArtifactDisplay,
        AddButton,
        SelectArtifact
    },
    data() {
        return {
            characterName: "Amber",
            characterLevel: "90",
            characterConfig: "NoConfig",
            characterSkill1: 8,
            characterSkill2: 8,
            characterSkill3: 8,
            characterConstellation: 0,

            weaponName: "PolarStar",
            weaponLevel: "90",
            weaponRefine: 1,
            weaponConfig: {
                "PolarStar": {
                    stack: 1
                }
            },

            buffs: [],

            artifactIds: [-1, -1, -1, -1, -1],
            artifactSingleConfig: null,
            selectArtifactSlot: "any",
            // artifactConfig: newDefaultArtifactConfigForWasm(),

            showSelectBuffDialog: false,
            showSelectArtifactDialog: false,

            deviceIsPC,

            outputs: [],
        }
    },
    methods: {
        appendOutput(s) {
            this.outputs.push(s)
        },

        handleClickRun() {
            let x = this.runInputInterface
            let artifacts = this.artifactWasmFormat
            let source = this.$refs["editor"].getValue().trim()
            // console.log(source)
            // console.log(artifacts)
            // console.log(x)

            let ret = this.$mona.DSLInterface.run(source, x, artifacts)

            if (ret.is_error) {
                this.appendOutput("[error]")
                for (const item of ret.error_msg.split("\n")) {
                    this.appendOutput(item)
                }
            } else {
                this.appendOutput("[success]")
                for (const item of ret.output.split("\n")) {
                    this.appendOutput(item)
                }
                // this.appendOutput(ret.output)
            }
            // console.log(ret)
        },

        handleClickClearOutput() {
            this.outputs = []
        },

        handleSelectArtifact(id) {
            const map = {
                "flower": 0,
                "feather": 1,
                "sand": 2,
                "cup": 3,
                "head": 4
            }
            const index = map[this.selectArtifactSlot]
            this.$set(this.artifactIds, index, id)

            this.showSelectArtifactDialog = false
        },

        handleGotoSelectArtifact(index) {
            const map = ["flower", "feather", "sand", "cup", "head"]
            const slotName = map[index]
            this.showSelectArtifactDialog = true
            this.selectArtifactSlot = slotName
        },

        handleRemoveArtifact(index) {
            this.$set(this.artifactIds, index, -1)
        },

        handleToggleArtifact(id) {
            this.$store.commit("artifacts/toggleById", { id })
        },

        handleClickAddBuff() {
            this.showSelectBuffDialog = true
        },

        handleClickDeleteBuff(id) {
            const index = this.buffs.findIndex(e => e.id === id)
            this.$delete(this.buffs, index)
        },

        handleClickToggleBuff(id) {
            const index = this.buffs.findIndex(e => e.id === id)
            const v = this.buffs[index].lock
            this.$set(this.buffs[index], "lock", !v)
        },

        handleSelectBuff(name) {
            this.showSelectBuffDialog = false
            this.addBuff(name)
        },

        addBuff(name) {
            const data = buffData[name]
            let defaultConfig = {}
            for (let c of data.config) {
                defaultConfig[c.name] = c.default
            }

            let config;
            if (data.config.length === 0) {
                config = "NoConfig"
            } else {
                config = {
                    [name]: defaultConfig
                }
            }

            this.buffs.push({
                name,
                config,
                id: Math.floor(Math.random() * 1e9),
                lock: false
            })
        },
    },
    computed: {
        ...mapGetters({
            artifactsById: "artifacts/artifactsById",
            allArtifactsFlat: "artifacts/allFlat"
        }),

        characterInterface() {
            let i = {
                name: this.characterName,
                level: this.characterLevelNumber,
                ascend: this.characterAscend,
                constellation: this.characterConstellation,
                skill1: this.characterSkill1 - 1,
                skill2: this.characterSkill2 - 1,
                skill3: this.characterSkill3 - 1,
                params: this.characterConfig
            }
            return i
        },

        weaponInterface() {
            return {
                name: this.weaponName,
                level: this.weaponLevelNumber,
                ascend: this.weaponAscend,
                refine: this.weaponRefine,
                params: this.weaponConfig
            }
        },

        buffsUnlocked() {
            return this.buffs.filter(e => !e.lock)
        },

        buffsInterface() {
            let temp = []
            for (let buff of this.buffsUnlocked) {
                temp.push({
                    name: buff.name,
                    config: buff.config
                })
            }
            return temp
        },

        artifactConfigForCalculator() {
            let base = newDefaultArtifactConfigForWasm()

            if (this.artifactNeedConfig4) {
                let name = this.artifactConfig4ItemName
                base[name] = this.artifactSingleConfig[name]
            }

            // console.log(base)
            return base
        },

        runInputInterface() {
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                buffs: this.buffsInterface,
                artifact_config: this.artifactConfigForCalculator,
                enemy: null, // todo
            }
        },

        characterSplash() {
            const data = characterData[this.characterName]
            return data.splash
        },

        characterLevelNumber() {
            return parseInt(this.characterLevel)
        },

        characterAscend() {
            return this.characterLevel.includes("+")
        },

        characterWeaponType() {
            const item = characterData[this.characterName]
            return item ? item.weapon : "Bow"
        },

        weaponNeedConfig() {
            // return !!weaponConfig[this.weaponName]
            return !!weaponData[this.weaponName].configs
        },

        weaponConfigConfig() {
            return weaponData[this.weaponName].configs
        },

        weaponLevelNumber() {
            return parseInt(this.weaponLevel)
        },

        weaponAscend() {
            return this.weaponLevel.includes("+")
        },

        artifactItems() {
            let temp = []
            for (let id of this.artifactIds) {
                if (this.artifactsById[id]) {
                    temp.push(this.artifactsById[id])
                } else {
                    temp.push(null)
                }
            }
            return temp
        },

        artifactWasmFormat() {
            let temp = []
            for (let id of this.artifactIds) {
                if (id !== -1) {
                    const artifact = this.$store.getters["artifacts/artifactsById"][id]
                    if (artifact && !artifact.omit) {
                        const wasmArtifact = convertArtifact(artifact)
                        temp.push(wasmArtifact)
                    }
                }
            }
            return temp
        },

        artifactNeedConfig4() {
            // console.log(this.artifactSetCount)
            for (let setName in this.artifactSetCount) {
                const count = this.artifactSetCount[setName]
                if (count >= 4) {
                    const data = artifactsData[setName]
                    // console.log(data)
                    if (data.config4 && data.config4.length > 0) {
                        return setName
                    }
                }
            }

            return null
        },

        artifactSetCount() {
            let temp = {}
            for (let artifact of this.artifactItems) {
                if (!artifact) {
                    continue
                }
                const setName = artifact.setName
                if (!Object.prototype.hasOwnProperty.call(temp, setName)) {
                    temp[setName] = 0
                }
                temp[setName] += 1
            }
            return temp
        },

        artifactConfig4ItemName() {
            const setNameWasm = convertArtifactName(this.artifactNeedConfig4)
            return `config_${toSnakeCase(setNameWasm)}`
        },
    }
}
</script>

<style scoped lang="scss">
.editor-container {
    height: calc(100% - 300px)
}

.left, .right {
    height: calc(100vh - 2 * 24px);
    //overflow: ;
}

.left {
    //background-color: rgb(30,30,30);
}

.bottom {
    background-color: rgb(30,30,30);
    height: 300px;
}

.config-character {
    .character-splash {
        position: absolute;
        width: 400px;
        opacity: 0.3;
        pointer-events: none;
    }

    .character-extra-config {
        margin-top: 16px;
    }

    .config-character-skill {
        .skill-div {
            display: flex;
            align-items: center;
            gap: 4px;
        }
    }
}

.config-weapon {
    margin-top: 16px;

    .weapon-splash {
        position: absolute;
        right: -25px;
        width: 150px;
        opacity: 0.3;
    }

    .select-weapon {
    }

    .weapon-extra-config {
        margin-top: 16px;
    }
}

.artifacts {
    gap: 4px;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    grid-auto-rows: minmax(64px, max-content);

    .artifact-item-or-button {
        .add-button {
            width: 100%;
            height: 100%;
        }
        .artifact-display {
            width: 100%;
            box-sizing: border-box;
        }
    }
}

.common-title {
    font-size: 0.9rem;
    font-weight: bold;
    margin: 0 0 1.5vh 0;
    color: #555555;
}

.common-title2, .constraint-title {
    font-size: 0.7rem;
    color: #666666;
    border-left: 2px solid #409EFF;
    padding-left: 0.5vw;
}

.output {
    padding: 12px;
    font-family: Consolas;
    font-size: 12px;
    height: calc(300px - 49px);
    box-sizing: border-box;

    .output-row {
        color: white;
        margin: 0;
    }
}

.tool-bar {
    height: 48px;
    display: flex;
    align-items: center;
    border-top: 1px solid rgb(68,68,68);
}

</style>