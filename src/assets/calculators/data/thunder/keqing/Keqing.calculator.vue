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

        <div v-show="showSkill === 'a'">
            <el-switch
                v-model="config.afterE"
                active-text="被E技能附魔"
                class="mb-16"
            ></el-switch>

            <div v-if="config.afterE">
                <common-table-thunder
                    :data="keqingA.a"
                    class="mb-16"
                ></common-table-thunder>
                <common-table-thunder
                    :data="keqingA.b"
                    class="mb-16"
                ></common-table-thunder>
                <common-table-thunder
                    :data="keqingA.air"
                ></common-table-thunder>
            </div>
            <div v-else>
                <common-table-physical
                    :data="keqingA.a"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="keqingA.b"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="keqingA.air"
                    class="mb-16"
                ></common-table-physical>
            </div>
        </div>

        <common-table-thunder
            v-show="showSkill === 'e'"
            :data="keqingE"
        ></common-table-thunder>

        <common-table-thunder
            v-show="showSkill === 'q'"
            :data="keqingQ"
        ></common-table-thunder>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import keqingA from "./keqing_a";
import keqingE from "./keqing_e";
import keqingQ from "./keqing_q";

import CommonTableThunder from "../../../CommonTableThunder";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Keqing.calculator",
    components: {
        CommonTableThunder,
        CommonTablePhysical
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
                afterE: false,
            }
        }
    },
    computed: {
        keqingA() {
            return keqingA(this.artifacts, this.configObject, this.enemy, this.config);
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