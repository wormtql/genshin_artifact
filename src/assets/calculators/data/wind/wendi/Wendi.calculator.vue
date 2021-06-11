<template>
    <div>
        <!-- <common-table-transformative

        ></common-table-transformative> -->

        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-神代射术</el-radio-button>
            <el-radio-button label="e">高天之歌</el-radio-button>
            <el-radio-button label="q">风神之诗</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="wendiA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="wendiA.b1"
                class="mb-16"
            ></common-table-physical>
            <common-table-wind
                :data="wendiA.b2"
                class="mb-16"
            ></common-table-wind>
            <common-table-physical
                :data="wendiA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-wind
                :data="wendiE"
            ></common-table-wind>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-wind
                :data="wendiQ.q"
            ></common-table-wind>
            <el-radio-group
                v-model="element"
                size="small"
                style="margin: 16px 0"
            >
                <el-radio-button label="water">水</el-radio-button>
                <el-radio-button label="fire">火</el-radio-button>
                <el-radio-button label="thunder">雷</el-radio-button>
                <el-radio-button label="ice">冰</el-radio-button>
            </el-radio-group>

            <common-table-water :data="wendiQ.water" v-show="element === 'water'"></common-table-water>
            <common-table-fire :data="wendiQ.fire" v-show="element === 'fire'"></common-table-fire>
            <common-table-thunder :data="wendiQ.thunder" v-show="element === 'thunder'"></common-table-thunder>
            <common-table-ice :data="wendiQ.ice" v-show="element === 'ice'"></common-table-ice>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import wendiA from "./wendi_a";
import wendiE from "./wendi_e";
import wendiQ from "./wendi_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableWind from "../../../CommonTableWind";
import CommonTableWater from "../../../CommonTableWater";
import CommonTableThunder from "../../../CommonTableThunder";
import CommonTableIce from "../../../CommonTableIce";
import CommonTableFire from "../../../CommonTableFire";
// import CommonTableTransformative from "../../../CommonTransformative";

export default {
    name: "Wendi.calculator",
    components: {
        CommonTablePhysical,
        CommonTableWind,
        CommonTableWater,
        CommonTableThunder,
        CommonTableIce,
        CommonTableFire,
        // CommonTableTransformative,
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
            element: "water",
        }
    },
    computed: {
        wendiA() {
            return wendiA(this.artifacts, this.configObject, this.enemy);
        },

        wendiE() {
            return wendiE(this.artifacts, this.configObject, this.enemy);
        },

        wendiQ() {
            return wendiQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>