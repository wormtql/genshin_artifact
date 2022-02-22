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
            width="60%"
        >
            <damage-analysis
                ref="damageAnalysis"
                :enemy-config="enemyConfig"
                :character-level="characterLevelNumber"
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

        <el-dialog
            :visible.sync="showConstraintDialog"
            title="计算限制"
            width="60%"
        >
            <p>限定套装</p>
            <div style="margin-top: 12px">
                <select-artifact-set
                    multiple
                    v-model="constraintArtifactSet"
                ></select-artifact-set>
            </div>
            
            <p>过滤圣遗物组</p>
            <el-tree
                :data="kumiTreeDataForElementUI"
                show-checkbox
                ref="filterKumiRef"
            >
            </el-tree>
        </el-dialog>

        <el-dialog
            :visible.sync="showArtifactAnalysisDialog"
            title="圣遗物分析"
            width="60%"
        >
            <artifacts-set-statistics
                :artifact-ids="artifactIds"
            ></artifacts-set-statistics>
        </el-dialog>

        <el-dialog
            :visible.sync="showArtifactPerBonusDialog"
            title="词条收益曲线"
            width="60%"
        >
            <artifact-per-stat-bonus
                :data="miscPerStatBonus"
            ></artifact-per-stat-bonus>
        </el-dialog>

        <el-dialog
            :visible.sync="showSaveKumiDialog"
            title="新建圣遗物组"
            width="60%"
        >
            <save-as-kumi
                @confirm="handleSaveAsKumi"
            ></save-as-kumi>
        </el-dialog>

        <el-dialog
            :visible.sync="showUseKumiDialog"
            title="选择圣遗物组"
            width="60%"
        >
            <div style="height: 60vh" class="mona-scroll">
                <el-tree
                    :data="kumiTreeDataForElementUI"
                    @node-click="handleUseKumi"
                ></el-tree>
            </div>

        </el-dialog>

        <el-dialog
            :visible.sync="showEnemyConfigDialog"
            title="敌人设置"
            width="60%"
        >
            <enemy-config
                v-model="enemyConfig"
            ></enemy-config>
        </el-dialog>

        <div class="top-things" ref="topThings">
            <el-breadcrumb>
                <el-breadcrumb-item>裏</el-breadcrumb-item>
            </el-breadcrumb>
            <el-divider></el-divider>
        </div>

        <el-row class="big-container" ref="bigContainer"
            :style="{ height: miscBigContainerHeight }"
        >
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

                <div class="config-target-function">
                    <p class="common-title">目标函数</p>
                    <div class="my-button-list" style="margin-bottom: 12px">
                        <my-button-1 icon="el-icon-caret-right" title="开始计算"
                            @click="handleOptimizeArtifact"
                        ></my-button-1>
                        <my-button-1 icon="el-icon-s-operation" title="设置"
                            @click="handleClickSetupOptimization"
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
                            :title="`共计算${optimizationResults.length}组圣遗物搭配`"
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
                        <value-display
                            :value="optimizationResults[optimizationResultIndex - 1].ratio"
                            style="margin-top: 12px"
                        ></value-display>
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
            </el-col>

            <el-col :span="12" class="middle-container">
                <p class="common-title">圣遗物</p>

                <div class="artifact-tool" style="margin-bottom: 12px">
                    <my-button-1 icon="el-icon-s-data" title="圣遗物分析"
                                 @click="handleClickArtifactAnalysis"
                    ></my-button-1>
                    <my-button-1 icon="el-icon-star-on" title="存为套装"
                                 @click="handleClickSaveAsKumi"
                    ></my-button-1>
                    <my-button-1 icon="el-icon-folder" title="应用套装"
                                 @click="handleClickUseKumi"
                    ></my-button-1>
                </div>

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

                <div v-if="artifactNeedConfig4" style="margin-top: 16px">
                    <p class="common-description">
                        <span class="effect4">四件套效果：</span>
                        <span v-html="artifactEffect4Text"></span>
                    </p>
                    <item-config
                        v-model="artifactSingleConfig"
                        :item-name="artifactConfig4ItemName"
                        :configs="artifactConfig4Configs"
                    ></item-config>
                </div>

                <el-divider></el-divider>

                <p class="common-title">伤害计算</p>
                <div class="my-button-list" style="margin-bottom: 12px">
                    <my-button-1 icon="el-icon-s-data" title="明细"
                        @click="handleDisplayAnalysis"
                    ></my-button-1>
                    <my-button-1 icon="el-icon-s-operation" title="敌人设置"
                                 @click="handleClickEnemyConfig"
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

                <h3 class="common-title2" style="margin-top: 24px">剧变反应伤害</h3>
                <transformative-damage
                    :data="characterTransformativeDamage"
                ></transformative-damage>
            </el-col>

            <el-col :span="6" class="right-container">
                <div class="common-title">面板</div>

                <div class="my-button-list" style="margin-bottom: 12px">
                    <my-button-1 icon="el-icon-s-data" title="开始计算"
                                 @click="handleClickAttributeAnalysis"
                    ></my-button-1>
                </div>

                <attribute-panel
                    :attribute="attributeFromWasm"
                ></attribute-panel>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import { mapGetters, mapState } from "vuex"

