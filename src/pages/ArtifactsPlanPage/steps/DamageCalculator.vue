<template>
    <div>
        <el-row :gutter="16">
            <el-col :span="18">
                <select-artifacts
                    :selected="selectedArtifactIds"
                ></select-artifacts>

                <component
                    :is="calculator"
                    :config-object="configObject"
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

export default {
    name: "DamageCalculator",
    components: {
        SelectArtifacts,
        AttributePanel,
    },
    inject: ["getConfigObject"],
    props: ["characterName"],
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
        }
    },
    methods: {
        // called when this component is displayed
        updateConfigObject() {
            console.log("on update");
            this.configObject = this.getConfigObject();
        }
    },
    computed: {
        calculator() {
            let temp = calculators[this.characterName];
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
            
            return getAttribute(this.selectedArtifactObjects, c, w, buffs);
        }
    }
}
</script>