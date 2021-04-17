<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Artifacts Planning</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <apply-preset-dialog ref="applyPresetDialog" @confirm="handleConfirmApplyPreset">
        </apply-preset-dialog>

        <div class="tool-bar">
            <el-button
                @click="saveAsPreset"
                size="small"
            >
                <i class="el-icon-folder-add"></i>
                存为预设
            </el-button>
            <el-button
                @click="$refs.applyPresetDialog.open()"
                size="small"
            >
                <i class="el-icon-truck"></i>
                应用预设
            </el-button>

            <div style="float: right">
                <el-button
                    size="small"
                    @click="startCalculating"
                    type="primary"
                >
                    <i class="el-icon-cpu"></i>
                    开始计算
                </el-button>
            </div>
        </div>

        <el-row :gutter="16">
            <el-col :span="4">
                <ul class="step" @click="handleNav">
                    <li :class="{active: currentstep === 'character'}" x-name="character">角色</li>
                    <li :class="{active: currentstep === 'character-config'}" x-name="character-config">角色参数</li>
                    <li :class="{active: currentstep === 'weapon'}" x-name="weapon">武器</li>
                    <li :class="{active: currentstep === 'weapon-config'}" x-name="weapon-config">武器参数</li>
                    <li :class="{active: currentstep === 'target-func'}" x-name="target-func">目标函数</li>
                    <li :class="{active: currentstep === 'target-func-config'}" x-name="target-func-config">目标函数参数</li>
                    <li :class="{active: currentstep === 'constraint'}" x-name="constraint">限制</li>
                    <li :class="{active: currentstep === 'buff'}" x-name="buff">全局buff</li>
                    <li :class="{active: currentstep === 'result'}" x-name="result">计算结果</li>
                </ul>
            </el-col>
            <el-col :span="20">
                <div class="choose-div">
                    <select-character
                        v-model="selected.characterName"
                        v-show="currentstep === 'character'"
                        class="step-div"
                    ></select-character>

                    <config-character
                        v-model="characterInfo"
                        v-show="currentstep === 'character-config'"
                        class="step-div"
                    ></config-character>

                    <select-weapon
                        :allow="characterWeapon"
                        v-model="selected.weaponName"
                        v-show="currentstep === 'weapon'"
                        class="step-div"
                    ></select-weapon>

                    <config-weapon
                        :weaponName="selected.weaponName"
                        v-model="weaponInfo"
                        v-show="currentstep === 'weapon-config'"
                        class="step-div"
                    ></config-weapon>

                    <select-target-function
                        :character-name="selected.characterName"
                        v-model="selected.targetFuncName"
                        v-show="currentstep === 'target-func'"
                        class="step-div"
                    ></select-target-function>

                    <config-target-function
                        :target-func-name="selected.targetFuncName"
                        v-show="currentstep === 'target-func-config'"
                        class="step-div"
                        v-model="targetFuncInfo"
                    ></config-target-function>

                    <config
                        v-model="selected.constraintConfig"
                        v-show="currentstep === 'constraint'"
                    ></config>

                    <result-page
                        v-show="currentstep === 'result'"
                        :calculating="calculating"
                        :result-data="resultData"
                        :config="config"
                        ref="resultPage"
                    ></result-page>
                </div>
            </el-col>
        </el-row>
        
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";
import { weaponsData } from "@asset/weapons";
import deepCopy from "@util/deepcopy"; 
import { toChs as estimateToChs } from "@util/time_estimate";

// import SelectCharacter from "./steps/SelectCharacter";
// import ConfigCharacter from "./steps/ConfigCharacter";
// import SelectWeapon from "./steps/SelectWeapon";
// import ConfigWeapon from "./steps/ConfigWeapon";
// import SelectTargetFunction from "./steps/SelectTargetFunction";
// import ConfigTargetFunction from "./steps/ConfigTargetFunction";
import Config from "./steps/Config";
import ResultPage from "./steps/ResultPage";

// import MyStep from "@c/MyStep";

import ApplyPresetDialog from "./ApplyPresetDialog";

