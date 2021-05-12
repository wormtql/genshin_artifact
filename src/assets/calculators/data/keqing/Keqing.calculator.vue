<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-云来剑法</el-radio-button>
            <el-radio-button label="e">星斗归位</el-radio-button>
            <el-radio-button label="q">天街巡游</el-radio-button>
        </el-radio-group>

        <el-table
            :data="keqingA"
            size="small"
            v-show="showSkill === 'a'"
            stripe
        >
            <el-table-column
                label="技能"
                property="chs"
                width="150"
            ></el-table-column>
            <el-table-column
                label="伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normal"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="伤害（被E附魔）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.thunder"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="keqingE"
            size="small"
            v-show="showSkill === 'e'"
            stripe
        >
            <el-table-column
                label="技能"
                property="chs"
                width="150"
            ></el-table-column>
            <el-table-column
                label="伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normal"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="keqingQ"
            size="small"
            v-show="showSkill === 'q'"
            stripe
        >
            <el-table-column
                label="技能"
                property="chs"
                width="150"
            ></el-table-column>
            <el-table-column
                label="伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normal"></damage-display>
                </template>
            </el-table-column>
        </el-table>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import keqingA from "./keqing_a";
import keqingE from "./keqing_e";
import keqingQ from "./keqing_q";

import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "Keqing.calculator",
    components: {
        DamageDisplay,
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
        keqingA() {
            return keqingA(this.artifacts, this.configObject, this.enemy);
        },

        keqingE() {
            return keqingE(this.artifacts, this.configObject, this.enemy);
        },

        keqingQ() {
            return keqingQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>