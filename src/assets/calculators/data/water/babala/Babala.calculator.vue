<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-水之浅唱</el-radio-button>
            <el-radio-button label="e">演唱，开始♪</el-radio-button>
            <el-radio-button label="q">闪耀奇迹♪</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-water
                :data="babalaA.a"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="babalaA.b"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="babalaA.air"
            ></common-table-water>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-water
                :data="babalaE.e"
            ></common-table-water>
            <p class="single-item">命中治疗量：{{ babalaE.cure1 }}</p>
            <p class="single-item">持续治疗量：{{ babalaE.cure2 }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <p class="single-item">治疗量：{{ babalaQ.cure }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import babalaA from "./babala_a";
import babalaE from "./babala_e";
import babalaQ from "./babala_q";

import CommonTableWater from "../../../CommonTableWater";

export default {
    name: "Babala.calculator",
    components: {
        CommonTableWater,
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
        babalaA() {
            return babalaA(this.artifacts, this.configObject, this.enemy);
        },

        babalaE() {
            return babalaE(this.artifacts, this.configObject, this.enemy);
        },

        babalaQ() {
            return babalaQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>