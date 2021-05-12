<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-往生秘传枪法</el-radio-button>
            <el-radio-button label="e">蝶引来生</el-radio-button>
            <el-radio-button label="q">安神秘法</el-radio-button>
        </el-radio-group>

        <el-table
            :data="hutaoA"
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
                label="开E后"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.afterE"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="开E后（蒸发）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.afterEVaporize"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="开E后（融化）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.afterEMelt"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <el-table
            :data="hutaoE"
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
                    <damage-display :damage="scope.row.xmx"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="伤害（融化）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.xmxMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="伤害（蒸发）"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.xmxVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
        <div v-show="showSkill === 'q'">
            <el-table
                :data="hutaoQ"
                size="small"
                stripe
                style="margin-bottom: 16px"
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
                :data="hutaoQCure"
                size="small"
                stripe
            >
                <el-table-column
                    label="项目"
                    property="chs"
                    width="150"
                ></el-table-column>
                <el-table-column
                    label="治疗"
                    width="200"
                >
                    <template slot-scope="scope">
                        {{ scope.row.cure.toFixed(0) }}
                    </template>
                </el-table-column>
            </el-table>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import hutaoA from "./hutao_a";
import hutaoE from "./hutao_e";
import hutaoQ from "./hutao_q";
import hutaoQCure from "./hutao_q_cure";

import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "Hutao.calculator",
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
        hutaoA() {
            return hutaoA(this.artifacts, this.configObject, this.enemy);
        },

        hutaoE() {
            return hutaoE(this.artifacts, this.configObject, this.enemy);
        },

        hutaoQ() {
            return hutaoQ(this.artifacts, this.configObject, this.enemy);
        },

        hutaoQCure() {
            return hutaoQCure(this.artifacts, this.configObject);
        }
    }
}
</script>