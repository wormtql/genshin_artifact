<template>
    <div style="overflow: hidden">
        <el-dialog
            :title="t('calcPage.selectArt')"
            :width="deviceIsPC ? '80%' : '90%'"
            v-model="showSelectArtifactDialog"
        >
            <select-artifact
                :position="selectArtifactSlot"
                @select="handleSelectArtifact"
            ></select-artifact>
        </el-dialog>

<!--        damage analysis-->
        <el-dialog
            v-model="showDamageAnalysisDialog"
            :title="t('calcPage.dmgComp')"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <damage-analysis
                ref="damageAnalysisComponent"
                :enemy-config="enemyConfig"
                :character-level="characterLevelNumber"
            ></damage-analysis>
        </el-dialog>

<!--    select buff    -->
        <el-dialog
            v-model="showSelectBuffDialog"
            :title="t('calcPage.selectBuff')"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <select-buff
                @select="handleSelectBuff"
            ></select-buff>
        </el-dialog>

<!--    computation setup    -->
        <el-dialog
            v-model="showConstraintDialog"
            :title="t('calcPage.setupCalc')"
            :width="deviceIsPC ? '400px' : '90%'"
        >
            <p class="common-title2">
                {{ t("misc.algo") }}
                <el-tooltip>
                    <i-fa6-solid-circle-question></i-fa6-solid-circle-question>
                    <template #content>
                        <span v-html="t('calcPage.algoDesc')">

                        </span>
<!--                        {{ t("calcPage.algoDesc") }}-->
                    </template>
                </el-tooltip>
            </p>
            <el-alert
                v-if="algorithm === 'Naive'"
                :title="t('calcPage.plzSetConst')"
                type="warning"
                style="margin-bottom: 12px"
            ></el-alert>
            <el-radio-group v-model="algorithm">
                <el-radio label="AStar">{{ t("calcPage.aStar") }}</el-radio>
<!--                <el-radio label="Heuristic">{{t("calcPage.heuristic")}}</el-radio>-->
                <el-radio label="Naive">{{ t("calcPage.naive") }}</el-radio>
            </el-radio-group>

            <p class="common-title2">{{ t("calcPage.constSet") }}</p>
            <div style="margin-top: 12px; margin-bottom: 12px">
                <select-artifact-set
                    multiple
                    v-model="constraintArtifactSet"
                    :style="{ width: '100%' }"
                    :placeholder="t('calcPage.constSet')"
                ></select-artifact-set>
            </div>

            <p class="common-title2">{{ t("calcPage.constMain") }}</p>
            <div style="margin-top: 12px; margin-bottom: 12px">
                <div class="constraint-main-stat-item">
                    <span>{{ t("misc.sand") }}</span>
                    <select-artifact-main-stat
                        v-model="constraintSandMainStats"
                        :include-any="false"
                        :multiple="true"
                        position="sand"
                        :style="{ width: 'calc(100% - 48px)' }"
                        :placeholder="t('calcPage.constMain')"
                    ></select-artifact-main-stat>
                </div>
                <div class="constraint-main-stat-item">
                    <span>{{ t("misc.cup") }}</span>
                    <select-artifact-main-stat
                        v-model="constraintGobletMainStats"
                        :include-any="false"
                        :multiple="true"
                        position="cup"
                        :style="{ width: 'calc(100% - 48px)' }"
                        :placeholder="t('calcPage.constMain')"
                    ></select-artifact-main-stat>
                </div>
                <div class="constraint-main-stat-item">
                    <span>{{ t("misc.head") }}</span>
                    <select-artifact-main-stat
                        v-model="constraintHeadMainStats"
                        :include-any="false"
                        :multiple="true"
                        position="head"
                        :style="{ width: 'calc(100% - 48px)' }"
                        :placeholder="t('calcPage.constMain')"
                    ></select-artifact-main-stat>
                </div>
            </div>

            <p class="constraint-title">{{ t("calcPage.constMin") }}</p>
            <div>
                <div class="constraint-min-item">
                    <span class="constraint-min-title">{{ t("stat.recharge") }}</span>
                    <div class="slider-div">
                        <el-slider
                            :min="1"
                            :max="4"
                            :step="0.05"
                            v-model="constraintMinRecharge"
                            :show-input="deviceIsPC"
                        ></el-slider>
                    </div>
                </div>
                <div class="constraint-min-item">
                    <span class="constraint-min-title">{{ t("stat.elementalMastery") }}</span>
                    <div class="slider-div">
                        <el-slider
                            :min="0"
                            :max="2000"
                            :step="10"
                            v-model="constraintMinElementalMastery"
                            :show-input="deviceIsPC"
                        ></el-slider>
                    </div>
                </div>
                <div class="constraint-min-item">
                    <span class="constraint-min-title">{{ t("stat.critical") }}</span>
                    <div class="slider-div">
                        <el-slider
                            :min="0"
                            :max="1"
                            :step="0.01"
                            v-model="constraintMinCritical"
                            :show-input="deviceIsPC"
                        ></el-slider>
                    </div>
                </div>
                <div class="constraint-min-item">
                    <span class="constraint-min-title">{{ t("stat.criticalDamage") }}</span>
                    <div class="slider-div">
                        <el-slider
                            :min="0"
                            :max="4"
                            :step="0.1"
                            v-model="constraintMinCriticalDamage"
                            :show-input="deviceIsPC"
                        ></el-slider>
                    </div>
                </div>
            </div>

            <p class="common-title2">{{ t("calcPage.filKumi") }}</p>
            <div style="max-height: 50vh; overflow: auto" class="mona-scroll">
                <el-tree
                    :data="kumiTreeDataForElementUI"
                    show-checkbox
                    ref="filterKumiRef"
                >
                </el-tree>
            </div>
        </el-dialog>

