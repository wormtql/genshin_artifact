<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-钢脊</el-radio-button>
            <el-radio-button label="e">利爪与苍雷</el-radio-button>
            <el-radio-button label="q">雷牙</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="leizeA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="leizeA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="leizeA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="leizeE"
            ></common-table-thunder>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-thunder
                :data="leizeQ.q"
                class="mb-16"
            ></common-table-thunder>
            <common-table-thunder
                :data="leizeQ.wolf"
            ></common-table-thunder>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import leizeA from "./leize_a";
import leizeE from "./leize_e";
import leizeQ from "./leize_q";

import CommonTableThunder from "../../../CommonTableThunder";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Leize.calculator",
    components: {
        CommonTableThunder,
        CommonTablePhysical,
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
        leizeA() {
            return leizeA(this.artifacts, this.configObject, this.enemy);
        },

        leizeE() {
            return leizeE(this.artifacts, this.configObject, this.enemy);
        },

        leizeQ() {
            return leizeQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>