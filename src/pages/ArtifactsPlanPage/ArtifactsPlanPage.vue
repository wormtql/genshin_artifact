<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Artifacts Planning</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <apply-preset-dialog
            ref="applyPresetDialog"
            @confirm="handleConfirmApplyPreset">
        </apply-preset-dialog>

        <div class="tool-bar">
            <el-button
                @click="saveAsPreset"
                size="small"
            >
                <i class="el-icon-folder-add"></i>
                另存为预设
            </el-button>
            <el-button
                @click="handleClickApplyPreset"
                size="small"
            >
                <i class="el-icon-truck"></i>
                应用预设
            </el-button>
            <el-button
                v-show="isPreset"
                @click="savePreset"
                size="small"
                type="success"
            >
                <i class="el-icon-check"></i>
                保存预设：{{ currentPresetName }}
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
                    <li :class="{active: currentstep === 'constraint'}" x-name="constraint">过滤/限定</li>
                    <li :class="{active: currentstep === 'buff'}" x-name="buff">全局buff</li>
                    <li :class="{active: currentstep === 'result'}" x-name="result">计算结果</li>
                    <li :class="{active: currentstep === 'calculator'}" x-name="calculator">计算器</li>
                </ul>
            </el-col>
            <el-col :span="20">
                <div class="choose-div">
                    <select-character
                        v-show="currentstep === 'character'"
                        class="step-div"
                        ref="selectCharacter"
                    ></select-character>

                    <config-character
                        v-show="currentstep === 'character-config'"
                        class="step-div"
                        ref="configCharacter"
                    ></config-character>

                    <select-weapon
                        v-show="currentstep === 'weapon'"
                        class="step-div"
                        ref="selectWeapon"
                    ></select-weapon>

                    <config-weapon
                        v-show="currentstep === 'weapon-config'"
                        class="step-div"
                        ref="configWeapon"
                    ></config-weapon>

                    <select-target-function
                        v-show="currentstep === 'target-func'"
                        class="step-div"
                        ref="selectTargetFunc"
                    ></select-target-function>

                    <config-target-function
                        v-show="currentstep === 'target-func-config'"
                        class="step-div"
                        ref="configTargetFunc"
                    ></config-target-function>

                    <config
                        v-show="currentstep === 'constraint'"
                        ref="constraint"
                    ></config>

                    <config-buff
                        v-show="currentstep === 'buff'"
                        ref="configBuff"
                    >
                    </config-buff>

                    <result-page
                        v-show="currentstep === 'result'"
                        :calculating="calculating"
                        :result-data="resultData"
                        ref="resultPage"
                    ></result-page>

                    <damage-calculator
                        v-show="currentstep === 'calculator'"
                        name="keqing"
                        ref="calculator"
                    ></damage-calculator>
                </div>
            </el-col>
        </el-row>
        
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";
// import { weaponsData } from "@asset/weapons";
// import deepCopy from "@util/deepcopy"; 
import { toChs as estimateToChs } from "@util/time_estimate";
import createFilterFunction from "@alg/attribute_target/create_filter_function";
// import { count as countArtifacts } from "@util/artifacts";