<!--    artifacts analysis    -->
        <el-dialog
            v-model="showArtifactAnalysisDialog"
            :title="t('calcPage.artAnalysis')"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <artifacts-set-statistics
                :artifact-ids="artifactIds"
            ></artifacts-set-statistics>
        </el-dialog>

<!--    stats marginal bonus    -->
        <el-dialog
            v-model="showArtifactPerBonusDialog"
            :title="t('calcPage.statCurve')"
            :width="deviceIsPC ? '80%' : '90%'"
        >
            <artifact-per-stat-bonus
                :data="miscPerStatBonus"
            ></artifact-per-stat-bonus>
        </el-dialog>

<!--        new artifact kumi-->
        <save-as-kumi
            v-model="showSaveKumiDialog"
            :default-name="kumiDefaultName"
            @confirm="handleSaveAsKumi"
        ></save-as-kumi>

<!--    select artifact kumi    -->
        <el-dialog
            v-model="showUseKumiDialog"
            :title="t('calcPage.selectKumi')"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <div style="height: 60vh" class="mona-scroll">
                <el-tree
                    :data="kumiTreeDataForElementUI"
                    @node-click="handleUseKumi"
                ></el-tree>
            </div>

        </el-dialog>

<!--        enemy config-->
        <el-dialog
            v-model="showEnemyConfigDialog"
            :title="t('calcPage.setupEnemy')"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <enemy-config-component
                v-model="enemyConfig"
            ></enemy-config-component>
        </el-dialog>

<!--        artifact config-->
        <el-dialog
            v-model="showConfigArtifactDialog"
            :title="t('calcPage.setupArt')"
            :width="deviceIsPC ? '60%' : '90%'"
        >
            <h3 class="common-title2">{{ t("calcPage.artMode") }}</h3>
            <el-radio-group
                v-model="artifactEffectMode"
            >
                <el-radio label="auto">{{ t("calcPage.modeAuto") }}</el-radio>
                <el-radio label="custom">{{ t("calcPage.modeCustom") }}</el-radio>
            </el-radio-group>

            <h3 class="common-title2">{{ t("calcPage.artEffect") }}</h3>
            <artifact-config
                v-model="artifactConfig"
            ></artifact-config>
        </el-dialog>

        <el-row class="big-container">
            <el-col class="left-container mona-scroll-hidden" :sm="24" :md="6">
                <div class="config-character">
                    <img :src="characterSplash" alt="角色" class="character-splash" />
                    <div class="select-character">
                        <p class="common-title">{{ t("misc.character") }}</p>
                        <div style="display: flex; gap: 12px">
<!--                            <select-character-->
<!--                                :model-value="characterName"-->
<!--                                @update:modelValue="changeCharacter"-->
<!--                                style="flex: 1"-->
<!--                            ></select-character>-->
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
                            <h3 class="common-title2">{{ t("misc.skill") }}</h3>
                            <div class="skill-div">
                                <el-input-number
                                    controls-position="right"
                                    v-model="characterSkill1"
                                    :min="1"
                                    :max="15"
                                    style="flex: 1; display: block; width: unset"
                                ></el-input-number>
                                <el-input-number
                                    controls-position="right"
                                    v-model="characterSkill2"
                                    :min="1"
                                    :max="15"
                                    style="flex: 1; display: block; width: unset"
                                ></el-input-number>
                                <el-input-number
                                    controls-position="right"
                                    v-model="characterSkill3"
                                    :min="1"
                                    :max="15"
                                    style="flex: 1; display: block; width: unset"
                                ></el-input-number>
                            </div>
                        </div>

                        <div class="config-character-constellation">
                            <h3 class="common-title2">{{ t("misc.conste") }}</h3>
                            <el-input-number
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
                        <p class="common-title">{{ t("misc.weapon") }}</p>

                        <div style="display: flex; gap: 12px; margin-bottom: 8px">
