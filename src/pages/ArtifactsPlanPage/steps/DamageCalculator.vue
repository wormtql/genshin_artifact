<template>
    <div v-if="configObject !== null">
        <div class="toolbar">
            <el-button
                size="small"
                @click="selectArtifactFromResult"
            >穿戴计算结果圣遗物</el-button>
        </div>

        <el-row :gutter="16">
            <el-col :span="18">
                <select-artifacts
                    :selected="selectedArtifactIds"
                ></select-artifacts>

                <div
                    class="legend"
                    style="padding: 16px 0"
                >
                    <damage-display :damage="{ crit: '暴击伤害', nonCrit: '非暴击伤害', expect: '期望伤害' }"></damage-display>
                </div>

                <component
                    :is="calculator"
                    :config-object="configObject"
                    :artifacts="selectedArtifactObjects"
                ></component>
            </el-col>
            <el-col :span="6">
                <attribute-panel v-if="configObject !== null" :panel="attributePanel"></attribute-panel>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import calculators from "@asset/calculators";
import { getAttribute } from "@util/attribute";

import SelectArtifacts from "@c/select/SelectArtifacts";
import AttributePanel from "@c/AttributePanel";
import DamageDisplay from "@c/display/DamageDisplay";


export default {
    name: "DamageCalculator",
    components: {
        SelectArtifacts,
        AttributePanel,
        DamageDisplay,
    },
    inject: ["getConfigObject", "getResultPage"],
    data() {
        return {
            configObject: null,

            selectedArtifactIds: {
                flower: null,
                feather: null,
                sand: null,
                cup: null,
                head: null,
            },

            artifactsFilter: () => true,
        }
    },
    methods: {
        // called when this component is displayed
        updateConfigObject() {
            // console.log("on update");
            this.configObject = this.getConfigObject();
        },

        selectArtifactFromResult() {
            let resultPage = this.getResultPage();
            let { error, data } = resultPage.getResultArtifacts();

            if (error) {
                this.$message({
                    type: "warning",
                    message: "请先进行一个计算",
                });
                return;
            }

            for (let art of data) {
                this.selectedArtifactIds[art.position] = art.id;
            }
            // console.log(data);
        }
    },
    computed: {
        calculator() {
            let cName = this.configObject.character.name;
            let temp = calculators[cName];
            temp = temp ?? calculators["keqing"];

            return temp;
        },

        selectedArtifactObjects() {
            let artifactsById = this.$store.getters["artifacts/artifactsById"];
            let temp = [];
            for (let id of Object.values(this.selectedArtifactIds)) {
                let art = artifactsById[id];
                if (art) {
                    temp.push(art);
                }
            }

            return temp;
        },

        attributePanel() {
            if (this.configObject === null) {
                return {};
            }

            let c = this.configObject.character;
            let w = this.configObject.weapon;
            let buffs = this.configObject.buffs;
            let artifactsConfig = this.configObject.artifactsConfig;
            
            return getAttribute(this.selectedArtifactObjects, c, w, buffs, artifactsConfig);
        }
    }
}
</script>

<style lang="scss" scoped>
.toolbar {
    margin-bottom: 16px;
}
</style>