<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·天狗传弓术</el-radio-button>
            <el-radio-button label="e">鸦羽天狗霆雷召咒</el-radio-button>
            <el-radio-button label="q">煌煌千道镇式</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="jiutiaoshaluoA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="jiutiaoshaluoA.b1"
                class="mb-16"
            ></common-table-physical>
            <common-table-thunder
                :data="jiutiaoshaluoA.b2"
                class="mb-16"
            ></common-table-thunder>
            <common-table-physical
                :data="jiutiaoshaluoA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="jiutiaoshaluoE.e"
            ></common-table-thunder>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-thunder
                :data="jiutiaoshaluoQ.q"
            ></common-table-thunder>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import jiutiaoshaluoA from "./jiutiaoshaluo_a";
import jiutiaoshaluoE from "./jiutiaoshaluo_e";
import jiutiaoshaluoQ from "./jiutiaoshaluo_q";

import CommonTableThunder from "../../../CommonTableThunder";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Jiutiaoshaluo.calculator",
    components: {
        CommonTableThunder,
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
        }
    },
    computed: {
        jiutiaoshaluoA() {
            return jiutiaoshaluoA(this.artifacts, this.configObject, this.enemy);
        },

        jiutiaoshaluoE() {
            return jiutiaoshaluoE(this.artifacts, this.configObject, this.enemy);
        },

        jiutiaoshaluoQ() {
            return jiutiaoshaluoQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>