<!--                            <select-weapon-->
<!--                                :type="characterWeaponType"-->
<!--                                :model-value="weaponName"-->
<!--                                @update:modelValue="changeWeapon"-->
<!--                                style="flex: 1"-->
<!--                            ></select-weapon>-->
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
                            <h3 class="common-title2">{{ t("misc.refine") }}</h3>
                            <el-input-number
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
                    <p class="common-title">{{ t("misc.tf") }}</p>
                    <div class="my-button-list" style="margin-bottom: 12px">
                        <el-button-group>
                            <el-button
                                type="primary"
                                :icon="IconEpCaretRight"
                                @click="handleOptimizeArtifact"
                            >{{ t("calcPage.start") }}</el-button>

                            <el-button
                                :icon="IconEpTools"
                                @click="handleClickSetupOptimization"
                            >{{ t("calcPage.setupCalc") }}</el-button>

                            <el-button
                                :icon="IconEpTools"
                                @click="handleClickArtifactConfig"
                            >{{ t("calcPage.setupArt") }}</el-button>
                        </el-button-group>
                    </div>

                    <div class="my-button-list" style="margin-bottom: 12px">
                        <el-dropdown
                            trigger="click"
                            @command="handleCommandPreset"
                            @click="handleSavePreset(miscCurrentPresetName)"
                            split-button
                        >
                            <template v-if="!miscCurrentPresetName">{{ t("calcPage.newPreset") }}</template>
                            <template v-else>{{ t("calcPage.savePreset") }}「{{ miscCurrentPresetName }}」</template>

                            <template #dropdown>
                                <el-dropdown-menu>
                                    <el-dropdown-item
                                        v-if="miscCurrentPresetName"
                                        icon="el-icon-s-tools"
                                        command="save-preset"
                                    >{{ t("calcPage.saveAsPreset") }}</el-dropdown-item>

                                    <el-dropdown-item
                                        v-for="(item, index) in presetStore.allFlat.value"
                                        :divided="index === 0 && !!miscCurrentPresetName"
                                        :key="item.name"
                                        :icon="IconEpMenu"
                                        :command="'apply-' + item.name"
                                    >{{ item.name }}</el-dropdown-item>
                                </el-dropdown-menu>
                            </template>
                        </el-dropdown>
                    </div>

                    <el-tabs v-model="miscTargetFunctionTab">
                        <el-tab-pane :label="t('calcPage.tfNormal')" name="normal">
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
                                        v-html="targetFunctionDescription"
                                    ></p>
                                </div>
                            </div>
                        </el-tab-pane>
                        <el-tab-pane label="MONA-DSL" name="dsl">
                            <el-alert type="warning" :title="t('calcPage.beta')" :closable="false" style="margin-bottom: 8px"></el-alert>
                            <el-input type="textarea" :rows="10" :placeholder="t('misc.code')" v-model="targetFunctionDSLSource" class="code-input"></el-input>
                        </el-tab-pane>
                    </el-tabs>


                    <div v-if="optimizationResults.length > 0"
                        style="margin-top: 12px"
                    >
                        <el-alert
                            :title="`共计算${optimizationResults.length}组圣遗物搭配`"
                            type="success"
                            style="margin-bottom: 12px"
                        ></el-alert>
                        <el-input-number
                            :model-value="optimizationResultIndex"
                            @update:modelValue="handleUseNthOptimizationResult"
                            :min="1"
                            :max="optimizationResults.length"
                            style="width: 100%"
                        ></el-input-number>
                        <value-display
                            :value="optimizationResults[optimizationResultIndex - 1].ratio"
                            :extra="optimizationResults[optimizationResultIndex - 1].value.toFixed(1)"
                            style="margin-top: 12px"
                        ></value-display>
                    </div>
                </div>

                <el-divider></el-divider>

                <div class="config-buff">
                    <p class="common-title">BUFF</p>
                    <div class="buff-tool" style="margin-bottom: 12px">
                        <el-button
                            :icon="IconEpPlus"
                            @click="handleClickAddBuff"
                            title="添加BUFF"
                            circle
                            text
                        ></el-button>
                    </div>
                    <div class="buffs" v-if="buffs.length > 0">
                        <buff-item
                            v-for="buff in buffs"
                            :key="buff.id"
                            :buff="buff"
                            v-model:buffConfig="buff.config"
                            @delete="handleClickDeleteBuff(buff.id)"
                            @toggle="handleClickToggleBuff(buff.id)"
                        ></buff-item>
                    </div>
                    <div v-else>
                        <el-empty description="无BUFF"></el-empty>
                    </div>
                </div>
            </el-col>

            <el-col :sm="24" :md="12" class="middle-container mona-scroll-hidden">
                <p class="common-title">{{ t("misc.artifact") }}</p>

                <div class="artifact-tool" style="margin-bottom: 12px">
                    <el-button-group>
                        <el-button
                            :icon="IconEpHistogram"
                            @click="handleClickArtifactAnalysis"
                        >{{ t("calcPage.statAnalysis") }}</el-button>
                        <el-button
                            :icon="IconEpStarFilled"
                            @click="handleClickSaveAsKumi"
                        >{{ t("calcPage.saveKumi") }}</el-button>
                        <el-button
                            :icon="IconEpFolder"
                            @click="handleClickUseKumi"
                        >{{ t("calcPage.useKumi") }}</el-button>
                        <el-button
                            v-show="artifactCount > 0"
                            :icon="isAllLocked ? IconEpUnlock : IconEpLock"
                            @click="() => { isAllLocked ? handleUnlockAll() : handleLockAll() }"
                        >{{ isAllLocked ? t("calcPage.unlockAll") : t("calcPage.lockAll") }}</el-button>
                    </el-button-group>
                </div>

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
                            @delete="removeArtifact(index)"
                            @toggle="artifactStore.toggleArtifact(artifactItems[index].id)"
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

                <div v-if="artifactNeedConfig4" style="margin-top: 16px">
                    <p class="common-description">
                        <span class="effect4">{{ t("calcPage.effect4") }}</span>
                        <span v-html="artifactEffect4Text"></span>
                    </p>
                    <item-config
                        v-if="artifactSingleConfig"
                        v-model="artifactSingleConfig"
                        :item-name="artifactConfig4ItemName"
                        :configs="artifactConfig4Configs"
                    ></item-config>
                </div>

                <el-divider></el-divider>

                <p class="common-title">{{ t("calcPage.dmg") }}</p>
                <div class="my-button-list" style="margin-bottom: 12px">
                    <el-button-group>
                        <el-button
                            :icon="IconEpHistogram"
                            @click="handleDisplayAnalysis"
                        >{{ t("calcPage.detail") }}</el-button>
                        <el-button
                            :icon="IconEpTools"
                            @click="handleClickEnemyConfig"
                        >{{ t("calcPage.setupEnemy") }}</el-button>
                    </el-button-group>
                </div>

                <div>
                    <h3 class="common-title2">{{ t("calcPage.fumo") }}</h3>
                    <select-element-type
                        v-model="fumo"
                        :elements="['Pyro', 'Electro', 'Hydro', 'Anemo', 'Geo', 'Cryo', 'Dendro', 'None']"
                    ></select-element-type>
                </div>

                <div v-if="characterNeedSkillConfig" style="margin-bottom: 16px;">
                    <h3 class="common-title2">{{ t("calcPage.skillConfig") }}</h3>
                    <item-config
                        v-model="characterSkillConfig"
                        :item-name="characterName"
                        :configs="characterSkillConfigConfig"
                    ></item-config>
                </div>
                <div class="damage-analysis-div">
                    <h3 class="common-title2">{{ t("calcPage.skill") }}</h3>
                    <select-character-skill
                        v-model="characterSkillIndex"
                        :character-name="characterName"
                        style="margin-bottom: 16px"
                    ></select-character-skill>
                    <damage-panel
                        :analysis-from-wasm="characterDamageAnalysis"
                    ></damage-panel>
                </div>

                <h3 class="common-title2" style="margin-top: 24px">{{ t("calcPage.dmg2") }}</h3>
                <transformative-damage
                    :data="characterTransformativeDamage"
                ></transformative-damage>
            </el-col>

            <el-col :sm="24" :md="6" class="right-container mona-scroll-hidden">
                <div class="common-title">{{ t("calcPage.panel") }}</div>

                <div class="my-button-list" style="margin-bottom: 12px">
                    <el-button
                        :icon="IconEpHistogram"
                        @click="handleClickAttributeAnalysis"
                    >{{ t("calcPage.statCurve") }}</el-button>
                </div>

                <attribute-panel
                    :attribute="attributeFromWasm"
                ></attribute-panel>
            </el-col>
        </el-row>
    </div>
