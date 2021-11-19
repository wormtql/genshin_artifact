<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·喧哗屋传说</el-radio-button>
            <el-radio-button label="e">魔杀绝技·赤牛发破！</el-radio-button>
            <el-radio-button label="q">最恶鬼王·一斗轰临！！</el-radio-button>
        </el-radio-group>

        <div>
            <el-switch
                v-model="config.afterQ"
                active-text="开启Q技能"
                style="margin-bottom: 16px"
            ></el-switch>
        </div>

        <div v-show="showSkill === 'a'">
            <div v-if="config.afterQ">
                <common-table-rock
                    :data="huanglongyidouA.a"
                    class="mb-16"
                ></common-table-rock>
                <common-table-rock
                    :data="huanglongyidouA.b"
                    class="mb-16"
                ></common-table-rock>
                <common-table-rock
                    :data="huanglongyidouA.air"
                ></common-table-rock>
            </div>
            <div v-else>
                <common-table-physical
                    :data="huanglongyidouA.a"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="huanglongyidouA.b"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="huanglongyidouA.air"
                ></common-table-physical>
            </div>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="huanglongyidouE.e"
            ></common-table-rock>
        </div>

        <div v-show="showSkill === 'q'">
            <p class="single-item">攻击力提高：{{ huanglongyidouQ.atkBonus }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import huanglongyidouA from "./huanglongyidou_a";
import huanglongyidouE from "./huanglongyidou_e";
import huanglongyidouQ from "./huanglongyidou_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Huanglongyidou.calculator",
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
        huanglongyidouA() {
            return huanglongyidouA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        huanglongyidouE() {
            return huanglongyidouE(this.artifacts, this.configObject, this.enemy, this.config);
        },

        huanglongyidouQ() {
            return huanglongyidouQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>