<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·异邦铁风</el-radio-button>
            <el-radio-button label="e">风涡剑</el-radio-button>
            <el-radio-button label="q">风息激荡</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="kongAnemoA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="kongAnemoA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="kongAnemoA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-wind
                :data="kongAnemoE"
            ></common-table-wind>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-wind
                :data="kongAnemoQ.q"
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

            <common-table-water :data="kongAnemoQ.water" v-show="element === 'water'"></common-table-water>
            <common-table-fire :data="kongAnemoQ.fire" v-show="element === 'fire'"></common-table-fire>
            <common-table-thunder :data="kongAnemoQ.thunder" v-show="element === 'thunder'"></common-table-thunder>
            <common-table-ice :data="kongAnemoQ.ice" v-show="element === 'ice'"></common-table-ice>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import kongAnemoA from "./kong_anemo_a";
import kongAnemoE from "./kong_anemo_e";
import kongAnemoQ from "./kong_anemo_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableWind from "../../../CommonTableWind";
import CommonTableWater from "../../../CommonTableWater";
import CommonTableThunder from "../../../CommonTableThunder";
import CommonTableIce from "../../../CommonTableIce";
import CommonTableFire from "../../../CommonTableFire";

export default {
    name: "Me_wind.calculator",
    components: {
        CommonTablePhysical,
        CommonTableWind,
        CommonTableWater,
        CommonTableThunder,
        CommonTableIce,
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
            element: "water",
        }
    },
    computed: {
        kongAnemoA() {
            return kongAnemoA(this.artifacts, this.configObject, this.enemy);
        },

        kongAnemoE() {
            return kongAnemoE(this.artifacts, this.configObject, this.enemy);
        },

        kongAnemoQ() {
            return kongAnemoQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>