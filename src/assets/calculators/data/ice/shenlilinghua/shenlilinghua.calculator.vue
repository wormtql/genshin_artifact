<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·神里流·倾</el-radio-button>
            <el-radio-button label="e">神里流·冰华</el-radio-button>
            <el-radio-button label="q">神里流·霜灭</el-radio-button>
        </el-radio-group>

        <div style="margin-bottom: 12px">
            <el-switch
                v-model="afterDash"
                active-text="神里流·霰步后"
            ></el-switch>
        </div>
        <div style="margin-bottom: 12px">
            <el-switch
                v-model="afterTalent1"
                active-text="天罪国罪镇词"
            ></el-switch>
        </div>
        <div style="margin-bottom: 12px">
            <el-switch
                v-model="afterTalent2"
                active-text="寒天宣命祝词"
            ></el-switch>
        </div>

        <div v-show="showSkill === 'a'">

            <common-table-physical
                v-if="!afterDash"
                :data="shenlilinghuaA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-ice
                v-else
                :data="shenlilinghuaA.a"
                class="mb-16"
            ></common-table-ice>
            <common-table-physical
                v-if="!afterDash"
                :data="shenlilinghuaA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-ice
                v-else
                :data="shenlilinghuaA.b"
                class="mb-16"
            ></common-table-ice>
            <common-table-physical
                v-if="!afterDash"
                :data="shenlilinghuaA.air"
            ></common-table-physical>
            <common-table-ice
                v-else
                :data="shenlilinghuaA.air"
            ></common-table-ice>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-ice
                :data="shenlilinghuaE.e"
            ></common-table-ice>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-ice
                :data="shenlilinghuaQ.q"
            ></common-table-ice>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import shenlilinghuaA from "./shenlilinghua_a";
import shenlilinghuaE from "./shenlilinghua_e";
import shenlilinghuaQ from "./shenlilinghua_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableIce from "../../../CommonTableIce";

export default {
    name: "Shenlilinghua.calculator",
    components: {
        CommonTablePhysical,
        CommonTableIce
    },
    props: {
        enemy: {
            type: Object,
            default: function () {
                return new Enemy("hilichurl", 80);
            }
        },
        configObject: {
            type: Object,
        },
        artifacts: {
            type: Object,
        }
    },
    data() {
        return {
            showSkill: "a",

            afterDash: true,
            afterTalent1: false,
            afterTalent2: false,
        }
    },
    computed: {
        hasTalent1() {
            const c = this.configObject.character;
            return c.hasTalent1;
        },

        hasTalent2() {
            return this.configObject.character.hasTalent2;
        },

        extraConfig() {
            return {
                afterDash: this.afterDash,
                afterTalent1: this.afterTalent1,
                afterTalent2: this.afterTalent2,
            }
        },

        shenlilinghuaA() {
            return shenlilinghuaA(this.artifacts, this.configObject, this.enemy, this.extraConfig);
        },

        shenlilinghuaE() {
            return shenlilinghuaE(this.artifacts, this.configObject, this.enemy, this.extraConfig);
        },

        shenlilinghuaQ() {
            return shenlilinghuaQ(this.artifacts, this.configObject, this.enemy, this.extraConfig);
        }
    }
}
</script>