import ResultPage from "./steps/ResultPage";
import DamageCalculator from "./steps/DamageCalculator";

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
        "ConfigBuff": () => import(/* webpackChunkName: "steps-select-buff" */ "./steps/ConfigBuff"),
        "Config": () => import(/* webpackChunkName: "steps-constraint" */ "./steps/Config"),
        // "DamageCalculator": () => import(/* webpackChunkName: "damage-calculator" */)
        ResultPage,
        DamageCalculator,
        // MyStep,
        ApplyPresetDialog,
    },
    provide() {
        return {
            notifyChange: this.notifyChange,
            getConfigObject: this.getConfigObject,
        }
    },
    created() {
        this.lock = false;
    },
    data: function () {
        return {
            currentstep: "character",
            // lock: false,

            isPreset: false,
            currentPresetName: "",
        }
    },
    methods: {
        handleClickApplyPreset() {
            let count = this.$store.getters["presets/count"];
            if (count === 0) {
                this.$message({
                    message: "没有可用的预设，请先保存一个预设",
                    type: "warning",
                });
                return;
            }
            this.$refs.applyPresetDialog.open();
        },

        notifyChange(type, value) {
            console.log(type, value);
            if (type === "character") {
                let weaponType = charactersData[value].weapon;
                this.$refs.selectWeapon.setAllow(weaponType);
                this.$refs.selectTargetFunc.setCharacterName(value);
                this.$refs.configCharacter.setCharacterName(value);
            } else if (type === "weapon") {
                this.$refs.configWeapon.setWeaponName(value);
            } else if (type === "targetFunc") {
                this.$refs.configTargetFunc.setTargetFuncName(value);
            }
        },

        handleNav(e) {
            let navTarget = e.target.attributes["x-name"].value;
            if (navTarget === "calculator") {
                this.$refs.calculator.updateConfigObject();
            }
            this.currentstep = navTarget;
        },

        handleConfirmApplyPreset(name) {
            // console.log(name);
            this.currentPresetName = name;
            this.isPreset = true;

            this.applyPreset(name);
        },

        applyPreset(name) {
            let preset = this.$store.getters["presets/all"][name];
            
            // set character
            this.$refs.selectCharacter.setCharacterName(preset.character.name);
            this.$refs.configCharacter.setCharacterConfig(preset.character);

            // set weapon
            let allowWeapon = charactersData[preset.character.name].weapon;
            this.$refs.selectWeapon.setWeaponName(allowWeapon, preset.weapon.name);
            this.$refs.configWeapon.setWeaponConfig(preset.weapon);

            // set target func
            this.$refs.selectTargetFunc.setTargetFuncName(preset.character.name, preset.targetFunc.name);
            this.$refs.configTargetFunc.setTargetFuncConfig(preset.targetFunc);

            // set constraint
            this.$refs.constraint.setConstraint(preset.constraint);
            
            // set buffs
            this.$refs.configBuff.setBuffs(preset.buffs ?? []);

            // update calculator
            this.$nextTick(() => {
                this.$refs.calculator.updateConfigObject();
            });
            

            this.$message({
                type: "success",
                message: "应用成功",
            });
        },

        savePreset() {
            let preset = this.getPresetObject();
            preset.name = this.currentPresetName;

            this.$store.commit("presets/update", preset);

            this.$message({
                type: "success",
                message: "保存成功",
            })
        },

        getPresetObject() {
            // version 2
            let obj = {
                character: this.getCharacterInfo(),
                weapon: this.getWeaponInfo(),
                targetFunc: this.getTargetFuncInfo(),
                constraint: this.getConstraint(),
                buffs: this.getBuffs(),
            };

            // console.log(obj);

            return obj;
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
                });

                this.isPreset = true;
                this.currentPresetName = value;
            }).catch(() => {});
        },


        getCharacterInfo() {
            return {
                name: this.$refs.selectCharacter.getCharacterName(),
                ...this.$refs.configCharacter.getCharacterConfig(),
            };
        },

        getWeaponInfo() {
            return {
                name: this.$refs.selectWeapon.getWeaponName(),
                ...this.$refs.configWeapon.getWeaponConfig(),
            };
        },

        getTargetFuncInfo() {
            return {
                name: this.$refs.selectTargetFunc.getTargetFuncName(),
                args: this.$refs.configTargetFunc.getTargetFuncConfig(),
            };
        },

        getConstraint() {
            return this.$refs.constraint.getConstraint();
        },

        getStandardBuffs() {
            return this.$refs.configBuff.getStandardBuffs();
        },

        getBuffs() {
            return this.$refs.configBuff.getBuffs();
        },

        getConfigObject() {
            return {
                character: this.getCharacterInfo(),
                weapon: this.getWeaponInfo(),
                targetFunc: this.getTargetFuncInfo(),
                constraint: this.getConstraint(),
                buffs: this.getStandardBuffs(),
            };
        },

        /**
         * all configs are ready,
         * start to compute
         */
        startCalculating() {
            let configObject = this.getConfigObject();

            let rawArtifacts = this.$store.getters["artifacts/notOmittedArtifacts"];

            let filter = createFilterFunction(configObject.constraint);
            let filteredArtifacts = filter(rawArtifacts);
            let iterCount
                = (filteredArtifacts.flower.length || 1)
                * (filteredArtifacts.feather.length || 1)
                * (filteredArtifacts.sand.length || 1)
                * (filteredArtifacts.cup.length || 1)
                * (filteredArtifacts.head.length || 1)
                ;
            // console.log(configObject);

            let start = () => {
                this.currentstep = "result";
                this.$refs.resultPage.doCompute(configObject);
            };

            console.log(iterCount);
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
        // config() {
        //     return {
        //         cArgs: this.characterInfo,
        //     }
        // }
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