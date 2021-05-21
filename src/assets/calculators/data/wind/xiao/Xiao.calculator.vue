<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-卷积微尘</el-radio-button>
            <el-radio-button label="e">风轮两立</el-radio-button>
            <el-radio-button label="q">靖妖傩舞</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="xiaoA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="xiaoA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="xiaoA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-wind
                :data="xiaoE"
            ></common-table-wind>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-wind
                :data="xiaoAAfterQ.a"
                class="mb-16"
            ></common-table-wind>
            <common-table-wind
                :data="xiaoAAfterQ.b"
                class="mb-16"
            ></common-table-wind>
            <common-table-wind
                :data="xiaoAAfterQ.air"
            ></common-table-wind>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import xiaoA from "./xiao_a";
import xiaoE from "./xiao_e";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableWind from "../../../CommonTableWind";

export default {
    name: "Xiao.calculator",
    components: {
        CommonTablePhysical,
        CommonTableWind,
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
        xiaoA() {
            return xiaoA(this.artifacts, this.configObject, this.enemy, { afterQ: false });
        },

        xiaoAAfterQ() {
            return xiaoA(this.artifacts, this.configObject, this.enemy, { afterQ: true });
        },

        xiaoE() {
            return xiaoE(this.artifacts, this.configObject, this.enemy);
        },
    }
}
</script>