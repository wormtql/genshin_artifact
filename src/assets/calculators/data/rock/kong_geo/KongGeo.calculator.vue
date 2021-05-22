<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·异邦岩峰</el-radio-button>
            <el-radio-button label="e">星陨剑</el-radio-button>
            <el-radio-button label="q">岩潮叠嶂</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="kongGeoA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="kongGeoA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="kongGeoA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="kongGeoE"
            ></common-table-rock>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-rock
                :data="kongGeoQ.q"
            ></common-table-rock>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import kongGeoA from "./kong_geo_a";
import kongGeoE from "./kong_geo_e";
import kongGeoQ from "./kong_geo_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Me_rock.calculator",
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
        }
    },
    computed: {
        kongGeoA() {
            return kongGeoA(this.artifacts, this.configObject, this.enemy);
        },

        kongGeoE() {
            return kongGeoE(this.artifacts, this.configObject, this.enemy);
        },

        kongGeoQ() {
            return kongGeoQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>