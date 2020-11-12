<template>
    <div>
        <result-dialog
            :show="showResultDialog"
            @close="showResultDialog = false"
            :combo="resultCombo"
            :resultValue="resultValue"
            :deritive="resultDeritive"
        ></result-dialog>

        <el-breadcrumb>
            <el-breadcrumb-item>Genshin Artifacts Planner</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row :gutter="16">
            <el-col :span="17">
                <el-radio-group v-model="current" style="margin-bottom: 16px">
                    <el-radio-button label="character">角色</el-radio-button>
                    <el-radio-button label="weapon">武器</el-radio-button>
                    <el-radio-button label="target">目标函数</el-radio-button>
                </el-radio-group>

                <select-character v-model="selectedCharacter" v-show="current === 'character'"></select-character>

                <select-weapon v-model="selectedWeapon" v-show="current === 'weapon'"></select-weapon>

                <select-target v-model="selectedTarget" v-show="current === 'target'"></select-target>


                <el-divider></el-divider>
                <div style="box-shadow: 0 2px 4px rgba(0,0,0,.12), 0 0 6px rgba(0,0,0,.04); padding: 16px; border: 1px solid #DCDFE6">
                    <el-row>
                    <el-col :span="12">
                        <p>已选角色属性：</p>
                        <preview-item :value="selectedCharacterAttribute">
                        </preview-item>
                    </el-col>
                    <el-col :span="12">
                        <p>已选武器属性：</p>
                        <preview-item :value="selectedWeaponAttribute">
                        </preview-item>
                    </el-col>
                </el-row>
                </div>
                

                <el-button
                    type="primary"
                    style="margin-top: 32px; width: 100%"
                    @click="doCalc"
                    :loading="isComputing"
                >
                    计算
                </el-button>
            </el-col>

            <el-col :span="7">
                <h3>
                    面板预览
                </h3>
                <attribute-panel :panel="resultAttribute" v-if="isCalculated">
                </attribute-panel>
                <el-alert :closable="false" title="请先进行一次运算" v-else type="warning">
                </el-alert>
            </el-col>
        </el-row>
        
    </div>
</template>

<script>
import SelectCharacter from "@/components/SelectCharacter";
import SelectWeapon from "@/components/SelectWeapon";
import SelectTarget from "@/components/SelectTarget";
import ResultDialog from "./ResultDialog";
import AttributePanel from "@/components/AttributePanel";
import PreviewItem from "@/components/PreviewItem";

import { plans as plansPreset, getTargetFunction } from "@/common/target";
import { compose, getCharacterAttribute, getWeaponAttribute } from "genshin_panel";
import { convertArtifact } from "@/utils/common";
import { mapState } from "vuex";
import { characterPreset, weaponPreset } from "@/common/basic";