</template>

<script setup lang="ts">
import {convertArtifact} from "@util/converter"
import {newDefaultArtifactConfigForWasm} from "@util/artifacts"
import {deepCopy} from "@/utils/common"
import {wasmSingleOptimize} from "@/wasm/single_optimize"
import {createComputeResult} from "@/api/misc"
import {deviceIsPC} from "@util/device"
import SelectArtifact from "@/components/select/SelectArtifact.vue"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectCharacter from "@/components/select/SelectCharacter.vue"
import SelectCharacterLevel from "@/components/select/SelectCharacterLevel.vue"
import SelectWeapon from "@/components/select/SelectWeapon.vue"
import SelectWeaponLevel from "@/components/select/SelectWeaponLevel.vue"
import SelectTargetFunction from "@/components/select/SelectTargetFunction.vue"
import SelectCharacterSkill from "@/components/select/SelectCharacterSkill.vue"
import SelectBuff from "@/components/select/SelectBuff.vue"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import AddButton from "@c/misc/AddButton"
import DamagePanel from "./DamagePanel"
import AttributePanel from "@c/display/AttributePanel"
import ItemConfig from "@c/config/ItemConfig"
import BuffItem from "./BuffItem"
import WeaponDisplay from "@/components/display/WeaponDisplay.vue"
import SaveAsKumi from "./SaveAsKumi.vue"
import TransformativeDamage from "./TransformativeDamage"
import ValueDisplay from "./ValueDisplay"
import EnemyConfigComponent from "./EnemyConfig"
import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"
import ArtifactConfig from "./ArtifactConfig.vue"
import DamageAnalysis from "@/components/display/DamageAnalysis"
import {useCharacter, useCharacterSkill} from "@/composables/character"
import {useEnemy} from "@/composables/enemy"
import {useWeapon} from "@/composables/weapon"
import {useTargetFunction} from "@/composables/targetFunction"
import type {ArtifactPosition, IArtifact, IArtifactWasm} from "@/types/artifact"
import IconEpCaretRight from "~icons/ep/caret-right"
import IconEpTools from "~icons/ep/tools"
import IconEpPlus from "~icons/ep/plus"
import IconEpMenu from "~icons/ep/menu"
import IconEpHistogram from "~icons/ep/histogram"
import IconEpStarFilled from "~icons/ep/star-filled"
import IconEpFolder from "~icons/ep/folder"
import IconEpLock from "~icons/ep/lock"
import IconEpUnlock from "~icons/ep/unlock"
import {useComputeConstraint} from "@/composables/constraint"
import {BuffEntry, useBuff} from "@/composables/buff"
import {type PresetEntry, usePresetStore} from "@/store/pinia/preset"
import {useArtifactStore} from "@/store/pinia/artifact"
import type {IPreset} from "@/types/preset"
import {RandomIDProvider} from "@/utils/idProvider"
import {use5Artifacts} from "@/composables/artifact"
import {positions} from "@/constants/artifact"
import {positionToIndex} from "@/utils/artifacts"
import {useMona} from "@/wasm/mona"
import {useKumiStore} from "@/store/pinia/kumi"
import SimpleLoading from "@/components/loading/SimpleLoading.vue"
import SimpleError from "@/components/loading/SimpleError.vue"
import {useRoute} from "vue-router"
import {useI18n} from "@/i18n/i18n"

import {ElMessage} from "element-plus"
import "element-plus/es/components/message/style/css"
import SelectElementType from "@/components/select/SelectElementType.vue";

// stores
const presetStore = usePresetStore()
const artifactStore = useArtifactStore()
const kumiStore = useKumiStore()

// mona
const mona = await useMona()

// router
const route = useRoute()

// i18n
const { t } = useI18n()


//////////////////////////////////////////////////////////
// set preset from other place
function setPresetFromRoute() {
    const presetName = route.params.presetName
    if (presetName && typeof presetName === "string") {
        usePreset(presetName)
    }
}

onActivated(setPresetFromRoute)

onMounted(setPresetFromRoute)


////////////////////////////////////////////////////////
// enemy
const {
    enemyConfig,
    enemyInterface
} = useEnemy()
const showEnemyConfigDialog = ref(false)

function handleClickEnemyConfig() {
    showEnemyConfigDialog.value = true
}


////////////////////////////////////////////////////////
// character
const {
    characterName,
    characterLevel,
    characterConfig,
    characterSkill1,
    characterSkill2,
    characterSkill3,
    characterConstellation,
    characterWeaponType,
    characterLevelNumber,
    characterAscend,
    characterSplash,
    characterNeedConfig,
    characterConfigConfig,
    characterInterface,
} = useCharacter()

