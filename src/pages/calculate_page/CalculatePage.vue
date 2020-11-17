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
                    <el-radio-button label="artifact">圣遗物</el-radio-button>
                </el-radio-group>

                <select-character v-model="selectedCharacter" v-show="current === 'character'"></select-character>

                <select-weapon v-model="selectedWeapon" v-show="current === 'weapon'"></select-weapon>

                <select-target v-model="selectedTarget" v-show="current === 'target'"></select-target>

                <el-alert show-icon v-show="current === 'artifact'" :closable="false"
                    style="margin-bottom: 16px"
                >
                    <template #title>
                        此处选择的圣遗物将会固定<br>
                        例如，“生之花”选择了某一个圣遗物，则“生之花”这个位置固定使用这个圣遗物
                    </template>
                </el-alert>
                <select-artifact v-model="selectedArtifact" v-show="current === 'artifact'"></select-artifact>

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
                    :loading.sync="isComputing"
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
import SelectArtifact from "@/components/select_artifact/SelectArtifact";
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
        SelectArtifact,
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
            selectedArtifact: [-1, -1, -1, -1, -1],

            current: "character",
        }
    },
    methods: {
        // 计算最优解
        calc() {
            // 最终要参与计算的圣遗物列表
            let tempArts = {
                "flower": [],
                "feather": [],
                "cup": [],
                "head": [],
                "sand": []
            };

            if (this.selectedArtifact[0] !== -1) {
                // 固定该处圣遗物
                tempArts.flower.push(convertArtifact(this.flower[this.selectedArtifact[0]]));
            } else {
                tempArts.flower = this.flower.filter(item => !item.omit).map(item => convertArtifact(item));
            }

            if (this.selectedArtifact[1] !== -1) {
                // 固定该处圣遗物
                tempArts.feather.push(convertArtifact(this.feather[this.selectedArtifact[1]]));
            } else {
                tempArts.feather = this.feather.filter(item => !item.omit).map(item => convertArtifact(item));
            }

            if (this.selectedArtifact[2] !== -1) {
                // 固定该处圣遗物
                tempArts.sand.push(convertArtifact(this.sand[this.selectedArtifact[2]]));
            } else {
                tempArts.sand = this.sand.filter(item => !item.omit).map(item => convertArtifact(item));
            }

            if (this.selectedArtifact[3] !== -1) {
                // 固定该处圣遗物
                tempArts.cup.push(convertArtifact(this.cup[this.selectedArtifact[3]]));
            } else {
                tempArts.cup = this.cup.filter(item => !item.omit).map(item => convertArtifact(item));
            }

            if (this.selectedArtifact[4] !== -1) {
                // 固定该处圣遗物
                tempArts.head.push(convertArtifact(this.head[this.selectedArtifact[4]]));
            } else {
                tempArts.head = this.head.filter(item => !item.omit).map(item => convertArtifact(item));
            }

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

            let flowerCount = this.flower.filter(item => !item.omit).length;
            if (this.selectedArtifact[0] !== -1) {
                flowerCount = 1;
            }

            let featherCount = this.feather.filter(item => !item.omit).length;
            if (this.selectedArtifact[1] !== -1) {
                featherCount = 1;
            }

            let sandCount = this.sand.filter(item => !item.omit).length;
            if (this.selectedArtifact[2] !== -1) {
                sandCount = 1;
            }

            let cupCount = this.cup.filter(item => !item.omit).length;
            if (this.selectedArtifact[3] !== -1) {
                cupCount = 1;
            }

            let headCount = this.head.filter(item => !item.omit).length;
            if (this.selectedArtifact[4] !== -1) {
                headCount = 1;
            }

            if (flowerCount + featherCount + sandCount + cupCount + headCount === 0) {
                return [false, "未指定圣遗物"];
            }

            flowerCount = Math.max(flowerCount, 1);
            featherCount = Math.max(featherCount, 1);
            sandCount = Math.max(sandCount, 1);
            cupCount = Math.max(cupCount, 1);
            headCount = Math.max(headCount, 1);
            let c = flowerCount * featherCount * sandCount * cupCount * headCount;
            if (c >= 1000000) {
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
            // window.console.log("aaa");
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
            "cup",
            "flower",
            "feather",
            "sand",
            "head",

            "customedCharacters",
            "customedWeapons",
            "customedTargets",

            // "selectedTargetFunction",
        ])
    }
}
</script>