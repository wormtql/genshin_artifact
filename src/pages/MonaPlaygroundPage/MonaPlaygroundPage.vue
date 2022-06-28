<template>
    <el-row>
        <el-dialog
            v-model="showSelectBuffDialog"
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
            v-model="showSelectArtifactDialog"
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
                        <h3 class="common-title2">命之座</h3>
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
                        @delete="deleteBuff(buff.id)"
                        @toggle="toggleBuff(buff.id)"
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
                            @delete="removeArtifact(index)"
                            @toggle="toggleArtifact(id)"
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
                    <el-button-group>
                        <el-button
                            :icon="IconFa6SolidPlay"
                            @click="handleClickRun"
                            size="small"
                            text
                            title="运行"
                            type="primary"
                        >运行</el-button>
                        <el-button
                            :icon="IconFa6SolidBan"
                            @click="handleClickClearOutput"
                            text
                            title="清除输出"
                            size="small"
                        ></el-button>
                    </el-button-group>

<!--                    <my-button1 icon="el-icon-caret-right" title="运行" @click="handleClickRun()"></my-button1>-->
<!--                    <my-button1 icon="el-icon-delete-solid" title="清除输出" @click="handleClickClearOutput"></my-button1>-->
                </div>

                <div class="output mona-scroll-hidden">
                    <p v-for="output in outputs" class="output-row">{{ output }}</p>
                </div>
            </div>
        </el-col>
    </el-row>
</template>

<script setup lang="ts">
import ItemConfig from "@c/config/ItemConfig"
import SelectCharacter from "@c/select/SelectCharacter"
import SelectWeapon from "@c/select/SelectWeapon"
import SelectCharacterLevel from "@c/select/SelectCharacterLevel"
import SelectWeaponLevel from "@c/select/SelectWeaponLevel"
import WeaponDisplay from "@c/display/WeaponDisplay"
import BuffItem from "@page/NewArtifactPlanPage/BuffItem"
import SelectBuff from "@c/select/SelectBuff"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import AddButton from "@c/misc/AddButton"
import SelectArtifact from "@c/select/SelectArtifact"
import MonaMonacoEditor from "@/components/MonaMonacoEditor.vue"

import {deviceIsPC} from "@util/device"
import {useCharacter} from "@/composables/character"
import {useWeapon} from "@/composables/weapon"
import {useBuff} from "@/composables/buff"
import {use5Artifacts} from "@/composables/artifact"
import type {ArtifactPosition} from "@/types/artifact"
import {positionToIndex} from "@/utils/artifacts"

import IconEpPlus from "~icons/ep/plus"
import IconFa6SolidBan from "~icons/fa6-solid/ban"
import IconFa6SolidPlay from "~icons/fa6-solid/play"
import {positions} from "@/constants/artifact"
import {useMona} from "@/wasm/mona"


//////////////////////////////////////////////////////////////////////////
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
    characterInterface,
    characterSplash,
    characterAscend,
    characterConfigConfig,
    characterNeedConfig
} = useCharacter()


//////////////////////////////////////////////////////////////////////////
// weapon
const {
    weaponName, weaponLevel, weaponRefine, weaponConfig,
    weaponNeedConfig,
    weaponConfigConfig,
    weaponAscend,
    weaponInterface,
} = useWeapon(characterWeaponType)


//////////////////////////////////////////////////////////////////////////
// buffs
const {
    buffs,
    buffsInterface,
    addBuff,
    deleteBuff,
    toggleBuff
} = useBuff()
const showSelectBuffDialog = ref(false)

function handleSelectBuff(name: string) {
    showSelectBuffDialog.value = false
    addBuff(name)
}

function handleClickAddBuff() {
    showSelectBuffDialog.value = true
}


//////////////////////////////////////////////////////////////////////////
// artifacts
const {
    artifactIds,
    artifactSingleConfig,
    artifactConfigForCalculator,
    artifactWasmFormat,
    artifactNeedConfig4,
    artifactConfig4Configs,
    artifactItems,

    removeArtifact,
    toggleArtifact,
    setArtifact,
} = use5Artifacts()
const showSelectArtifactDialog = ref(false)
const selectArtifactSlot = ref<ArtifactPosition>("flower")

function handleSelectArtifact(id: number) {
    const index = positionToIndex(selectArtifactSlot.value)
    artifactIds.value[index] = id

    showSelectArtifactDialog.value = false
}

function handleGotoSelectArtifact(index: number) {
    const slotName = positions[index]
    showSelectArtifactDialog.value = true
    selectArtifactSlot.value = slotName
}


//////////////////////////////////////////////////////////////////////////
// dsl
const outputs = ref<string[]>([])
const editor = ref<null | InstanceType<typeof MonaMonacoEditor>>(null)

const runInputInterface = computed(() => {
    return {
        character: characterInterface.value,
        weapon: weaponInterface.value,
        buffs: buffsInterface.value,
        artifact_config: artifactConfigForCalculator.value,
        enemy: null, // todo
    }
})

function appendOutput(s: string) {
    outputs.value.push(s)
}

function handleClickClearOutput() {
    outputs.value = []
}

async function handleClickRun() {
    if (editor.value) {
        const x = runInputInterface.value
        const artifacts = artifactWasmFormat.value
        const source = editor.value.getValue().trim()

        const mona = await useMona()

        const ret = mona.DSLInterface.run(source, x, artifacts)

        if (ret.is_error) {
            appendOutput("[error]")
            for (const item of ret.error_msg.split("\n")) {
                appendOutput(item)
            }
        } else {
            appendOutput("[success]")
            for (const item of ret.output.split("\n")) {
                appendOutput(item)
            }
        }
    }
}
</script>

<style scoped lang="scss">
$bottom-height: 300px;

.editor-container {
    height: calc(100% - #{$bottom-height})
    //height: 300px;
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
    height: $bottom-height;
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
    padding-left: 12px;

    //.el-button {
    //    height: 32px;
    //    width: 32px;
    //}
}

</style>