<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-砰砰</el-radio-button>
            <el-radio-button label="e">蹦蹦炸弹</el-radio-button>
            <el-radio-button label="q">轰轰火花</el-radio-button>
        </el-radio-group>

        <el-table
            :data="keliA"
            v-show="showSkill === 'a'"
            size="small"
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
                label="融化"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="蒸发"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="keliE"
            v-show="showSkill === 'e'"
            size="small"
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
                label="融化"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="蒸发"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="keliQ"
            v-show="showSkill === 'q'"
            size="small"
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
                label="融化"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="蒸发"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import keliA from "./keli_a";
import keliE from "./keli_e";
import keliQ from "./keli_q";

import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "Keli.calculator",
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
        keliA() {
            return keliA(this.artifacts, this.configObject, this.enemy);
        },

        keliE() {
            return keliE(this.artifacts, this.configObject, this.enemy);
        },

        keliQ() {
            return keliQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>