export default {
    name: "ArtifactsPlanPage",
    components: {
        "select-character": () => import(/* webpackChunkName: "steps-select-c" */ "./steps/SelectCharacter"),
        "ConfigCharacter": () => import(/* webpackChunkName: "steps-select-c" */ "./steps/ConfigCharacter"),
        "SelectWeapon": () => import(/* webpackChunkName: "steps-select-w" */ "./steps/SelectWeapon"),
        "ConfigWeapon": () => import(/* webpackChunkName: "steps-select-w" */ "./steps/ConfigWeapon"),
        "SelectTargetFunction": () => import(/* webpackChunkName: "steps-select-t" */ "./steps/SelectTargetFunction"),
        "ConfigTargetFunction": () => import(/* webpackChunkName: "steps-select-t" */ "./steps/ConfigTargetFunction"),
        Config,
        ResultPage,
        // MyStep,
        ApplyPresetDialog,
    },
    created() {
        this.lock = false;
    },
    data: function () {
        return {
            selected: {
                characterName: "anbo",
                characterLevel: 1,
                characterAscend: false,
                characterSkill1: 6,
                characterSkill2: 6,
                characterSkill3: 6,
                characterConstellation: 0,

                weaponName: "liegong",
                weaponLevel: 1,
                weaponAscend: false,
                weaponRefine: 1,
                weaponArgs: {},

                targetFuncName: "single",
                targetFuncArgs: { fieldName: "attack" },

                constraintConfig: {
                    constraintSet: {
                        mode: "any",
                        setName1: "berserker",
                        setName2: "berserker",
                        setName3: "berserker",
                        setName4: "berserker",
                    },
                    constraintMainTag: {
                        sand: "any",
                        cup: "any",
                        head: "any",
                    }
                },
            },

            // resultData: {},

            currentstep: "character",
            // lock: false,
        }
    },
    methods: {
        handleNav(e) {
            this.currentstep = e.target.attributes["x-name"].value;
        },

        handleConfirmApplyPreset(name) {
            // console.log(name);
            this.applyPreset(name);
        },

        applyPreset(name) {
            this.lock = true;

            let preset = this.$store.getters["presets/all"][name];
            this.selected.characterName = preset.cName;
            this.selected.characterLevel = preset.cLevel;
            this.selected.characterAscend = preset.cAscend;
            this.selected.characterSkill1 = preset.cS1;
            this.selected.characterSkill2 = preset.cS2;
            this.selected.characterSkill3 = preset.cS3;
            this.selected.characterConstellation = preset.cC;

            this.selected.weaponName = preset.wName;
            this.selected.weaponLevel = preset.wLevel;
            this.selected.weaponAscend = preset.sAscend;
            this.selected.weaponRefine = preset.wRefine;
            this.selected.weaponArgs = deepCopy(preset.wArgs);

            this.selected.targetFuncName = preset.tName;
            this.selected.targetFuncArgs = deepCopy(preset.tArgs);

            this.$message({
                type: "success",
                message: "应用成功",
            });

            this.$nextTick(() => {
                this.lock = false;
            });
        },

        getPresetObject() {
            return {
                cName: this.selected.characterName,
                cLevel: this.selected.characterLevel,
                cAscend: this.selected.characterAscend,
                cS1: this.selected.characterSkill1,
                cS2: this.selected.characterSkill2,
                cS3: this.selected.characterSkill3,
                cC: this.selected.characterConstellation,

                wName: this.selected.weaponName,
                wLevel: this.selected.weaponLevel,
                wAscend: this.selected.weaponAscend,
                wRefine: this.selected.weaponRefine,
                wArgs: deepCopy(this.selected.weaponArgs),

                tName: this.selected.targetFuncName,
                tArgs: deepCopy(this.selected.targetFuncArgs),
            };
        },

        checkPresetNameDuplicate(name) {
            let presets = this.$store.getters["presets/all"];
            return !Object.prototype.hasOwnProperty.call(presets, name)
        },

        saveAsPreset() {
            this.$prompt("取个名字吧", "存为预设", {
                confirmButtonText: "确定",
                cancelButtonText: "取消",
            }).then(({ value }) => {
                if (!value) {
                    this.$message.error("名字不能为空哦");
                    return;
                }
                if (!this.checkPresetNameDuplicate(value)) {
                    this.$message.error("名字已经存在");
                    return;
                }

                let preset = this.getPresetObject();
                preset.name = value;
                this.$store.commit("presets/add", {
                    name: value,
                    value: preset,
                })
            }).catch(() => {});
        },

        /**
         * all configs are ready,
         * start to compute
         */
        startCalculating() {
            let configObject = {
                character: this.characterInfo,
                weapon: this.weaponInfo,
                targetFunc: {
                    name: this.selected.targetFuncName,
                    args: this.selected.targetFuncArgs,
                },
                constraint: this.selected.constraintConfig,
            };

            let start = () => {
                this.currentstep = "result";
                this.$refs.resultPage.doCompute(configObject);
            };

            let iterCount = this.$store.getters["artifacts/iterCount"];
            if (iterCount >= 5000000) {
                this.$confirm(`计算将会非常耗时（约 ${estimateToChs(iterCount)}）,是否继续？`, "警告", {
                    confirmButtonText: "确定",
                    cancelButtonText: "取消",
                    type: "warning",
                }).then(() => {
                    start();
                }).catch(() => {
                    return;
                })
            } else {
                start();
            }
        },
    },
    computed: {
        selectedCharacterData() {
            return charactersData[this.selected.characterName];
        },

        selectedWeaponData() {
            return weaponsData[this.selected.weaponName];
        },

        // which weapon type the selected character will use
        characterWeapon() {
            if (this.selectedCharacterData) {
                return this.selectedCharacterData.weapon;
            }

            return "none";
        },

        characterInfo: {
            get() {
                return {
                    name: this.selected.characterName,
                    level: this.selected.characterLevel,
                    ascend: this.selected.characterAscend,
                    skill1: this.selected.characterSkill1,
                    skill2: this.selected.characterSkill2,
                    skill3: this.selected.characterSkill3,
                    constellation: this.selected.characterConstellation,
                };
            },
            set(value) {
                // console.log(value);
                this.selected.characterLevel = value.level;
                this.selected.characterAscend = value.ascend;
                this.selected.characterSkill1 = value.skill1;
                this.selected.characterSkill2 = value.skill2;
                this.selected.characterSkill3 = value.skill3;
                this.selected.characterConstellation = value.constellation;
            }
        },

        weaponInfo: {
            get() {
                let temp = {
                    name: this.selected.weaponName,
                    level: this.selected.weaponLevel,
                    ascend: this.selected.weaponAscend,
                    refine: this.selected.weaponRefine,
                    args: this.selected.weaponArgs,
                };
                return temp;
            },
            set(value) {
                this.selected.weaponLevel = value.level;
                this.selected.weaponAscend = value.ascend;
                this.selected.weaponRefine = value.refine;
                this.selected.weaponArgs = value.args;
            }
        },

        targetFuncInfo: {
            get() {
                return {
                    name: this.selected.targetFuncName,
                    args: this.selected.targetFuncArgs,
                }
            },

            set(value) {
                this.selected.targetFuncArgs = value.args;
            }
        },

        config() {
            return {
                cArgs: this.characterInfo,
            }
        }
    }
}
</script>

<style scoped>
.step {
    list-style: none;
    margin: 16px 0;
    padding: 0;
    background: #f5f7fa;
    border-radius: 3px;
}

.step li {
    padding: 8px 16px;
    cursor: pointer;
    font-size: 14px;
    color: #409eff;
    border-bottom: 1px solid #00000011;
    transition: 300ms;
}

.step li:hover {
    color: #6eb7ff;
}

.step li:last-of-type {
    border: none;
}

.step li.active {
    box-shadow: 0 0 20px 3px rgb(0 0 0 / 10%);
}

.tool-bar {
    margin-bottom: 16px;
}

.choose-div {
    margin-top: 16px;
}

.fade-enter-active, .fade-leave-active {
    transition: 150ms ease;
}

.fade-enter {
    opacity: 0;
    /* transform: translateX(100%); */
}

.fade-leave-to {
    opacity: 0;
    /* transform: translateX(-100%); */
}
</style>