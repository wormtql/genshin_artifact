<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>数值计算器</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row :gutter="16">
            <el-col :span="17">
                <h3>装备</h3>
                <el-radio-group v-model="current" style="margin-bottom: 16px">
                    <el-radio-button label="character">角色</el-radio-button>
                    <el-radio-button label="weapon">武器</el-radio-button>
                    <el-radio-button label="target">目标函数</el-radio-button>
                    <el-radio-button label="artifact">圣遗物</el-radio-button>
                </el-radio-group>

                <select-character v-model="selectedCharacter" v-show="current === 'character'"></select-character>

                <select-weapon v-model="selectedWeapon" v-show="current === 'weapon'"></select-weapon>

                <select-target v-model="selectedTarget" v-show="current === 'target'"></select-target>

                <select-artifact v-model="selectedArtifact" v-show="current === 'artifact'"></select-artifact>
            </el-col>

            <el-col :span="7">
                <h3>目标数值</h3>
                {{ finalTargetValue }}
                <h3>
                    面板
                </h3>
                <attribute-panel :panel="finalAttribute">
                </attribute-panel>
            </el-col>
        </el-row>
        
    </div>
</template>

<script>
import AttributePanel from "@/components/AttributePanel";
import SelectCharacter from "@/components/SelectCharacter";
import SelectWeapon from "@/components/SelectWeapon";
import SelectTarget from "@/components/SelectTarget";
import SelectArtifact from "@/components/select_artifact/SelectArtifact";
// import PreviewItem from "@/components/PreviewItem";

import { plans as plansPreset, getTargetFunction } from "@/common/target";
import { compose, getCharacterAttribute, getWeaponAttribute } from "genshin_panel";
// import { convertArtifact } from "@/utils/common";

import { mapState } from "vuex";
import { characterPreset, weaponPreset } from "@/common/basic";
import { convertArtifact } from "@/utils/common";

export default {
    name: "PanelCalculatePage",
    components: {
        AttributePanel,
        SelectCharacter,
        SelectWeapon,
        SelectTarget,
        SelectArtifact,
        // PreviewItem,
    },
    data: function() {
        return {
            plansPreset,

            characterPreset, weaponPreset,

            selectedCharacter: "keqing-70-0",
            selectedWeapon: "heijian-70-0",
            selectedTarget: "keqing-normal",
            selectedArtifact: [-1, -1, -1, -1, -1],

            current: "character",
        }
    },
    methods: {

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
        finalArtifacts() {
            let flower = this.flower[this.selectedArtifact[0]] || null;
            let feather = this.feather[this.selectedArtifact[1]] || null;
            let sand = this.sand[this.selectedArtifact[2]] || null;
            let cup = this.cup[this.selectedArtifact[3]] || null;
            let head = this.head[this.selectedArtifact[4]] || null;

            let arts = [flower, feather, sand, cup, head]
                .filter(item => item !== null)
                .map(item => convertArtifact(item));
            
            return arts;
        },
        finalAttribute() {
            return compose(this.selectedCharacterAttribute, this.selectedWeaponAttribute, this.finalArtifacts);
        },
        finalTargetValue() {
            return this.targetFunction(this.finalAttribute);
        },
        ...mapState([
            "flower",
            "feather",
            "sand",
            "cup",
            "head",

            "customedCharacters",
            "customedWeapons",
            "customedTargets",

            // "selectedTargetFunction",
        ])
    }
}
</script>