<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-神射手</el-radio-button>
            <el-radio-button label="e">爆弹玩偶</el-radio-button>
            <el-radio-button label="q">箭雨</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="anboA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="anboA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-fire
                :data="anboA.b2"
                class="mb-16"
            ></common-table-fire>
            <common-table-physical
                :data="anboA.air"
            ></common-table-physical>
        </div>

        <common-table-fire
            v-show="showSkill === 'e'"
            :data="anboE"
        ></common-table-fire>

        <common-table-fire
            v-show="showSkill === 'q'"
            :data="anboQ"
        ></common-table-fire>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import anboA from "./anbo_a";
import anboE from "./anbo_e";
import anboQ from "./anbo_q";

// import DamageDisplay from "@c/display/DamageDisplay";
import CommonTableFire from "../../../CommonTableFire";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Anbo.calculator",
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
        anboA() {
            return anboA(this.artifacts, this.configObject, this.enemy);
        },

        anboE() {
            return anboE(this.artifacts, this.configObject, this.enemy);
        },

        anboQ() {
            return anboQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>