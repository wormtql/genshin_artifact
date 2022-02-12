<template>
    <div>
        <el-dialog
            title="选择圣遗物"
            width="80%"
            :visible.sync="showSelectArtifactDialog"
        >
            <select-artifact
                :position="selectArtifactSlot"
                @select="handleSelectArtifact"
            ></select-artifact>
        </el-dialog>

        <el-dialog
            :visible.sync="showDamageAnalysisDialog"
            title="伤害构成"
            width="80%"
        >
            <damage-analysis
                ref="damageAnalysis"
            ></damage-analysis>
        </el-dialog>

        <el-dialog
            :visible.sync="showSelectBuffDialog"
            title="选择BUFF"
            width="60%"
        >
            <select-buff
                @select="handleSelectBuff"
            ></select-buff>
        </el-dialog>

        <div class="outer-container">
            <div>
                <el-breadcrumb>
                    <el-breadcrumb-item>裏</el-breadcrumb-item>
                </el-breadcrumb>
                <el-divider></el-divider>
            </div>
            
            <el-row class="big-container">
                <el-col class="left-container" :span="6">
                    <div class="config-character">
                        <img :src="characterSplash" class="character-splash" />
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
                                        style="flex: 1; display: blockl width: unset"
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
                            <div style="display: flex; gap: 12px">
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

                    <div class="config-target-function">
                        <p class="common-title">目标函数</p>
                        <div class="my-button-list" style="margin-bottom: 12px">
                            <my-button-1 icon="el-icon-caret-right" title="开始计算"
                                @click="handleOptimizeArtifact"
                            ></my-button-1>
                            <my-button-1 icon="el-icon-s-operation" title="设置"
                                @click="handleOptimizeArtifact"
                            ></my-button-1>
                        </div>
                        <select-target-function
                            v-model="targetFunctionName"
                            :character-name="characterName"
                        ></select-target-function>
                        <div class="target-function-config" v-if="targetFunctionNeedConfig"
                            style="margin-top: 12px"
                        >
                            <item-config
                                v-model="targetFunctionConfig"
                                :item-name="targetFunctionName"
                                :configs="targetFunctionConfigConfig"
                            ></item-config>
                        </div>

                        <div class="target-function-detail">
                            <div class="detail-left">
                                <img :src="targetFunctionBadge" />
                            </div>
                            <div class="detail-right">
                                <p
                                    class="target-function-description"
                                >{{ targetFunctionDescription }}</p>
                            </div> 
                        </div>

                        <div v-if="optimizationResults.length > 0"
                            style="margin-top: 12px"
                        >
                            <el-alert
                                title="共计算100组圣遗物搭配"
                                type="success"
                                style="margin-bottom: 12px"
                            ></el-alert>
                            <el-input-number
                                :value="optimizationResultIndex"
                                @input="handleUseNthOptimizationResult"
                                :min="1"
                                :max="optimizationResults.length"
                                size="small"
                                style="width: 100%"
                            ></el-input-number>
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
                        <div class="buffs">
                            <buff-item
                                v-for="buff in buffs"
                                :key="buff.id"
                                :buff="buff"
                                :buff-config.sync="buff.config"
                                @delete="handleClickDeleteBuff(buff.id)"
                                @toggle="handleClickToggleBuff(buff.id)"
                            ></buff-item>
                        </div>
                    </div>
                </el-col>

                <el-col :span="12" class="middle-container">
                    <p class="common-title">圣遗物</p>
                    <div class="artifacts">
                        <div
                            v-for="(id, index) in artifactIds"
                            :key="id"
                            class="artifact-item-or-button"
                        >
                            <artifact-display
                                v-if="artifactItems[index]"
                                :item="artifactItems[index]"
                                selectable
                                buttons
                                delete-button
                                @delete="handleRemoveArtifact(index)"
                                @toggle="handleToggleArtifact(id)"
                                @click="handleGotoSelectArtifact(index)"
                                class="artifact-display"
                            ></artifact-display>
                            <add-button
                                v-else
                                @click="handleGotoSelectArtifact(index)"
                                class="add-button"
                                style="height: 125px"
                            ></add-button>
                        </div>
                    </div>

                    <el-divider></el-divider>

                    <p class="common-title">伤害计算</p>
                    <div class="my-button-list" style="margin-bottom: 12px">
                        <my-button-1 icon="el-icon-s-operation" title="明细"
                            @click="handleDisplayAnalysis"
                        ></my-button-1>
                    </div>
                    <div v-if="characterNeedSkillConfig" style="margin-bottom: 16px;">
                        <item-config
                            v-model="characterSkillConfig"
                            :item-name="characterName"
                            :configs="characterSkillConfigConfig"
                        ></item-config>
                    </div>
                    <div class="damage-analysis-div">
                        <select-character-skill
                            v-model="characterSkillIndex"
                            :character-name="characterName"
                            style="margin-bottom: 16px"
                        ></select-character-skill>
                        <damage-panel
                            :analysis-from-wasm="characterDamageAnalysis"
                        ></damage-panel>
                    </div>
                </el-col>

                <el-col :span="6" class="right-container">
                    <div class="common-title">面板</div>
                    <attribute-panel
                        :attribute="attributeFromWasm"
                    ></attribute-panel>
                </el-col>
            </el-row>
        </div>
    </div>
