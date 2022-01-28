<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>裏</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

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

        <div class="top-toolbar">
            <el-button
                size="small"
                type="primary"
                style="margin-bottom: 16px"
            >开始计算</el-button>
        </div>

        <div class="big-container">
            <div class="left-container">
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
                        <div class="character-extra-config" v-if="characterNeedConfig">
                            <component
                                :is="characterConfigComponent"
                                ref="characterConfigComponent"
                                v-model="characterConfig"
                            ></component>
                        </div>
                    </div>
                </div>

                <el-divider></el-divider>

                <div class="config-weapon">
                    <img :src="weaponSplash" class="weapon-splash" />
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
                        <div class="weapon-extra-config" v-if="weaponNeedConfig">
                            <component
                                :is="weaponConfigComponent"
                                ref="weaponConfigComponent"
                                v-model="weaponConfig"
                            ></component>
                        </div>
                    </div>
                </div>

                <el-divider></el-divider>

                <div class="config-target-function">
                    <p class="common-title">目标函数</p>
                    <div class="my-button-list" style="margin-bottom: 12px">
                        <my-button-1 icon="el-icon-s-operation" title="Optimize"
                            @click="handleOptimizeArtifact"
                        ></my-button-1>
                    </div>
                    <select-target-function
                        v-model="targetFunctionName"
                        :character-name="characterName"
                    ></select-target-function>
                    <div class="target-function-detail">
                        <div class="detail-left">
                            <img :src="targetFunctionBadge" />
                        </div>
                        <div class="detail-right">
                            <p
                                v-for="(description, index) in targetFunctionDescription"
                                :key="index"
                                class="target-function-description"
                            >{{ description }}</p>
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
                            v-model="optimizationResultIndex"
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
                    <p class="common-title">Buff</p>
                    <div class="buff-tool" style="margin-bottom: 12px">
                        <el-button
                            size="small"
                            type="primary"
                        >添加BUFF</el-button>
                    </div>
                    <div class="buffs">

                    </div>
                </div>
            </div>

            <div class="middle-container">
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
                    <component
                        :is="characterSkillConfigComponent"
                        v-model="characterSkillConfig"
                    ></component>
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
            </div>

            <div class="right-container">
                <div class="common-title">面板</div>
                <attribute-panel
                    :attribute="attributeFromWasm"
                ></attribute-panel>
            </div>
        </div>
    </div>
</template>

<script>
import { mapGetters } from "vuex"

import { convertArtifact } from "@util/converter"
import { characterData, characterConfig, characterSkillConfig } from "@character"
import { weaponConfig, weaponData } from "@weapon"
import { targetFunctionData } from "@targetFunction"

