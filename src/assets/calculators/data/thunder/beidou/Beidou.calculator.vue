<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-征涛</el-radio-button>
            <el-radio-button label="e">捉浪</el-radio-button>
            <el-radio-button label="q">斫雷</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="beidouA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="beidouA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="beidouA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="beidouE.e"
            ></common-table-thunder>
            <p class="single-item">护盾吸收量：{{ beidouE.shield }}</p>
        </div>

        <common-table-thunder
            v-show="showSkill === 'q'"
            :data="beidouQ"
        ></common-table-thunder>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import beidouA from "./beidou_a";
import beidouE from "./beidou_e";
import beidouQ from "./beidou_q";

import CommonTableThunder from "../../../CommonTableThunder";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Beidou.calculator",
    components: {
        CommonTableThunder,
        CommonTablePhysical
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
        beidouA() {
            return beidouA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        beidouE() {
            return beidouE(this.artifacts, this.configObject, this.enemy);
        },

        beidouQ() {
            return beidouQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>