const {
    characterSkillConfig,
    characterSkillIndex,
    characterNeedSkillConfig,
    characterSkillConfigConfig,
    characterSkillInterface,
} = useCharacterSkill(characterName)

const fumo = ref("None")


//////////////////////////////////////////////////////////////
// weapon
const {
    weaponName,
    weaponLevel,
    weaponRefine,
    weaponConfig,
    weaponLevelNumber,
    weaponAscend,
    weaponSplash,
    weaponNeedConfig,
    weaponConfigConfig,
    weaponInterface,
} = useWeapon(characterWeaponType)


//////////////////////////////////////////////////////////////
// target function
const {
    targetFunctionName,
    targetFunctionConfig,
    targetFunctionUseDSL,
    targetFunctionDSLSource,
    targetFunctionBadge,
    targetFunctionDescription,
    targetFunctionNeedConfig,
    targetFunctionConfigConfig,
    targetFunctionInterface
} = useTargetFunction(characterName)
const miscTargetFunctionTab = ref<"normal" | "dsl">("normal")

watch(() => miscTargetFunctionTab.value, v => {
    targetFunctionUseDSL.value = v === "dsl"
})


//////////////////////////////////////////////////////////
// artifacts
const {
    artifactIds,
    artifactCount,
    artifactSingleConfig,
    artifactWasmFormat,

    artifactItems,
    artifactSetCount,
    artifactNeedConfig4,
    artifactConfig4ItemName,
    artifactEffect4Text,
    artifactConfig4Configs,
    artifactConfigForCalculator,

    setArtifact,
    removeArtifact,
} = use5Artifacts()

const showSelectArtifactDialog = ref(false)
const selectArtifactSlot = ref<ArtifactPosition>("flower")

function handleGotoSelectArtifact(index: number) {
    const slotName = positions[index]
    showSelectArtifactDialog.value = true
    selectArtifactSlot.value = slotName
}

function handleSelectArtifact(id: number) {
    const index = positionToIndex(selectArtifactSlot.value)
    setArtifact(index, id)

    showSelectArtifactDialog.value = false
}

function handleLockAll() {
    for (const id of artifactIds.value) {
        artifactStore.lockArtifact(id)
    }
}

function handleUnlockAll() {
    for (const id of artifactIds.value) {
        artifactStore.unlockArtifact(id)
    }
}

const isAllLocked = computed(() => {
    for (const id of artifactIds.value) {
        let artifact = artifactStore.getArtifact(id)
        if (artifact) {
            if (!artifact.omit) {
                return false
            }
        }
    }
    return true
})


//////////////////////////////////////////////////////////////
// optimization setup
const showConstraintDialog = ref(false)
const {
    algorithm,
    constraintArtifactSet,
    constraintSandMainStats,
    constraintGobletMainStats,
    constraintHeadMainStats,
    constraintMinRecharge,
    constraintMinElementalMastery,
    constraintMinCritical,
    constraintMinCriticalDamage,

    constraintSetMode,
    constraintInterface,
} = useComputeConstraint()
const filterKumiRef = ref<InstanceType<typeof ElTree> | null>(null)

function handleClickSetupOptimization() {
    showConstraintDialog.value = true
}


//////////////////////////////////////////////////////////////
// artifact config
const showConfigArtifactDialog = ref(false)
const artifactEffectMode = ref<"auto" | "custom">("auto")
const artifactConfig = ref(newDefaultArtifactConfigForWasm())

function handleClickArtifactConfig() {
    showConfigArtifactDialog.value = true
}


///////////////////////////////////////////////////////////////
// save and use presets
const miscCurrentPresetName = ref<null | string>(null)
const presetDefaultName = computed((): string => {
    // const cName = characterData[characterName.value].chs
    // const wName = weaponData[weaponName.value].chs
    const cName = t("character", characterName.value)
    const wName = t("weapon", weaponName.value)
    return `${cName}-${wName}`
})

function handleCommandPreset(cmd: string) {
    if (cmd === "save-preset") {
        handleClickSaveOptimizeConfig()
    } else {
        if (cmd.startsWith("apply-")) {
            const name = cmd.slice(6)

            usePreset(name)
        }
    }
}

function handleSavePreset(name: string) {
    if (!name) {
        handleClickSaveOptimizeConfig()
    } else {
        const item = getPresetItem()
        // console.log(item)
        item.name = name
        presetStore.addOrOverwrite(name, item)

        ElMessage.success({
            message: "已保存"
        })
    }
}

function handleClickSaveOptimizeConfig() {
    const item = getPresetItem()

    ElMessageBox.prompt("输入名称（重复名称将覆盖）", "存为预设", {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        inputPattern: /[^\s]+$/,
        inputValue: presetDefaultName.value
    }).then(({ value }) => {
        item.name = value
        presetStore.addOrOverwrite(value, item)
        miscCurrentPresetName.value = value
        ElMessage.success({
            message: "保存成功"
        })
    })
}

function getPresetItem() {
    type BuffType = Omit<BuffEntry, "id">
    let buffsToBeSaved: BuffType[] = []
    for (let buff of buffs.value) {
        buffsToBeSaved.push({
            name: buff.name,
            config: deepCopy(buff.config),
            lock: buff.lock
        })
    }

    const item = {
        // buffs: deepCopy(config.buffs),
        buffs: buffsToBeSaved,
        character: deepCopy(characterInterface.value),
        weapon: deepCopy(weaponInterface.value),
        targetFunction: deepCopy(targetFunctionInterface.value),
        constraint: {
            setNames: deepCopy(constraintArtifactSet.value),
            minRecharge: constraintMinRecharge.value,
            minCritical: constraintMinCritical.value,
            minCriticalDamage: constraintMinCriticalDamage.value,
            minElementalMastery: constraintMinElementalMastery.value
        },
        filter: {
            sandMainStats: deepCopy(constraintSandMainStats.value),
            gobletMainStats: deepCopy(constraintGobletMainStats.value),
            headMainStats: deepCopy(constraintHeadMainStats.value),
        },
        artifactConfig: deepCopy(artifactConfig.value),
        algorithm: algorithm.value,
        artifactEffectMode: artifactEffectMode.value,
        useDSL: miscTargetFunctionTab.value === "dsl",
        dslSource: targetFunctionDSLSource.value,
        name: ""
    } as IPreset
    return item
}

