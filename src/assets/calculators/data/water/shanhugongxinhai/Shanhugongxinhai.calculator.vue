<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·水有常形</el-radio-button>
            <el-radio-button label="e">海月之誓</el-radio-button>
            <el-radio-button label="q">海人化羽</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <div class="buff-config-item">
                <h3 class="buff-config-title">是否处于“仪来羽衣”状态</h3>
                <el-switch
                    active-text="是"
                    v-model="configA.afterQ"
                ></el-switch>
            </div>

            <common-table-water
                :data="shanhugongxinhaiA.a"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="shanhugongxinhaiA.b"
                class="mb-16"
            ></common-table-water>
            <common-table-water
                :data="shanhugongxinhaiA.air"
            ></common-table-water>
        </div>

        <div v-show="showSkill === 'e'">
            <div class="buff-config-item">
                <h3 class="buff-config-title">是否处于“仪来羽衣”状态</h3>
                <el-switch
                    active-text="是"
                    v-model="configE.afterQ"
                ></el-switch>
            </div>

            <common-table-water
                :data="shanhugongxinhaiE.e"
            ></common-table-water>
            <p class="single-item">治疗量：{{ shanhugongxinhaiE.heal.toFixed(2) }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-water
                :data="shanhugongxinhaiQ.q"
            ></common-table-water>
            <p class="single-item">命中治疗量：{{ shanhugongxinhaiQ.heal }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import shanhugongxinhaiA from "./shanhugongxinhai_a";
import shanhugongxinhaiE from "./shanhugongxinhai_e";
import shanhugongxinhaiQ from "./shanhugongxinhai_q";

import CommonTableWater from "../../../CommonTableWater";

export default {
    name: "Shanhugongxinhai.calculator",
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
            configA: {
                afterQ: false,
            },
            configE: {
                afterQ: false,
            }
        }
    },
    computed: {
        shanhugongxinhaiA() {
            return shanhugongxinhaiA(this.artifacts, this.configObject, this.enemy, this.configA);
        },

        shanhugongxinhaiE() {
            return shanhugongxinhaiE(this.artifacts, this.configObject, this.enemy, this.configE);
        },

        shanhugongxinhaiQ() {
            return shanhugongxinhaiQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>