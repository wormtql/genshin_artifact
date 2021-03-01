<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>Artifacts Planning</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div v-loading.fullscreen.lock="calculating"></div>

        <my-step
            :steps="['角色', '角色参数', '武器', '武器参数', '目标', '目标参数', '配置', '结果']"
            :pointer="currentstep"
            @navigate="currentstep = $event"
        ></my-step>

        <div class="choose-div">
            <transition name="fade" mode="out-in">
                <select-character
                    @select="handleSelectCharacter"
                    v-if="currentstep === 0"
                    class="step-div"
                ></select-character>

                <select-character-level
                    @select="handleSelectCharacterLevel"
                    v-else-if="currentstep === 1"
                    class="step-div"
                ></select-character-level>

                <select-weapon
                    :allow="characterWeapon"
                    @select="handleSelectWeapon"
                    v-else-if="currentstep === 2"
                    class="step-div"
                ></select-weapon>

                <select-weapon-level
                    :weaponName="selected.weaponName"
                    @select="handleSelectWeaponLevel"
                    v-else-if="currentstep === 3"
                    class="step-div"
                ></select-weapon-level>

                <select-target-function
                    :character-name="selected.characterName"
                    v-else-if="currentstep === 4"
                    class="step-div"
                    @select="handleSelectTargetFunction"
                ></select-target-function>

                <config-target-function
                    :target-func-name="selected.targetFuncName"
                    v-else-if="currentstep === 5"
                    class="step-div"
                    @select="handleConfigTargetFunc"
                ></config-target-function>

                <config
                    v-else-if="currentstep === 6"
                    @select="handleConfig"
                ></config>

                <result-page
                    v-else-if="currentstep === 7"
                    :calculating="calculating"
                    :result-data="resultData"
                ></result-page>
            </transition>
        </div>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";
import { weaponsData } from "@asset/weapons";
// import { targetFunctionsData } from "@asset/target_functions";
// import compute from "@alg/compute_artifacts";
import compute from "@alg/compute_artifacts_promise";

import SelectCharacter from "./steps/SelectCharacter";
import SelectCharacterLevel from "./steps/SelectCharacterLevel";
import SelectWeapon from "./steps/SelectWeapon";
import SelectWeaponLevel from "./steps/SelectWeaponLevel";
import SelectTargetFunction from "./steps/SelectTargetFunction";
import ConfigTargetFunction from "./steps/ConfigTargetFunction";
import Config from "./steps/Config";
import ResultPage from "./steps/ResultPage";

import MyStep from "@c/MyStep";

export default {
    name: "ArtifactsPlanPage",
    components: {
        SelectCharacter,
        SelectCharacterLevel,
        SelectWeapon,
        SelectWeaponLevel,
        SelectTargetFunction,
        ConfigTargetFunction,
        Config,
        ResultPage,
        MyStep
    },
    data: function () {
        return {
            selected: {
                characterName: "",
                characterLevel: 1,
                characterAscend: false,
                characterSkill1: 6,
                characterSkill2: 6,
                characterSkill3: 6,
                characterConstellation: 0,

                weaponName: "",
                weaponLevel: 1,
                weaponAscend: false,
                weaponRefine: 1,
                weaponArgs: {},

                targetFuncName: "",
                targetFuncArgs: {},

                checkFunctionConfig: null,
            },

            resultData: {},
            calculating: false,

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
            this.selected.checkFunctionConfig = config;

            if (!this.$store.getters.valid) {
                this.$message.error("圣遗物数量过多，请禁用或删除明显更次的圣遗物");
                return;
            }

            this.currentstep++;
            this.startCalculating();
        },

        /**
         * every config is ready,
         * start to compute
         */
        startCalculating() {
            let character = {
                name: this.selected.characterName,
                level: this.selected.characterLevel,
                ascend: this.selected.characterAscend,
                skill1: this.selected.characterSkill1,
                skill2: this.selected.characterSkill2,
                skill3: this.selected.characterSkill3,
                constellation: this.selected.characterConstellation,
            };
            let weapon = {
                name: this.selected.weaponName,
                level: this.selected.weaponLevel,
                ascend: this.selected.weaponAscend,
                refine: this.selected.weaponRefine,
                args: this.selected.weaponArgs,
            };
            let artifacts = this.getArtifacts();
            let checkFuncConfig = this.selected.checkFunctionConfig;
            let targetFuncName = this.selected.targetFuncName;
            let targetFuncArgs = this.selected.targetFuncArgs;

            this.calculating = true;

            // this is a web worker wrapped by a promise
            compute(artifacts, character, weapon, targetFuncName, targetFuncArgs, checkFuncConfig).then(result => {
                this.resultData = {
                    artifacts: Object.values(result.combo),
                    value: result.value,
                    attribute: result.attribute,
                    error: result.error,
                };
                this.calculating = false;
            }).catch(reason => {
                this.$message.error("计算过程发生错误：" + reason);
            })
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

        characterWeapon() {
            if (this.selectedCharacterData) {
                return this.selectedCharacterData.weapon;
            }

            return "";
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