function usePreset(name: string) {
    const entry: PresetEntry = presetStore.presets.value[name]
    if (!entry || !entry.item) {
        return
    }

    const item: IPreset = deepCopy(entry.item)

    const idGenerator = new RandomIDProvider()

    // use buffs
    if (item.buffs) {
        const newBuffs: BuffItem[] = []
        for (let buff of item.buffs) {
            const newBuff: BuffEntry = {
                id: idGenerator.generateId(),
                name: buff.name,
                config: buff.config,
                lock: buff.lock
            }
            newBuffs.push(newBuff)
        }
        buffs.value = newBuffs
    }

    // use character
    const c = item.character
    if (c) {
        // this.characterName = c.name
        characterName.value = c.name
        characterLevel.value = c.level.toString() + (c.ascend ? "+" : "-")
        characterConstellation.value = c.constellation ?? 0
        characterSkill1.value = c.skill1 + 1
        characterSkill2.value = c.skill2 + 1
        characterSkill3.value = c.skill3 + 1
        characterConfig.value = c.params
    }

    // use weapon
    const w = item.weapon
    if (w) {
        weaponName.value = w.name
        weaponLevel.value = w.level.toString() + (w.ascend ? "+" : "-")
        weaponRefine.value = w.refine
        weaponConfig.value = w.params
    }

    // use target function
    const tf = item.targetFunction
    if (tf) {
        targetFunctionName.value = tf.name
        targetFunctionConfig.value = tf.params
    }

    // is DSL?
    const use_dsl = !!item.useDSL
    if (use_dsl) {
        miscTargetFunctionTab.value = "dsl"
        targetFunctionDSLSource.value = item.dslSource ?? ""
    } else {
        miscTargetFunctionTab.value = "normal"
    }


    // use constraint
    const constraint = item.constraint
    if (constraint) {
        constraintArtifactSet.value = constraint.setNames ?? []
        constraintMinCriticalDamage.value = constraint.minCriticalDamage ?? 0
        constraintMinCritical.value = constraint.minCritical ?? 0
        constraintMinElementalMastery.value = constraint.minElementalMastery ?? 0
        constraintMinRecharge.value = constraint.minRecharge ?? 1.0
    }

    // use filter
    const filter = item.filter
    if (filter) {
        constraintSandMainStats.value = filter.sandMainStats ?? []
        constraintGobletMainStats.value = filter.gobletMainStats ?? []
        constraintHeadMainStats.value = filter.headMainStats ?? []
    }

    // use compute mode
    algorithm.value = item.algorithm ?? "AStar"
    // disable heuristic algorithm in 5.11
    if (algorithm.value === "Heuristic") {
        algorithm.value = "AStar"
    }

    // use artifact effect mode
    artifactEffectMode.value = item.artifactEffectMode ?? "auto"

    // use artifact config
    artifactConfig.value = item.artifactConfig ?? newDefaultArtifactConfigForWasm()

    miscCurrentPresetName.value = name
}


///////////////////////////////////////////////////////////
// buffs
const {
    buffs,
    buffsInterface,
    addBuff,
    deleteBuff,
    toggleBuff
} = useBuff()

const showSelectBuffDialog = ref(false)

function handleClickAddBuff() {
    showSelectBuffDialog.value = true
}

function handleClickDeleteBuff(id: number) {
    deleteBuff(id)
}

function handleClickToggleBuff(id: number) {
    toggleBuff(id)
}

function handleSelectBuff(name: string) {
    showSelectBuffDialog.value = false
    addBuff(name)
}


////////////////////////////////////////////////////////////////////////
// damage
const showDamageAnalysisDialog = ref(false)
const damageAnalysisComponent = ref<null | InstanceType<typeof DamageAnalysis>>(null)

const damageAnalysisWasmInterface = computed(() => {
    // console.log("123")
    return {
        character: characterInterface.value,
        weapon: weaponInterface.value,
        buffs: buffsInterface.value,
        artifacts: artifactWasmFormat.value,
        artifact_config: artifactConfigForCalculator.value,
        skill: characterSkillInterface.value,
        enemy: enemyInterface.value,
    }
})

const characterDamageAnalysis = computed(() => {
    let fumo2 = null
    if (fumo.value !== "None") {
        fumo2 = fumo.value
    }
    const temp = mona.CalculatorInterface.get_damage_analysis(damageAnalysisWasmInterface.value, fumo2)
    // console.log(temp)
    return temp
})

const characterTransformativeDamage = computed(() => {
    // return  {
    //     "electro_charged": 0,
    //     "overload": 0,
    //     "shatter": 0,
    //     "superconduct": 0,
    //     "swirl_cryo": 0,
    //     "swirl_pyro": 0,
    //     "swirl_electro": 0,
    //     "swirl_hydro": 0
    // }
    const temp = mona.CalculatorInterface.get_transformative_damage(damageAnalysisWasmInterface.value)
    return temp
    // console.log(temp)
    // return temp
    // const ret = {
    //     "electro_charged": temp.electro_charged,
    //     "overload": temp.overload,
    //     "shatter": temp.shatter,
    //     "superconduct": temp.superconduct,
    //     "swirl_cryo": temp.swirl_cryo,
    //     "swirl_pyro": temp.swirl_pyro,
    //     "swirl_electro": temp.swirl_electro,
    //     "swirl_hydro": temp.swirl_hydro
    // }
    // return ret
})

