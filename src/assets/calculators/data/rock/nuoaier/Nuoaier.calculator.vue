<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·西风剑术·女仆</el-radio-button>
            <el-radio-button label="e">护心铠</el-radio-button>
            <el-radio-button label="q">大扫除</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <el-switch
                v-model="config.afterQ"
                active-text="开启Q技能“大扫除”后"
                style="margin-bottom: 16px"
            ></el-switch>
            <div v-if="config.afterQ">
                <common-table-rock
                    :data="nuoaierA.a"
                    class="mb-16"
                ></common-table-rock>
                <common-table-rock
                    :data="nuoaierA.b"
                    class="mb-16"
                ></common-table-rock>
                <common-table-rock
                    :data="nuoaierA.air"
                ></common-table-rock>
            </div>
            <div v-else>
                <common-table-physical
                    :data="nuoaierA.a"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="nuoaierA.b"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="nuoaierA.air"
                ></common-table-physical>
            </div>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="nuoaierE.e"
            ></common-table-rock>
            <p class="single-item">吸收量：{{ nuoaierE.shield }}</p>
            <p class="single-item">治疗量：{{ nuoaierE.cure }}</p>
            <p class="single-item">治疗触发几率：{{ nuoaierE.prob * 100 }}%</p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-rock
                :data="nuoaierQ.q"
            ></common-table-rock>
            <p class="single-item">攻击力提高：{{ nuoaierQ.atkLift }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import nuoaierA from "./nuoaier_a";
import nuoaierE from "./nuoaier_e";
import nuoaierQ from "./nuoaier_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Nuoaier.calculator",
    components: {
        CommonTablePhysical,
        CommonTableRock,
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
                afterQ: false,
            }
        }
    },
    computed: {
        nuoaierA() {
            return nuoaierA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        nuoaierE() {
            return nuoaierE(this.artifacts, this.configObject, this.enemy);
        },

        nuoaierQ() {
            return nuoaierQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>