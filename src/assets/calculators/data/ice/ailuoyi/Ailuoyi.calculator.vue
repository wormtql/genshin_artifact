<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·快速射击</el-radio-button>
            <el-radio-button label="e">冰尘雪野</el-radio-button>
            <el-radio-button label="q">曙光预言</el-radio-button>
        </el-radio-group>

        <el-alert title="暂未计算天赋效果，请手动添加" :closable="false" style="margin-bottom: 16px"></el-alert>

        <div v-show="showSkill === 'a'">
            <div class="buff-config-item">
                <h3 class="buff-config-title">冰驰</h3>
                <el-switch
                    active-text="是"
                    inactive-text="否"
                    v-model="config.iceRush"
                ></el-switch>
            </div>

            <div class="buff-config-item">
                <h3 class="buff-config-title">线圈层数</h3>
                <el-slider
                    :min="0"
                    :max="3"
                    :step="1"
                    show-input
                    input-size="small"
                    v-model="config.coilCount"
                ></el-slider>
            </div>

            <template v-if="!config.iceRush">
                <common-table-physical
                    :data="ailuoyiA.a"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="ailuoyiA.b1"
                    class="mb-16"
                ></common-table-physical>
                <common-table-ice
                    :data="ailuoyiA.b2"
                    class="mb-16"
                ></common-table-ice>
                <common-table-physical
                    :data="ailuoyiA.air"
                ></common-table-physical>
            </template>
            <template v-else>
                <common-table-ice
                    :data="ailuoyiA.a"
                    class="mb-16"
                ></common-table-ice>
                <common-table-ice
                    :data="ailuoyiA.b"
                    class="mb-16"
                ></common-table-ice>
                <common-table-ice
                    :data="ailuoyiA.air"
                ></common-table-ice>
            </template>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-ice
                :data="ailuoyiE.e"
            ></common-table-ice>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-ice
                :data="ailuoyiQ.q"
            ></common-table-ice>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import ailuoyiA from "./ailuoyi_a";
import ailuoyiE from "./ailuoyi_e";
import ailuoyiQ from "./ailuoyi_q";

import CommonTablePhysical from "../../../CommonTablePhysical";
import CommonTableIce from "../../../CommonTableIce";

export default {
    name: "Ailuoyi.calculator",
    components: {
        CommonTablePhysical,
        CommonTableIce
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
                coilCount: 0,
                iceRush: false,
            }
        }
    },
    computed: {
        ailuoyiA() {
            return ailuoyiA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        ailuoyiE() {
            return ailuoyiE(this.artifacts, this.configObject, this.enemy);
        },

        ailuoyiQ() {
            return ailuoyiQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>