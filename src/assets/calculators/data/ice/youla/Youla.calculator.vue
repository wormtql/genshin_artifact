<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·西风剑术·宗室</el-radio-button>
            <el-radio-button label="e">冰潮的涡旋</el-radio-button>
            <el-radio-button label="q">凝浪之光剑</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="youlaA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="youlaA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="youlaA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-ice
                :data="youlaE.e"
            ></common-table-ice>
        </div>

        <div v-show="showSkill === 'q'">
            <div style="margin-bottom: 16px">
                <span style="font-size: 12px;">光降之剑层数：</span>
                <el-input-number
                    v-model="config.count"
                    :min="0"
                    :max="70"
                    size="mini"
                ></el-input-number>
            </div>
            
            <common-table-ice
                :data="youlaQ.q"
                style="margin-bottom: 16px"
            ></common-table-ice>
            <common-table-physical
                :data="youlaQ.sword"
            ></common-table-physical>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import youlaA from "./youla_a";
import youlaE from "./youla_e";
import youlaQ from "./youla_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableIce from "../../../CommonTableIce";

export default {
    name: "Youla.calculator",
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
            config: {
                count: 3,
            }
        }
    },
    computed: {
        youlaA() {
            return youlaA(this.artifacts, this.configObject, this.enemy);
        },

        youlaE() {
            return youlaE(this.artifacts, this.configObject, this.enemy);
        },

        youlaQ() {
            return youlaQ(this.artifacts, this.configObject, this.enemy, this.config);
        }
    }
}
</script>