function handleDisplayAnalysis() {
    showDamageAnalysisDialog.value = true

    nextTick(() => {
        if (damageAnalysisComponent.value) {
            const component = damageAnalysisComponent.value

            component.setValue(characterDamageAnalysis.value)
        }
    })
}


//////////////////////////////////////////////////////////////////
// attribute
const getAttributeWasmInterface = computed(() => {
    return {
        character: characterInterface.value,
        weapon: weaponInterface.value,
        buffs: buffsInterface.value,
        artifacts: artifactWasmFormat.value,
        artifact_config: artifactConfigForCalculator.value,
    }
})

const attributeFromWasm = computed(() => {
    return mona.CommonInterface.get_attribute(getAttributeWasmInterface.value)
})


//////////////////////////////////////////////////////////////////
// bonus analysis
const miscPerStatBonus = ref<any>({})
const showArtifactPerBonusDialog = ref(false)

const bonusPerStatWasmInterface = computed(() => {
    return {
        character: characterInterface.value,
        weapon: weaponInterface.value,
        artifacts: artifactWasmFormat.value,
        tf: targetFunctionInterface.value,
        buffs: buffsInterface.value,
        artifacts_config: artifactConfigForCalculator.value
    }
})

function handleClickAttributeAnalysis() {
    miscPerStatBonus.value = mona.BonusPerStat.bonus_per_stat(bonusPerStatWasmInterface.value)
    showArtifactPerBonusDialog.value = true
}

const ArtifactPerStatBonus = defineAsyncComponent({
    loader: () => import("@/components/display/ArtifactPerStatBonus.vue")
})


/////////////////////////////////////////////////////////////////////////
// kumi
const showSaveKumiDialog = ref(false)
const showUseKumiDialog = ref(false)

interface Node {
    label: string,
    children?: Node[],
    id: number,
}

const kumiDefaultName = computed((): string => {
    // let name = characterData[characterName.value].chs
    let name = t("character", characterName.value)
    for (const setName in artifactSetCount.value) {
        if (artifactSetCount.value[setName] >= 2) {
            // name += '-' + artifactsData[setName].chs
            name += '-' + t("artifact", setName, "setName")
        }
    }
    return name
})

const kumiTreeDataForElementUI = computed(() => {
    const data: Node[] = []
    for (const item of kumiStore.dirs.value) {
        const children = item.children ?? []

        if (children.length > 0) {
            const temp: Node[] = []
            for (let childId of children) {
                const item = kumiStore.kumiById.value.get(childId)
                if (item) {
                    temp.push({
                        label: item.title,
                        id: item.id
                    })
                }
            }

            const node = {
                label: item.title,
                children: temp,
                id: item.id
            }
            data.push(node)
        }
    }

    return data
})

function handleClickSaveAsKumi() {
    showSaveKumiDialog.value = true
}

function handleSaveAsKumi({ dirIds, name }: { dirIds: number[], name: string }) {
    showSaveKumiDialog.value = false

    for (const dirId of dirIds) {
        let kumiId = kumiStore.createKumi(dirId, name)
        if (kumiId) {
            for (const artifactId of artifactIds.value) {
                if (artifactId >= 0) {
                    kumiStore.addArtifact(kumiId, artifactId)
                }
            }
        }
    }
}

function handleClickUseKumi() {
    showUseKumiDialog.value = true
}

