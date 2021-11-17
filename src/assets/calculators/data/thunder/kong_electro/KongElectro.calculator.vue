<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·异邦惊雷</el-radio-button>
            <el-radio-button label="e">雷影剑</el-radio-button>
            <el-radio-button label="q">雷轰电转</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="kongElectroA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="kongElectroA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="kongElectroA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="kongElectroE"
            ></common-table-thunder>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-thunder
                :data="kongElectroQ.q"
            ></common-table-thunder>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import kongElectroA from "./kong_electro_a";
import kongElectroE from "./kong_electro_e";
import kongElectroQ from "./kong_electro_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableThunder from "../../../CommonTableThunder";

export default {
    name: "Me_electro.calculator",
    components: {
        CommonTablePhysical,
        CommonTableThunder,
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
        kongElectroA() {
            return kongElectroA(this.artifacts, this.configObject, this.enemy);
        },

        kongElectroE() {
            return kongElectroE(this.artifacts, this.configObject, this.enemy);
        },

        kongElectroQ() {
            return kongElectroQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>