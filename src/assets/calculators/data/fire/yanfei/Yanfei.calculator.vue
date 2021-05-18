<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-火漆制印</el-radio-button>
            <el-radio-button label="e">丹书立约</el-radio-button>
            <el-radio-button label="q">凭此结契</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <el-switch
                v-model="config.burn"
                active-text="灼灼效果"
                class="mb-16"
            ></el-switch>

            <common-table-fire
                :data="yanfeiA.a"
                class="mb-16"
            ></common-table-fire>
            <common-table-fire
                :data="yanfeiA.b"
                class="mb-16"
            ></common-table-fire>
            <common-table-fire
                :data="yanfeiA.air"
                class="mb-16"
            ></common-table-fire>
        </div>

        <common-table-fire
            v-show="showSkill === 'e'"
            :data="yanfeiE"
        ></common-table-fire>

        <div v-show="showSkill === 'q'">
            <common-table-fire
                :data="yanfeiQ.q"
            ></common-table-fire>
            <p class="single-item">重击伤害提高：{{ yanfeiQ.bBonus * 100 }}%</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import yanfeiA from "./yanfei_a";
import yanfeiE from "./yanfei_e";
import yanfeiQ from "./yanfei_q";

import CommonTableFire from "../../../CommonTableFire";

export default {
    name: "Yanfei.calculator",
    components: {
        CommonTableFire,
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
                burn: false,
            }
        }
    },
    computed: {
        yanfeiA() {
            return yanfeiA(this.artifacts, this.configObject, this.enemy, this.config);
        },
        yanfeiE() {
            return yanfeiE(this.artifacts, this.configObject, this.enemy);
        },
        yanfeiQ() {
            return yanfeiQ(this.artifacts, this.configObject, this.enemy);
        },
    }
}
</script>