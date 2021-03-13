<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Artifacts Planning</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <my-step
            :steps="['角色', '角色参数', '武器', '武器参数', '目标', '目标参数', '配置', '结果']"
            :pointer="currentstep"
            @navigate="currentstep = $event"
        ></my-step>

        <div class="choose-div">
            <!-- <transition name="fade" mode="out-in"> -->
                <select-character
                    @select="handleSelectCharacter"
                    v-show="currentstep === 0"
                    class="step-div"
                ></select-character>

                <config-character
                    @select="handleSelectCharacterLevel"
                    v-show="currentstep === 1"
                    class="step-div"
                ></config-character>

                <select-weapon
                    :allow="characterWeapon"
                    @select="handleSelectWeapon"
                    v-show="currentstep === 2"
                    class="step-div"
                ></select-weapon>

                <config-weapon
                    :weaponName="selected.weaponName"
                    @select="handleSelectWeaponLevel"
                    v-show="currentstep === 3"
                    class="step-div"
                ></config-weapon>

                <select-target-function
                    :character-name="selected.characterName"
                    v-show="currentstep === 4"
                    class="step-div"
                    @select="handleSelectTargetFunction"
                ></select-target-function>

                <config-target-function
                    :target-func-name="selected.targetFuncName"
                    v-show="currentstep === 5"
                    class="step-div"
                    @select="handleConfigTargetFunc"
                ></config-target-function>

                <config
                    v-show="currentstep === 6"
                    @select="handleConfig"
                ></config>

                <result-page
                    v-show="currentstep === 7"
                    :calculating="calculating"
                    :result-data="resultData"
                    :config="config"
                    ref="resultPage"
                ></result-page>
            <!-- </transition> -->
        </div>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";
import { weaponsData } from "@asset/weapons";
// import { targetFunctionsData } from "@asset/target_functions";
// import compute from "@alg/compute_artifacts";
// import compute from "@alg/attribute_target/compute_artifacts_promise";
// import timer from "@util/timer";
import { toChs as estimateToChs } from "@util/time_estimate";

import SelectCharacter from "./steps/SelectCharacter";
import ConfigCharacter from "./steps/ConfigCharacter";
import SelectWeapon from "./steps/SelectWeapon";
import ConfigWeapon from "./steps/ConfigWeapon";
import SelectTargetFunction from "./steps/SelectTargetFunction";
import ConfigTargetFunction from "./steps/ConfigTargetFunction";
import Config from "./steps/Config";
import ResultPage from "./steps/ResultPage";

import MyStep from "@c/MyStep";

export default {
    name: "ArtifactsPlanPage",
    components: {
        SelectCharacter,
        ConfigCharacter,
        SelectWeapon,
        ConfigWeapon,
        SelectTargetFunction,
        ConfigTargetFunction,
        Config,
        ResultPage,
        MyStep
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

                constraintConfig: null,
            },

            resultData: {},

            currentstep: 0,
        }
    },
    methods: {
        /**
         * when a character is selected
         */
        handleSelectCharacter(name) {
            this.selected.characterName = name;

            this.currentstep++;
        },

        /**
         * when character level is selected
         */
        handleSelectCharacterLevel(item) {
            this.selected.characterLevel = item.level;
            this.selected.characterAscend = item.ascend;
            this.selected.characterSkill1 = item.skill1;
            this.selected.characterSkill2 = item.skill2;
            this.selected.characterSkill3 = item.skill3;
            this.selected.characterConstellation = item.constellation;

            this.currentstep++;
        },

        /**
         * when a weapon is selected
         */
        handleSelectWeapon(name) {
            this.selected.weaponName = name;

            this.currentstep++;
        },

        /**
         * when weapon level is selected
         */
        handleSelectWeaponLevel(config) {
            this.selected.weaponLevel = config.level;
            this.selected.weaponAscend = config.ascend;
            this.selected.weaponRefine = config.refine;
            this.selected.weaponArgs = config.args;

            this.currentstep++;
        },

        /**
         * when target function is selected
         */
        handleSelectTargetFunction(name) {
            this.selected.targetFuncName = name;

            this.currentstep++;
        },

        /**
         * when config target function
         */
        handleConfigTargetFunc(configData) {
            this.selected.targetFuncArgs = configData;

            this.currentstep++;
        },

        /**
         * when resctrictions are determined
         */
        handleConfig(config) {
            this.selected.constraintConfig = config;

            let iterCount = this.$store.getters.iterCount;
            if (iterCount >= 5000000) {
                this.$confirm(`计算将会非常耗时（约 ${estimateToChs(iterCount)}）,是否继续？`, "警告", {
                    confirmButtonText: "确定",
                    cancelButtonText: "取消",
                    type: "warning",
                }).then(() => {
                    this.currentstep++;
                    this.startCalculating();
                }).catch(() => {
                    return;
                })
            } else {
                this.currentstep++;
                this.startCalculating();
            }
        },

        /**
         * all configs are ready,
         * start to compute
         */
        startCalculating() {
            this.$refs.resultPage.doCompute();
        },

        /**
         * return an object representing all not omited artifacts
         */
        getArtifacts() {
            let allFlower = this.$store.state.flower;
            let allFeather = this.$store.state.feather;
            let allSand = this.$store.state.sand;
            let allCup = this.$store.state.cup;
            let allHead = this.$store.state.head;

            let fil = (item) => !item.omit;
            let flower = allFlower.filter(fil);
            let feather = allFeather.filter(fil);
            let sand = allSand.filter(fil);
            let cup = allCup.filter(fil);
            let head = allHead.filter(fil);

            return {
                flower,
                feather,
                sand,
                cup,
                head,
            };
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

        characterInfo() {
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

        weaponInfo() {
            return {
                name: this.selected.weaponName,
                level: this.selected.weaponLevel,
                ascend: this.selected.weaponAscend,
                refine: this.selected.weaponRefine,
                args: this.selected.weaponArgs,
            };
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
.choose-div {
    margin-top: 24px;
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