export default {
    name: "CalculatePage",
    components: {
        ResultDialog,
        AttributePanel,
        SelectCharacter,
        SelectWeapon,
        SelectTarget,
        PreviewItem,
    },
    data: function() {
        return {
            plansPreset,

            showResultDialog: false,

            // 计算结果
            resultCombo: [],
            resultValue: -1,
            resultAttribute: {},
            resultDeritive: [],

            // 是否正在计算
            isComputing: false,

            // 是否已经计算过
            isCalculated: false,

            characterPreset, weaponPreset,

            selectedCharacter: "keqing-70-0",
            selectedWeapon: "heijian-70-0",
            selectedTarget: "keqing-normal",

            current: "character",
        }
    },
    methods: {
        // 计算最优解
        calc() {
            const storeState = this.$store.state;

            let tempArts = {
                "flower": storeState.flower.filter(item => !item.omit).map(item => convertArtifact(item)),
                "feather": storeState.feather.filter(item => !item.omit).map(item => convertArtifact(item)),
                "cup": storeState.cup.filter(item => !item.omit).map(item => convertArtifact(item)),
                "head": storeState.head.filter(item => !item.omit).map(item => convertArtifact(item)),
                "sand": storeState.sand.filter(item => !item.omit).map(item => convertArtifact(item)),
            };
            if (tempArts.flower.length === 0) {
                tempArts.flower.push(null);
            }
            if (tempArts.feather.length === 0) {
                tempArts.feather.push(null);
            }
            if (tempArts.cup.length === 0) {
                tempArts.cup.push(null);
            }
            if (tempArts.head.length === 0) {
                tempArts.head.push(null);
            }
            if (tempArts.sand.length === 0) {
                tempArts.sand.push(null);
            }

            let cupSize = tempArts.cup.length;
            let flowerSize = tempArts.flower.length;
            let featherSize = tempArts.feather.length;
            let sandSize = tempArts.sand.length;
            let headSize = tempArts.head.length;
            
            // let targetFunction = getTargetFunction(this.selectedTargetFunction);
            let maxCombo = [];
            let maxValue = { value: -Infinity };
            let maxAttribute = {};
            for (let flower = 0; flower < flowerSize; flower++) {
                for (let feather = 0; feather < featherSize; feather++) {
                    for (let sand = 0; sand < sandSize; sand++) {
                        for (let cup = 0; cup < cupSize; cup++) {
                            for (let head = 0; head < headSize; head++) {
                                let arts = [
                                    tempArts.flower[flower],
                                    tempArts.feather[feather],
                                    tempArts.sand[sand],
                                    tempArts.cup[cup],
                                    tempArts.head[head],
                                ].filter(item => item !== null);

                                // window.console.log(arts);

                                // let attribute = Object.assign({}, storeState.attribute);
                                // applyArtifacts(attribute, arts);
                                // window.console.log(attribute);
                                let attribute = compose(
                                    this.selectedCharacterAttribute, this.selectedWeaponAttribute, arts
                                );

                                let newValue = this.targetFunction(attribute);
                                // window.console.log(newValue.directives);
                                if (newValue.value > maxValue.value) {
                                    maxValue = newValue;
                                    maxCombo = arts;
                                    maxAttribute = attribute;
                                }
                            }
                        }
                    }
                }
            }

            maxCombo = maxCombo.map(item => item.refer);
            // window.console.log(maxCombo);
            return {
                combo: maxCombo,
                value: maxValue,
                attribute: maxAttribute,
            };
        },

        // 检查是否能进行运算
        check() {
            if (this.selectedTargetFunction === "") {
                return [false, "未指定目标函数"];
            }
            if (this.selectedWeapon === "") {
                return [false, "未指定武器"];
            }
            if (this.selectedCharacter === "") {
                return [false, "未指定角色"];
            }

            let flowerCount = this.$store.state.flower.filter(item => !item.omit).length;
            let featherCount = this.$store.state.feather.filter(item => !item.omit).length;
            let sandCount = this.$store.state.sand.filter(item => !item.omit).length;
            let cupCount = this.$store.state.cup.filter(item => !item.omit).length;
            let headCount = this.$store.state.head.filter(item => !item.omit).length;
            if (flowerCount + featherCount + sandCount + cupCount + headCount === 0) {
                return [false, "未指定圣遗物"];
            }

            flowerCount = Math.max(flowerCount, 1);
            featherCount = Math.max(featherCount, 1);
            sandCount = Math.max(sandCount, 1);
            cupCount = Math.max(cupCount, 1);
            headCount = Math.max(headCount, 1);
            let c = flowerCount * featherCount * sandCount * cupCount * headCount;
            if (c >= 5000000) {
                return [false, "圣遗物数量过多，请禁用明显更次的圣遗物"];
            }

            return [true];
        },

        doCalc() {
            let checkResult = this.check();
            if (!checkResult[0]) {
                this.$message.error(checkResult[1] || "错误");
                return;
            }

            this.isComputing = true;
            new Promise((resolve, reject) => {
                try {
                    let result = this.calc();
                    resolve(result);
                } catch(e) {
                    reject(e);
                }
            }).then(({combo, value, attribute}) => {
                // window.console.log(combo);
                this.showResultDialog = true;
                this.resultCombo = combo;
                this.resultValue = value.value;
                this.resultDeritive = value.deritives || [];
                this.resultAttribute = attribute;

                this.isCalculated = true;
            }).catch((e) => {
                this.$message.error("计算过程发生错误" + e);
            }).finally(() => {
                this.isComputing = false;
            });
        }
    },
    computed: {
        selectedCharacterAttribute() {
            if (this.customedCharacters[this.selectedCharacter]) {
                return this.customedCharacters[this.selectedCharacter];
            } else {
                return getCharacterAttribute(this.selectedCharacter);
            }
        },
        selectedWeaponAttribute() {
            if (this.customedWeapons[this.selectedWeapon]) {
                return this.customedWeapons[this.selectedWeapon];
            } else {
                return getWeaponAttribute(this.selectedWeapon);
            }
        },
        targetFunction() {
            if (this.customedTargets[this.selectedTarget]) {
                return this.customedTargets[this.selectedTarget].calc;
            } else {
                return getTargetFunction(this.selectedTarget);
            }
        },
        ...mapState([
            // "selectedCharacter",
            // "selectedWeapon",
            // "selectedWeaponAttribute",
            // "selectedCharacterAttribute",
            // "customWeapon",
            // "customCharacter",
            "customedCharacters",
            "customedWeapons",
            "customedTargets",

            // "selectedTargetFunction",
        ])
    }
}
</script>