import { convertArtifact, convertArtifactName } from "@util/converter"
import { newDefaultArtifactConfigForWasm } from "@util/artifacts"
import {getArtifactIdsByKumiId, newKumiWithArtifacts} from "@util/kumi"
import { toSnakeCase } from "@util/common"
import { positions } from "@const/artifact"
import { characterData } from "@character"
import { weaponData } from "@weapon"
import { targetFunctionData } from "@targetFunction"
import { buffData } from "@buff"
import { artifactsData } from "@artifact"
import { wasmBonusPerStat } from "@/wasm/bonus_per_stat"

import SelectArtifact from "@c/select/SelectArtifact"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
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
import AttributePanel from "@c/display/AttributePanel"
import ItemConfig from "@c/config/ItemConfig"
import BuffItem from "./BuffItem"
import WeaponDisplay from "@c/display/WeaponDisplay"
import SaveAsKumi from "./SaveAsKumi"
import TransformativeDamage from "./TransformativeDamage"
import ValueDisplay from "./ValueDisplay"
import EnemyConfig from "./EnemyConfig"

export default {
    name: "NewArtifactPlanPage",
    components: {
        SelectArtifact,
        SelectArtifactSet,
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
        DamageAnalysis: () => import("@c/display/DamageAnalysis"),
        AttributePanel,
        ItemConfig,
        BuffItem,
        WeaponDisplay,
        ArtifactsSetStatistics: () => import("@c/display/ArtifactsSetStatistics"),
        ArtifactPerStatBonus: () => import("@c/display/ArtifactPerStatBonus"),
        SaveAsKumi,
        TransformativeDamage,
        ValueDisplay,
        EnemyConfig,
    },
    created() {
        // this.characterData = characterData
    },
    mounted() {
        const container = this.$refs["bigContainer"]
        const top = this.$refs["topThings"]

        const heightTop = top.offsetHeight + 24
        // console.log(heightTop)
        // container.style.height = `calc(100% - ${heightTop}px)`
        this.miscBigContainerHeight = `calc(100% - ${heightTop}px)`
        // console.log(container.style.height)
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

            constraintArtifactSet: [],

            enemyConfig: {
                level: 90,
                electro_res: 0.1,
                pyro_res: 0.1,
                hydro_res: 0.1,
                cryo_res: 0.1,
                geo_res: 0.1,
                anemo_res: 0.1,
                dendro_res: 0.1,
                physical_res: 0.1
            },

            buffs: [],

            targetFunctionName: "AmberDefault",
            targetFunctionConfig: "NoConfig",
            optimizationResults: [],
            optimizationResultIndex: 0,

            artifactIds: [-1, -1, -1, -1, -1],
            artifactSingleConfig: null,

            selectArtifactSlot: "any",

            showSelectArtifactDialog: false,
            showDamageAnalysisDialog: false,
            showSelectBuffDialog: false,
            showConstraintDialog: false,
            showArtifactAnalysisDialog: false,
            showArtifactPerBonusDialog: false,
            showSaveKumiDialog: false,
            showUseKumiDialog: false,
            showEnemyConfigDialog: false,

            miscBigContainerHeight: "",
            miscPerStatBonus: {},
        }
    },
    computed: {
        ...mapGetters({
            artifactsById: "artifacts/artifactsById",
            allArtifactsFlat: "artifacts/allFlat"
        }),

        ...mapGetters("kumi", {
            kumiTreeDataForElementUI: "treeDataForElementUI"
        }),

        // ...mapState("kumi", {
        //
        // }),

        // enemy
        enemyInterface() {
            return this.enemyConfig
        },
        // end enemy

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

        characterTransformativeDamage() {
            return this.$mona.CalculatorInterface.get_transformative_damage(this.damageAnalysisWasmInterface)
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

        // artifact computed
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

        // if needing config, return set name
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

        artifactConfig4ItemName() {
            const setNameWasm = convertArtifactName(this.artifactNeedConfig4)
            return `config_${toSnakeCase(setNameWasm)}`
        },

        artifactEffect4Text() {
            if (!this.artifactNeedConfig4) {
                return ""
            }
            const data = artifactsData[this.artifactNeedConfig4]
            return data.effect4
        },

        artifactConfig4Configs() {
            if (this.artifactNeedConfig4) {
                const data = artifactsData[this.artifactNeedConfig4]
                // console.log(data.config4)
                return data.config4
            }
            return []
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

        // constraint
        constraintSetMode() {
            const convertedName = this.constraintArtifactSet.map(x => convertArtifactName(x))
            const len = convertedName.length
            if (len === 2) {
                return {
                    "Set22": convertedName
                }
            } else if (len === 1) {
                return {
                    "Set4": convertedName[0]
                }
            } else {
                return "Any"
            }
        },

        constraintInterface() {
            let t = {
                "set_mode": this.constraintSetMode,
                "hp_min": null,
                "atk_min": null,
                "def_min": null,
                "recharge_min": null,
                "em_min": null,
                "crit_min": null,
                "crit_dmg_min": null
            }
            return t
        },

        damageAnalysisWasmInterface() {
            // console.log(this.weaponInterface);
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                buffs: this.buffsInterface,
                artifacts: this.artifactWasmFormat,
                artifact_config: this.artifactConfigForCalculator,
                skill: this.characterSkillInterface,
                enemy: this.enemyInterface,
            }
        },

        getAttributeWasmInterface() {
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                buffs: this.buffsInterface,
                artifacts: this.artifactWasmFormat,
                artifact_config: this.artifactConfigForCalculator,
            }
        },

        attributeFromWasm() {
            return this.$mona.CommonInterface.get_attribute(this.getAttributeWasmInterface)
        },

        bonusPerStatWasmInterface() {
            return {
                character: this.characterInterface,
                weapon: this.weaponInterface,
                artifacts: this.artifactWasmFormat,
                tf: this.targetFunctionInterface,
                buffs: this.buffsInterface,
                artifacts_config: this.artifactConfigForCalculator
            }
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
        handleClickEnemyConfig() {
            this.showEnemyConfigDialog = true
        },

        handleClickSetupOptimization() {
            this.showConstraintDialog = true

        },

        handleClickSaveAsKumi() {
            this.showSaveKumiDialog = true
        },

        handleSaveAsKumi({ dirIds, name }) {
            // console.log(dirIds)
            let ids = this.artifactIds.filter(x => x)
            for (let dirId of dirIds) {
                newKumiWithArtifacts(dirId, name, ids)
            }

            this.showSaveKumiDialog = false
        },

        handleClickUseKumi() {
            this.showUseKumiDialog = true
        },

        handleUseKumi(node) {
            if (Object.prototype.hasOwnProperty.call(node, "kumiId")) {
                const kumiId = node.kumiId
                // console.log(kumiId)
                this.showUseKumiDialog = false

                const ids = getArtifactIdsByKumiId(kumiId)
                let temp = {}
                for (let id of ids) {
                    const artifact = this.artifactsById[id]
                    if (artifact) {
                        temp[artifact.position] = id
                    }
                }

                let idsWithNull = []
                idsWithNull.push(temp.flower ?? null)
                idsWithNull.push(temp.feather ?? null)
                idsWithNull.push(temp.sand ?? null)
                idsWithNull.push(temp.cup ?? null)
                idsWithNull.push(temp.head ?? null)

                this.artifactIds = idsWithNull
            }
        },

        getAllArtifactsFiltered() {
            const component = this.$refs["filterKumiRef"]

            let s = new Set()
            if (component) {
                const nodes = component.getCheckedNodes(true)
                for (let node of nodes) {
                    const kumiId = node.kumiId
                    const artifactIds = getArtifactIdsByKumiId(kumiId)
                    for (let i of artifactIds) {
                        s.add(i)
                    }
                }
            }
            
            // console.log(s)

            let filtered = []

            for (let artifact of this.allArtifactsFlat) {
                if (!artifact.lock && !s.has(artifact.id)) {
                    filtered.push(artifact)
                }
            }

            return filtered
        },

        getAllArtifactsFilteredWasm() {
            return this.getAllArtifactsFiltered().map(x => convertArtifact(x))
        },

        getOptimizeArtifactWasmInterface() {
            const artifacts = this.getAllArtifactsFilteredWasm()
            const artifacts16 = artifacts.filter(x => x.level >= 16)

            return {
                artifacts: artifacts16,
                character: this.characterInterface,
                weapon: this.weaponInterface,
                target_function: this.targetFunctionInterface,
                constraint: this.constraintInterface,
                buffs: this.buffsInterface
            }
        },

        handleClickArtifactAnalysis() {
            this.showArtifactAnalysisDialog = true
        },

        handleOptimizeArtifact() {
            const interfac = this.getOptimizeArtifactWasmInterface()
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

        handleClickAttributeAnalysis() {
            // const ret = this.$mona.BonusPerStat.bonus_per_stat(this.bonusPerStatWasmInterface)
            // const atk_ptr = temp.atk_ptr
            // const arr = new Float64Array(this.$mona.memory.buffer, atk_ptr, 10)
            // const length = 10;

            // const f = (ret, name) => Array.from(new Float64Array(this.$mona.memory.buffer, ret[`${name}_ptr`], ret[`${name}_len`]))

            wasmBonusPerStat(this.bonusPerStatWasmInterface).then(ret => {
                // let temp = new Float64Array(this.$mona.memory.buffer, ret.atk_ptr, 100)
                // console.log(this.$mona.memory.buffer)
                // console.log(temp)
                this.miscPerStatBonus = ret
                // this.miscPerStatBonus = {
                //     atk: f(ret, "atk"),
                //     atk_percentage: f(ret, "atk_percentage"),
                //     def: f(ret, "def"),
                //     def_percentage: f(ret, "def_percentage"),
                //     hp: f(ret, "hp"),
                //     hp_percentage: f(ret, "hp_percentage"),
                //     critical: f(ret, "critical"),
                //     critical_damage: f(ret, "critical_damage"),
                //     recharge: f(ret, "recharge"),
                //     elemental_mastery: f(ret, "elemental_mastery"),
                // }

                this.showArtifactPerBonusDialog = true

                // console.log(this.miscPerStatBonus)
            })
            // console.log(arr)
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
        },

        artifactNeedConfig4(newName) {
            if (!newName) {
                this.artifactSingleConfig = null
            } else {
                const data = artifactsData[newName]

                let defaultConfig = {}
                for (let c of data.config4) {
                    defaultConfig[c.name] = c.default
                }

                const nameWasm = convertArtifactName(newName)
                const configItemName = `config_${toSnakeCase(nameWasm)}`
                this.artifactSingleConfig = {
                    [configItemName]: defaultConfig
                }
                // console.log(this.artifactSingleConfig)
            }
        },
    }
};

</script>

<style lang="scss" scoped>
//.outer-container {
//    display: flex;
//    flex-direction: column;
//    height: 100%;
//}

.big-container {
    //flex: 1;
    //flex-grow: 1;
    //overflow-y: hidden;
    //overflow-x: visible;
    //overflow-x: visible;
    //overflow: hidden;

    .left-container, .middle-container, .right-container {
        height: 100%;
        overflow-x: hidden;
        overflow-y: auto;
        //overflow: visible;

        &::-webkit-scrollbar {
            width: 4px;
        }

        &::-webkit-scrollbar-track {
            background: rgb(247, 247, 247);
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
        //align-items: top;
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

.common-description {
    font-size: 12px;
    color: #606266;
}

 //artifact effect description title
.effect4 {
    color: #6eb7ff;
}
</style>