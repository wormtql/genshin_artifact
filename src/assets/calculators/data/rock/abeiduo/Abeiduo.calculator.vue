<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·西风剑术·白</el-radio-button>
            <el-radio-button label="e">创生法·拟造阳华</el-radio-button>
            <el-radio-button label="q">诞生式·大地之潮</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="abeiduoA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="abeiduoA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="abeiduoA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="abeiduoE.e"
            ></common-table-rock>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-rock
                :data="abeiduoQ.q"
            ></common-table-rock>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import abeiduoA from "./abeiduo_a";
import abeiduoE from "./abeiduo_e";
import abeiduoQ from "./abeiduo_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Abeiduo.calculator",
    components: {
        CommonTablePhysical,
        CommonTableRock,
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
        abeiduoA() {
            return abeiduoA(this.artifacts, this.configObject, this.enemy);
        },

        abeiduoE() {
            return abeiduoE(this.artifacts, this.configObject, this.enemy);
        },

        abeiduoQ() {
            return abeiduoQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>