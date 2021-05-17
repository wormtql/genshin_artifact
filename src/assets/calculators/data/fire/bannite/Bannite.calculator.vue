<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-好运剑</el-radio-button>
            <el-radio-button label="e">热情过载</el-radio-button>
            <el-radio-button label="q">美妙旅程</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="banniteA"
            ></common-table-physical>
        </div>

        <common-table-fire
            v-show="showSkill === 'e'"
            :data="banniteE"
        ></common-table-fire>

        <div v-show="showSkill === 'q'">
            <common-table-fire
                :data="banniteQ.damage"
            ></common-table-fire>

            <p class="single-item">持续治疗：{{ banniteQ.cure }}</p>
            <p class="single-item">攻击力加成：{{ banniteQ.atkBonus }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import { normalA as banniteA } from "./bannite_a";
import banniteE from "./bannite_e";
import banniteQ from "./bannite_q";

// import DamageDisplay from "@c/display/DamageDisplay";
import CommonTableFire from "../../../CommonTableFire";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Bannite.calculator",
    components: {
        // DamageDisplay,
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
        banniteA() {
            return banniteA(this.artifacts, this.configObject, this.enemy);
        },

        banniteE() {
            return banniteE(this.artifacts, this.configObject, this.enemy);
        },

        banniteQ() {
            return banniteQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>