function handleUseKumi(node: Node) {
    const item = kumiStore.kumiById.value.get(node.id)
    if (!item || item.dir || !item.artifactIds) {
        return
    }

    showUseKumiDialog.value = false
    for (let i = 0; i < 5; i++) {
        const artifactId = item.artifactIds[i]
        if (artifactId !== null) {
            artifactIds.value[i] = artifactId
        } else {
            artifactIds.value[i] = -1
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// artifact analysis
const showArtifactAnalysisDialog = ref(false)

const ArtifactsSetStatistics = defineAsyncComponent({
    loader: () => import("@/components/display/ArtifactsSetStatistics"),
    loadingComponent: SimpleLoading,
    errorComponent: SimpleError,
})

function handleClickArtifactAnalysis() {
    showArtifactAnalysisDialog.value = true
}


////////////////////////////////////////////////////////////////////////////////
// optimization
interface ResultEntry {
    flower: null | number,
    feather: null | number,
    sand: null | number,
    goblet: null | number,
    head: null | number,
}

const optimizationResults = ref<ResultEntry[]>([])
const optimizationResultIndex = ref(0)

function handleUseNthOptimizationResult(n: number) {
    const result = optimizationResults.value[n - 1]
    const m = (x: null | number) => {
        if (x !== null) {
            return x
        } else {
            return -1
        }
    }

    let temp: number[] = []
    temp.push(m(result.flower))
    temp.push(m(result.feather))
    temp.push(m(result.sand))
    temp.push(m(result.goblet))
    temp.push(m(result.head))

    artifactIds.value = temp

    optimizationResultIndex.value = n
}

function getOptimizeArtifactWasmInterface() {
    let artifact_config: any = null
    if (artifactEffectMode.value === "custom") {
        artifact_config = artifactConfig.value
    }

    const i = {
        character: characterInterface.value,
        weapon: weaponInterface.value,
        target_function: targetFunctionInterface.value,
        constraint: constraintInterface.value,
        buffs: buffsInterface.value,
        artifact_config,
        algorithm: algorithm.value,
    }

    // some values may be under a proxy, which cannot be passed to web worker
    // use deep copy to remove proxy
    return deepCopy(i)
}

function getAllArtifactsFiltered(): IArtifact[] {
    const component = filterKumiRef.value

    // s is artifact ids to be filtered
    let s = new Set()

    // filter kumi
    if (component) {
        const nodes = component.getCheckedNodes(true)
        for (let node of nodes) {
            const kumiId = node.id
            const kumiItem = kumiStore.kumiById.value.get(kumiId)
            if (kumiItem && kumiItem.artifactIds) {
                for (const artifactId of kumiItem.artifactIds) {
                    s.add(artifactId)
                }
            }
        }
    }

    // filter main stat
    for (let artifact of artifactStore.artifacts.value.values()) {
        const position = artifact.position
        const mainStatName = artifact.mainTag.name
        const id = artifact.id
        if (position === "sand") {
            if (constraintSandMainStats.value.length > 0) {
                const index = constraintSandMainStats.value.indexOf(mainStatName)
                if (index === -1) {
                    s.add(id)
                }
            }
        } else if (position === "cup") {
            if (constraintGobletMainStats.value.length > 0) {
                const index = constraintGobletMainStats.value.indexOf(mainStatName)
                if (index === -1) {
                    s.add(id)
                }
            }
        } else if (position === "head") {
            if (constraintHeadMainStats.value.length > 0) {
                const index = constraintHeadMainStats.value.indexOf(mainStatName)
                if (index === -1) {
                    s.add(id)
                }
            }
        }
    }

    let filtered = []

    for (let artifact of artifactStore.artifacts.value.values()) {
        if (!artifact.omit && !s.has(artifact.id)) {
            filtered.push(artifact)
        }
    }

    return filtered.filter(a => !a.omit)
}

function getAllArtifactsFilteredWasm(): IArtifactWasm[] {
    return getAllArtifactsFiltered().map(x => convertArtifact(x))
}

function getArtifactsToBeCalculated(): IArtifactWasm[] {
    const artifacts = getAllArtifactsFilteredWasm()
    const artifacts16 = artifacts.filter(x => x.level >= 16)

    return artifacts16
}

function handleOptimizeArtifact() {
    const start = new Date()
    const loading = ElLoading.service({
        lock: true,
        fullscreen: true,
        text: "莫娜占卜中"
    })

    const optimizeInterface = getOptimizeArtifactWasmInterface()
    const artifacts = getArtifactsToBeCalculated()

    wasmSingleOptimize(optimizeInterface, artifacts).then(results => {
        const end = new Date()
        // @ts-ignore
        console.log(`time: ${(end - start) / 1000}s`)

        if (results.length === 0) {
            ElMessage.error({
                message: "没有符合条件的圣遗物"
            })
            return
        }
        optimizationResults.value = results
        handleUseNthOptimizationResult(1)

        // report best result to server, only report player whose 20 artifacts count is above 100
        if (artifactStore.artifacts20Count.value >= 100 && process.env.NODE_ENV === 'production') {
            let result_artifacts_wasm_format: any[] = []
            let first_result = results[0]
            result_artifacts_wasm_format.push(first_result.flower)
            result_artifacts_wasm_format.push(first_result.feather)
            result_artifacts_wasm_format.push(first_result.sand)
            result_artifacts_wasm_format.push(first_result.goblet)
            result_artifacts_wasm_format.push(first_result.head)
            result_artifacts_wasm_format = result_artifacts_wasm_format
                .filter(v => v !== null && v !== undefined)
                .map(id => artifactStore.artifacts.value.get(id))
                .filter(a => !!a)
                .map(a => convertArtifact(a))

            // the return value can be omitted, because there's nothing valuable
            createComputeResult(
                characterInterface.value,
                weaponInterface.value,
                buffsInterface.value,
                targetFunctionInterface.value,
                result_artifacts_wasm_format
            )
        }
    }).catch(e => {
        ElMessage.error({
            message: e.message ?? e
        })
    }).finally(() => {
        loading.close()
    })
}
</script>

<style lang="scss" scoped>
.big-container {
    @media only screen and (min-width: 992px) {
        .left-container, .middle-container, .right-container {
            height: calc(100vh - 24px * 2);
        }

        .left-container {
            padding-right: 12px;
        }

        .middle-container {
            padding-left: 12px;
            padding-right: 12px;
        }

        .right-container {
            // flex: 1;
            padding-left: 12px;
            padding-right: 12px;
            // overflow-y: auto;
            // overflow-x: hidden;
        }
    }

    @media only screen and (max-width: 992px) {
        .left-container, .middle-container {
            margin-bottom: 12px;
        }
    }
}

.middle-container {
    .artifacts {
        //display: flex;
        gap: 4px;
        //flex-wrap: wrap;
        //align-items: center;

        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
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
}

.config-character {
    //overflow: visible;
    //position: relative;

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

.common-description {
    font-size: 0.7rem;
    color: #606266;
}

 //artifact effect description title
.effect4 {
    color: #6eb7ff;
}

.constraint-main-stat-item {
    display: flex;
    align-items: center;
    margin-bottom: 12px;

    &:last-of-type {
        margin-bottom: 0;
    }

    span {
        width: 48px;
        font-size: 0.7rem;
    }
}

//@media only screen and (min-width: 992px) {
//    .constraint-min-item {
//        display: flex;
//        align-items: center;
//
//        .constraint-min-title {
//            font-size: 0.7rem;
//            width: 7vw;
//        }
//    }
//}
//
//@media only screen and (max-width: 992px) {
//    .constraint-min-item {
//        .constraint-min-title {
//            font-size: 0.7rem;
//            display: block;
//        }
//    }
//}

.constraint-min-item {
    .constraint-min-title {
        font-size: 0.7rem;
        display: block;
    }

    .slider-div {
        width: 100%;
    }
}
</style>