<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-古华剑法</el-radio-button>
            <el-radio-button label="e">古华剑·画雨笼山</el-radio-button>
            <el-radio-button label="q">古华剑·裁雨留虹</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="xingqiuA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="xingqiuA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="xingqiuA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-water
                :data="xingqiuE.e"
                class="mb-16"
            ></common-table-water>
            <p class="single-item">伤害减免：{{ xingqiuE.res }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-water
                :data="xingqiuQ.q"
            ></common-table-water>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import xingqiuA from "./xingqiu_a";
import xingqiuE from "./xingqiu_e";
import xingqiuQ from "./xingqiu_q";

import CommonTableWater from "../../../CommonTableWater";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Xingqiu.calculator",
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
        xingqiuA() {
            return xingqiuA(this.artifacts, this.configObject, this.enemy);
        },

        xingqiuE() {
            return xingqiuE(this.artifacts, this.configObject, this.enemy);
        },

        xingqiuQ() {
            return xingqiuQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>