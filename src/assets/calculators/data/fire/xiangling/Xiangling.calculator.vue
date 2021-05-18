<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-白案功夫</el-radio-button>
            <el-radio-button label="e">锅巴出击</el-radio-button>
            <el-radio-button label="q">旋火轮</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="xianglingA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="xianglingA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="xianglingA.air"
            ></common-table-physical>
        </div>

        <common-table-fire
            v-show="showSkill === 'e'"
            :data="xianglingE"
        ></common-table-fire>

        <common-table-fire
            v-show="showSkill === 'q'"
            :data="xianglingQ"
        ></common-table-fire>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import xianglingA from "./xiangling_a";
import xianglingE from "./xiangling_e";
import xianglingQ from "./xiangling_q";

import CommonTableFire from "../../../CommonTableFire";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Xiangling.calculator",
    components: {
        CommonTableFire,
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
        }
    },
    computed: {
        xianglingA() {
            return xianglingA(this.artifacts, this.configObject, this.enemy);
        },

        xianglingE() {
            return xianglingE(this.artifacts, this.configObject, this.enemy);
        },

        xianglingQ() {
            return xianglingQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>