import SelectArtifact from "@c/select/SelectArtifact"
import SelectCharacter from "@c/select/SelectCharacter"
import SelectCharacterLevel from "@c/select/SelectCharacterLevel"
import SelectWeapon from "@c/select/SelectWeapon"
import SelectWeaponLevel from "@c/select/SelectWeaponLevel"
import SelectTargetFunction from "@c/select/SelectTargetFunction"
import SelectCharacterSkill from "@c/select/SelectCharacterSkill"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import AddButton from "@c/misc/AddButton"
import DamagePanel from "./DamagePanel"
import MyButton1 from "@c/button/MyButton1"
import DamageAnalysis from "@c/display/DamageAnalysis"
import AttributePanel from "@c/display/AttributePanel"

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
        ArtifactDisplay,
        AddButton,
        DamagePanel,
        MyButton1,
        DamageAnalysis,
        AttributePanel
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

            weaponName: "PolarStar",
            weaponLevel: "90",
            weaponConfig: {
                "PolarStar": {
                    stack: 1
                }
            },

            targetFunctionName: "GanyuDefault",
            targetFunctionConfig: "NoConfig",
            optimizationResults: [],
            optimizationResultIndex: 0,

            artifactIds: [-1, -1, -1, -1, -1],

            showSelectArtifactDialog: false,
            selectArtifactSlot: "any",

            showDamageAnalysisDialog: false
        }
    },
    computed: {
        ...mapGetters({
            artifactsById: "artifacts/artifactsById"
        }),

        characterLevelNumber() {
            return parseInt(this.characterLevel)
        },

        characterAscend() {
            return this.characterLevel.includes("+")
        },

        characterSplash() {
            const data = characterData[this.characterName]
            return data.splash ?? data.card
        },

        characterNeedConfig() {
            return !!characterConfig[this.characterName]
        },

        characterConfigComponent() {
            return characterConfig[this.characterName]
        },

        characterNeedSkillConfig() {
            return !!characterSkillConfig[this.characterName]
        },

        characterSkillConfigComponent() {
            return characterSkillConfig[this.characterName]
        },

        characterWeaponType() {
            const item = characterData[this.characterName]
            return item ? item.weapon : "bow"
        },

        characterInterface() {
            return {
                name: this.characterName,
                level: this.characterLevelNumber,
                ascend: this.characterAscend,
                constellation: 0,
                skill1: 8,
                skill2: 8,
                skill3: 8,
                params: this.characterConfig
            }
        },

        characterSkillInterface() {
            return {
                index: this.characterSkillIndex,
                config: this.characterSkillConfig
            }
        },

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
            return !!weaponConfig[this.weaponName]
        },

        weaponConfigComponent() {
            return weaponConfig[this.weaponName]
        },

        weaponInterface() {
            return {
                name: this.weaponName,
                level: this.weaponLevelNumber,
                ascend: this.weaponAscend,
                refine: 1,
                params: this.weaponConfig
            }
        },

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
                buffs: [], // todo
                artifacts: this.artifactWasmFormat,
                artifact_config: null, // todo
                skill: this.characterSkillInterface
            }
        },

        getAttributeWasmInterface() {
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                buffs: [], // todo
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
                buffs: [] // todo
            }
        },

        characterDamageAnalysis() {
            const temp = this.$mona.CalculatorInterface.get_damage_analysis(this.damageAnalysisWasmInterface)
            // console.log(temp)
            return temp
        },

        attributeFromWasm() {
            return this.$mona.CommonInterface.get_attribute(this.getAttributeWasmInterface)
        }
    },
    methods: {
        handleOptimizeArtifact() {
            const interfac = this.optimizeArtifactWasmInterface
            const start = new Date()

            const worker = new Worker(new URL("@worker/optimize_artifact.js", import.meta.url))
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
                }
            }
        },

        handleUseNthOptimizationResult(n) {
            // const index = this.optimizationResultIndex
            // if (index === 0) {
            //     return
            // }

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
        }
    },
    watch: {
        weaponName(newName, oldName) {
            if (weaponConfig[newName]) {
                const componentDefinition = weaponConfig[newName]
                const defaultConfig = componentDefinition.getDefaultConfig()
                this.weaponConfig = defaultConfig
            } else {
                this.weaponConfig = "NoConfig"
            }
        },

        characterName(newName, oldName) {
            if (characterConfig[newName]) {
                const componentDefinition = characterConfig[newName]
                const defaultConfig = componentDefinition.getDefaultConfig()
                this.characterConfig = defaultConfig
            } else {
                this.characterConfig = "NoConfig"
            }

            if (characterSkillConfig[newName]) {
                const skillComponentDefinition = characterSkillConfig[newName]
                const defaultConfig = skillComponentDefinition.getDefaultConfig()
                this.characterSkillConfig = defaultConfig
            } else {
                this.characterSkillConfig = "NoConfig"
            }
        }
    }
};

</script>

<style lang="scss" scoped>
.big-container {
    display: flex;
    gap: 16px;

    .left-container {
        flex: 1;
        // width: 1000px;
        // margin: 0 auto;
        // background: #00000011;
        position: relative;
        // overflow-y: auto;
        // overflow-x: visible;
    }

    .middle-container {
        flex: 2;
        // overflow-y: auto;
        // overflow-x: hidden;
    }

    .right-container {
        flex: 1;
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
        right: -100px;
        top: -32px;
        pointer-events: none;
    }

    .character-extra-config {
        margin-top: 16px;
        border-left: #222222 solid 3px;
        padding: 8px;
        background: #12345611;
        border-top-right-radius: 3px;
        border-bottom-right-radius: 3px;
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
        border-left: #222222 solid 3px;
        padding: 8px;
        background: #12345611;
        border-top-right-radius: 3px;
        border-bottom-right-radius: 3px;
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
</style>