</template>

<script>
import { mapGetters } from "vuex"

import { convertArtifact } from "@util/converter"
import { characterData } from "@character"
import { weaponData } from "@weapon"
import { targetFunctionData } from "@targetFunction"
import { buffData } from "@buff"

import SelectArtifact from "@c/select/SelectArtifact"
import SelectCharacter from "@c/select/SelectCharacter"
import SelectCharacterLevel from "@c/select/SelectCharacterLevel"
import SelectWeapon from "@c/select/SelectWeapon"
import SelectWeaponLevel from "@c/select/SelectWeaponLevel"
import SelectTargetFunction from "@c/select/SelectTargetFunction"
import SelectCharacterSkill from "@c/select/SelectCharacterSkill"
import SelectBuff from "@c/select/SelectBuff"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import AddButton from "@c/misc/AddButton"
import DamagePanel from "./DamagePanel"
import MyButton1 from "@c/button/MyButton1"
import DamageAnalysis from "@c/display/DamageAnalysis"
import AttributePanel from "@c/display/AttributePanel"
import ItemConfig from "@c/config/ItemConfig"
import BuffItem from "./BuffItem"

const artifactConfig = {
    "config_archaic_petra": {
        "element": "Electro",
        rate: 0,
    },
    "config_berserker": { rate: 0 },
    "config_blizzard_strayer": { critical_bonus: 0 },
    "config_bloodstained_chivalry": { rate: 0 },
    "config_brave_heart": { rate: 0 },
    "config_crimson_witch_of_flames": { level: 0 },
    "config_heart_of_depth": { rate: 0 },
    "config_husk_of_opulent_dreams": { level: 0 },
    "config_instructor": { rate: 0 },
    "config_lavawalker": { rate: 0 },
    "config_martial_artist": { rate: 0 },
    "config_noblesse_oblige": { rate: 0 },
    "config_pale_flame": { avg_level: 0, full_rate: 0 },
    "config_retracing_bolide": { rate: 0 },
    "config_shimenawas_reminiscence": { rate: 0 },
    "config_tenacity_of_the_millelith": { rate: 0 },
    "config_thundersoother": { rate: 0 },
}

