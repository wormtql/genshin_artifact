<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·岩雨</el-radio-button>
            <el-radio-button label="e">地心</el-radio-button>
            <el-radio-button label="q">天星</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="zhongliA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="zhongliA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="zhongliA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="zhongliE.e"
            ></common-table-rock>
            <p class="single-item">护盾吸收量：{{ zhongliE.shield }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-rock
                :data="zhongliQ.q"
            ></common-table-rock>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import zhongliA from "./zhongli_a";
import zhongliE from "./zhongli_e";
import zhongliQ from "./zhongli_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Zhongli.calculator",
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
        zhongliA() {
            return zhongliA(this.artifacts, this.configObject, this.enemy);
        },

        zhongliE() {
            return zhongliE(this.artifacts, this.configObject, this.enemy);
        },

        zhongliQ() {
            return zhongliQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>