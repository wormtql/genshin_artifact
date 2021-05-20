<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-因果点破</el-radio-button>
            <el-radio-button label="e">水中幻愿</el-radio-button>
            <el-radio-button label="q">星命定轨</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-water
                :data="monaA.a"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="monaA.b"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="monaA.air"
            ></common-table-water>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-water
                :data="monaE"
            ></common-table-water>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-water
                :data="monaQ.q"
            ></common-table-water>
            <p class="single-item">伤害加成：{{ monaQ.bonus }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import monaA from "./mona_a";
import monaE from "./mona_e";
import monaQ from "./mona_q";

import CommonTableWater from "../../../CommonTableWater";

export default {
    name: "Mona.calculator",
    components: {
        CommonTableWater,
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
        monaA() {
            return monaA(this.artifacts, this.configObject, this.enemy);
        },

        monaE() {
            return monaE(this.artifacts, this.configObject, this.enemy);
        },

        monaQ() {
            return monaQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>