export default {
    name: "NewArtifactPlanPage",
    components: {
        SelectArtifact,
        SelectCharacter,
        SelectCharacterLevel,
        SelectCharacterSkill,
        SelectWeapon,
        SelectWeaponLevel,
        SelectTargetFunction,
        SelectBuff,
        ArtifactDisplay,
        AddButton,
        DamagePanel,
        MyButton1,
        DamageAnalysis,
        AttributePanel,
        ItemConfig,
        BuffItem,
    },
    created() {
        // this.characterData = characterData
    },
    data () {
        return {
            characterName: "Amber",
            characterLevel: "90",
            characterConfig: "NoConfig",
            characterSkillConfig: "NoConfig",
            characterSkillIndex: 0,
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

            targetFunctionName: "AmberDefault",
            targetFunctionConfig: "NoConfig",
            optimizationResults: [],
            optimizationResultIndex: 0,

            artifactIds: [-1, -1, -1, -1, -1],

            showSelectArtifactDialog: false,
            selectArtifactSlot: "any",

            showDamageAnalysisDialog: false,
            showSelectBuffDialog: false,
        }
    },
    computed: {
        ...mapGetters({
            artifactsById: "artifacts/artifactsById"
        }),

        // character
        characterLevelNumber() {
            return parseInt(this.characterLevel)
        },

        characterAscend() {
            return this.characterLevel.includes("+")
        },

        characterSplash() {
            const data = characterData[this.characterName]
            return data.splash
        },

        characterNeedConfig() {
            let temp = characterData[this.characterName].config
            return temp && temp.length > 0
        },

        characterConfigConfig() {
            return characterData[this.characterName].config
        },

        characterNeedSkillConfig() {
            let temp = characterData[this.characterName].configSkill
            return temp && temp.length > 0
        },

        characterSkillConfigConfig() {
            return characterData[this.characterName].configSkill
        },

        characterWeaponType() {
            const item = characterData[this.characterName]
            return item ? item.weapon : "Bow"
        },

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

        characterSkillInterface() {
            return {
                index: this.characterSkillIndex,
                config: this.characterSkillConfig
            }
        },

        characterDamageAnalysis() {
            const temp = this.$mona.CalculatorInterface.get_damage_analysis(this.damageAnalysisWasmInterface)
            // console.log(temp)
            return temp
        },

        // weapon
        weaponLevelNumber() {
            return parseInt(this.weaponLevel)
        },

        weaponAscend() {
            return this.weaponLevel.includes("+")
        },
        
        weaponSplash() {
            const data = weaponData[this.weaponName]
            return data.gacha ?? data.url ?? data.tn
        },

        weaponNeedConfig() {
            // return !!weaponConfig[this.weaponName]
            return !!weaponData[this.weaponName].configs
        },

        weaponConfigConfig() {
            return weaponData[this.weaponName].configs
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

        // tf
        targetFunctionBadge() {
            return targetFunctionData[this.targetFunctionName].badge
        },

        targetFunctionDescription() {
            return targetFunctionData[this.targetFunctionName].description
        },

        targetFunctionInterface() {
            return {
                name: this.targetFunctionName,
                params: this.targetFunctionConfig
            }
        },

        targetFunctionNeedConfig() {
            const temp = targetFunctionData[this.targetFunctionName].config
            return temp && temp.length > 0
        },

        targetFunctionConfigConfig() {
            return targetFunctionData[this.targetFunctionName].config
        },

        // artifact
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
                    // console.log(artifact)
                    if (artifact && !artifact.omit) {
                        const wasmArtifact = convertArtifact(artifact)
                        temp.push(wasmArtifact)
                    }
                }
            }
            // console.log(temp)
            return temp
        },

        damageAnalysisWasmInterface() {
            // console.log(this.weaponInterface);
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                buffs: this.buffsInterface,
                artifacts: this.artifactWasmFormat,
                artifact_config: null, // todo
                skill: this.characterSkillInterface
            }
        },

        getAttributeWasmInterface() {
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                buffs: this.buffsInterface,
                artifacts: this.artifactWasmFormat,
                artifact_config: null, // todo
            }
        },

        optimizeArtifactWasmInterface() {
            const artifacts = this.getAllArtifactsWasmFormat()
            const artifacts16 = artifacts.filter(x => x.level >= 16)

            return {
                artifacts: artifacts16,
                character: this.characterInterface,
                weapon: this.weaponInterface,
                target_function: this.targetFunctionInterface,
                constraint: null, // todo
                buffs: this.buffsInterface
            }
        },

        

        attributeFromWasm() {
            return this.$mona.CommonInterface.get_attribute(this.getAttributeWasmInterface)
        },


        // buff
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
        }
    },
    methods: {
        handleOptimizeArtifact() {
            const interfac = this.optimizeArtifactWasmInterface
            const start = new Date()

            const loading = this.$loading({
                lock: true,
                text: "莫娜占卜中"
            })
            const worker = new Worker(new URL("@worker/optimize_artifact.js", import.meta.url))

            const closeLoading = () => {
                loading.close()
            }

            const closeWorker = () => {
                worker.terminate()
            }

            // max calc time: 2min
            const timer = setTimeout(() => {
                closeLoading()
                closeWorker()
                this.$message({
                    message: "计算超时",
                    type: "error"
                })
            }, 120000)
            
            worker.onmessage = e => {
                if (e.data.type === "ready") {
                    worker.postMessage({
                        interfac
                    })
                } else {
                    const results = e.data.data.results
                    const end = new Date()

                    console.log(`time: ${(end - start) / 1000}s`)
                    // console.log(results)

                    this.optimizationResults = results
                    clearTimeout(timer)
                    closeLoading()
                    this.handleUseNthOptimizationResult(1)
                    closeWorker()
                }
            }
        },

        handleUseNthOptimizationResult(n) {
            const result = this.optimizationResults[n - 1]
            const m = x => {
                if (x !== null) {
                    return x
                } else {
                    return -1
                }
            }

            let temp = []
            temp.push(m(result.flower))
            temp.push(m(result.feather))
            temp.push(m(result.sand))
            temp.push(m(result.goblet))
            temp.push(m(result.head))

            this.artifactIds = temp

            this.optimizationResultIndex = n
        },

        handleChangeCharacter(name) {
            this.characterName = name
        },

        handleGotoSelectArtifact(index) {
            const map = ["flower", "feather", "sand", "cup", "head"]
            const slotName = map[index]
            this.showSelectArtifactDialog = true
            this.selectArtifactSlot = slotName
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

        handleRemoveArtifact(index) {
            this.$set(this.artifactIds, index, -1)
        },

        handleToggleArtifact(id) {
            this.$store.commit("artifacts/toggleById", { id })
        },

        handleDisplayAnalysis() {
            this.showDamageAnalysisDialog = true;
            this.$nextTick(() => {
                const component = this.$refs["damageAnalysis"]
                component.setValue(this.characterDamageAnalysis)
            })
        },

        getAllArtifactsWasmFormat() {
            return this.$store.getters["artifacts/allFlat"].map(x => convertArtifact(x))
        },

        // BUFF
        handleClickAddBuff() {
            this.showSelectBuffDialog = true
        },

        handleSelectBuff(name) {
            this.showSelectBuffDialog = false
            this.addBuff(name)
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
    watch: {
        weaponName(newName) {
            const hasConfig = !!weaponData[newName]?.configs
            if (hasConfig) {
                const configs = weaponData[newName].configs

                let defaultConfig = {}
                for (let config of configs) {
                    defaultConfig[config.name] = config.default
                }

                this.weaponConfig = {
                    [newName]: defaultConfig
                }
            } else {
                this.weaponConfig = "NoConfig"
            }
        },

        characterName(newName) {
            const hasConfigData = characterData[newName].config.length > 0;
            const hasConfigSkill = characterData[newName].configSkill.length > 0;

            if (hasConfigData) {
                const configs = characterData[newName].config

                let defaultConfig = {}
                for (let c of configs) {
                    defaultConfig[c.name] = c.default
                }
                this.characterConfig = {
                    [newName]: defaultConfig
                }
            } else {
                this.characterConfig = "NoConfig"
            }

            if (hasConfigSkill) {
                let defaultConfig = {}
                for (let c of characterData[newName].configSkill) {
                    defaultConfig[c.name] = c.default
                }
                this.characterSkillConfig = {
                    [newName]: defaultConfig
                }
            } else {
                this.characterSkillConfig = "NoConfig"
            }
        },

        targetFunctionName(newName) {
            const hasConfig = targetFunctionData[newName].config.length > 0

            if (hasConfig) {
                let defaultConfig = {}
                for (let c of targetFunctionData[newName].config) {
                    defaultConfig[c.name] = c.default
                }
                this.targetFunctionConfig = {
                    [newName]: defaultConfig
                }
            } else {
                this.targetFunctionConfig = "NoConfig"
            }
        }
    }
};

</script>

<style lang="scss" scoped>
.outer-container {
    display: flex;
    flex-direction: column;
    height: 100%;
}

.big-container {
    // display: flex;
    // gap: 16px;
    flex: 1;
    overflow-y: hidden;
    // overflow-x: visible;

    .left-container, .middle-container, .right-container {
        height: 100%;
        overflow-x: visible;
        overflow-y: auto;

        &::-webkit-scrollbar {
            width: 4px;
        }

        &::-webkit-scrollbar-track {
            background: rgb(236, 245, 255);
            border-radius: 2px;
        }

        &::-webkit-scrollbar-thumb {
            background: #d4d4d4;
        }
    }

    .left-container {
        // flex: 1;
        padding-right: 12px;
        
    }

    .middle-container {
        // flex: 2;
        padding-left: 12px;
        padding-right: 12px;

        // overflow-y: auto;
        // overflow-x: hidden;
    }

    .right-container {
        // flex: 1;
        padding-left: 12px;
        // overflow-y: auto;
        // overflow-x: hidden;
    }
}

.middle-container {
    .artifacts {
        display: flex;
        gap: 12px;
        flex-wrap: wrap;
        align-items: center;

        .artifact-item-or-button {
            
        }
    }
}

.config-character {
    // padding: 16px;
    // position: relative;

    .character-splash {
        position: absolute;
        width: 400px;
        opacity: 0.3;
        // left: -150px;
        // right: -100px;
        // top: -32px;
        pointer-events: none;
    }

    .character-extra-config {
        margin-top: 16px;
        // border-left: #222222 solid 3px;
        // padding: 8px;
        // background: #12345611;
        // border-top-right-radius: 3px;
        // border-bottom-right-radius: 3px;
    }

    .config-character-skill {
        .skill-div {
            display: flex;
            align-items: center;
            gap: 4px;
        }
    }

    .select-character {
        // padding-left: 50px;
    }
}

.config-weapon {
    // padding: 16px;
    // position: relative;
    margin-top: 16px;

    .weapon-splash {
        position: absolute;
        right: -25px;
        // top: 30px;
        width: 150px;
        opacity: 0.3;
    }

    .select-weapon {
        // padding-left: 250px;
    }

    .weapon-extra-config {
        margin-top: 16px;
        // border-left: #222222 solid 3px;
        // padding: 8px;
        // background: #12345611;
        // border-top-right-radius: 3px;
        // border-bottom-right-radius: 3px;
    }
}

.config-target-function {
    .target-function-detail {
        display: flex;
        align-items: top;
        margin-top: 16px;

        .detail-left {
            width: 64px;
            margin-right: 16px;
            
            img {
                height: 64px;
                width: 64px;
                border-radius: 50%;
            }
        }

        .detail-right {
            .target-function-description {
                margin: 0;
                font-size: 14px;
                color: #555555;
            }
        }
    }
}

.common-title {
    font-size: 16px;
    font-weight: bold;
    margin: 0 0 12px 0;
    color: #555555;
}

.common-title2 {
    font-size: 12px;
    color: #666666;
}
</style>