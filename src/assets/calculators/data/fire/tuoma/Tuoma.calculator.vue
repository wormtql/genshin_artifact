<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·迅破枪势</el-radio-button>
            <el-radio-button label="e">烈烧佑命之侍护</el-radio-button>
            <el-radio-button label="q">真红炽火之大铠</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="tuomaA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="tuomaA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="tuomaA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-fire
                :data="tuomaE.e"
            ></common-table-fire>
            <p class="single-item">护盾吸收量：{{ tuomaE.shield1 }}</p>
            <p class="single-item">护盾吸收量上限：{{ tuomaE.shield2 }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-fire
                :data="tuomaQ.q"
            ></common-table-fire>
            <p class="single-item">护盾吸收量：{{ tuomaQ.shield1 }}</p>
        </div>
        
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import tuomaA from "./tuoma_a";
import tuomaE from "./tuoma_e";
import tuomaQ from "./tuoma_q";

import CommonTableFire from "../../../CommonTableFire";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Tuoma.calculator",
    components: {
        CommonTableFire,
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
        tuomaA() {
            return tuomaA(this.artifacts, this.configObject, this.enemy);
        },

        tuomaE() {
            return tuomaE(this.artifacts, this.configObject, this.enemy);
        },

        tuomaQ() {
            return tuomaQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>