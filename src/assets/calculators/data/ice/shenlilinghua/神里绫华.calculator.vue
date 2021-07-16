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

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="shenlilinghuaA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="shenlilinghuaA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="shenlilinghuaA.air"
            ></common-table-physical>
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
        }
    },
    computed: {
        shenlilinghuaA() {
            return shenlilinghuaA(this.artifacts, this.configObject, this.enemy);
        },

        shenlilinghuaE() {
            return shenlilinghuaE(this.artifacts, this.configObject, this.enemy);
        },

        shenlilinghuaQ() {
            return shenlilinghuaQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>