<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-断雨</el-radio-button>
            <el-radio-button label="e">魔王武装·狂澜</el-radio-button>
            <el-radio-button label="q">极恶技·尽灭闪</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="dadaliyaA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="dadaliyaA.b1"
                class="mb-16"
            ></common-table-physical>
            <common-table-water
                :data="dadaliyaA.b2"
                class="mb-16"
            ></common-table-water>
            <common-table-physical
                :data="dadaliyaA.air"
                class="mb-16"
            ></common-table-physical>
            <common-table-water
                :data="dadaliyaA.duanliu"
            ></common-table-water>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-water
                :data="dadaliyaE.a"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="dadaliyaE.b"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="dadaliyaE.e"
            ></common-table-water>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-water
                :data="dadaliyaQ"
            ></common-table-water>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import dadaliyaA from "./dadaliya_a";
import dadaliyaE from "./dadaliya_e";
import dadaliyaQ from "./dadaliya_q";

import CommonTableWater from "../../../CommonTableWater";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Dadaliya.calculator",
    components: {
        CommonTableWater,
        CommonTablePhysical,
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
            config: {
                afterE: false,
            }
        }
    },
    computed: {
        dadaliyaA() {
            return dadaliyaA(this.artifacts, this.configObject, this.enemy);
        },

        dadaliyaE() {
            return dadaliyaE(this.artifacts, this.configObject, this.enemy);
        },

        dadaliyaQ() {
            return dadaliyaQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>