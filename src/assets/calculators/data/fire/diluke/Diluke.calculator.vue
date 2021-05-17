<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-淬炼之剑</el-radio-button>
            <el-radio-button label="e">逆焰之刃</el-radio-button>
            <el-radio-button label="q">黎明</el-radio-button>
        </el-radio-group>

        <el-table
            :data="dilukeA"
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
                label="被Q附魔"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.fire"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="被Q附魔（融化）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.fireMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="被Q附魔（蒸发）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.fireVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="dilukeE"
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
                label="伤害（融化）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="伤害（蒸发）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="dilukeQ"
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
                label="伤害（融化）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.normalMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="伤害（蒸发）"
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
import dilukeA from "./diluke_a";
import dilukeE from "./diluke_e";
import dilukeQ from "./diluke_q";

import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "Diluke.calculator",
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
        dilukeA() {
            return dilukeA(this.artifacts, this.configObject, this.enemy);
        },

        dilukeE() {
            return dilukeE(this.artifacts, this.configObject, this.enemy);
        },

        dilukeQ() {
            return dilukeQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>