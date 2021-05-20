<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-罪灭之矢</el-radio-button>
            <el-radio-button label="e">夜巡影翼</el-radio-button>
            <el-radio-button label="q">至夜幻现</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="feixieerA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="feixieerA.b1"
                class="mb-16"
            ></common-table-physical>
            <common-table-thunder
                :data="feixieerA.b2"
                class="mb-16"
            ></common-table-thunder>
            <common-table-physical
                :data="feixieerA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="feixieerE"
            ></common-table-thunder>
        </div>

        <common-table-thunder
            v-show="showSkill === 'q'"
            :data="feixieerQ"
        ></common-table-thunder>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import feixieerA from "./feixieer_a";
import feixieerE from "./feixieer_e";
import feixieerQ from "./feixieer_q";

import CommonTableThunder from "../../../CommonTableThunder";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Feixieer.calculator",
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
        feixieerA() {
            return feixieerA(this.artifacts, this.configObject, this.enemy);
        },

        feixieerE() {
            return feixieerE(this.artifacts, this.configObject, this.enemy);
        },

        feixieerQ() {
            return feixieerQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>