<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·呲牙裂扇箭</el-radio-button>
            <el-radio-button label="e">犬坂吠吠方圆阵</el-radio-button>
            <el-radio-button label="q">兽牙逐突形胜战法</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="wulangA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="wulangA.b1"
                class="mb-16"
            ></common-table-physical>
            <common-table-rock
                :data="wulangA.b2"
                class="mb-16"
            ></common-table-rock>
            <common-table-physical
                :data="wulangA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="wulangE.e"
            ></common-table-rock>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-rock
                :data="wulangQ.q"
            ></common-table-rock>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import wulangA from "./wulang_a";
import wulangE from "./wulang_e";
import wulangQ from "./wulang_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Wulang.calculator",
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
        wulangA() {
            return wulangA(this.artifacts, this.configObject, this.enemy);
        },

        wulangE() {
            return wulangE(this.artifacts, this.configObject, this.enemy);
        },

        wulangQ() {
            return wulangQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>