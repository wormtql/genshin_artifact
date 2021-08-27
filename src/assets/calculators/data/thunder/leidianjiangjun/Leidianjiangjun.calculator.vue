<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·源流</el-radio-button>
            <el-radio-button label="e">神变·恶曜开眼</el-radio-button>
            <el-radio-button label="q">奥义·梦想真说</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="leidianjiangjunA.a"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="leidianjiangjunA.b"
                class="mb-16"
            ></common-table-physical>
            <common-table-physical
                :data="leidianjiangjunA.air"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="leidianjiangjunE"
            ></common-table-thunder>
        </div>

        <div v-show="showSkill === 'q'">
            <div class="buff-config-item">
                <h3 class="buff-config-title">是否在雷罚恶曜之眼状态</h3>
                <el-switch
                    active-text="是"
                    inactive-text="否"
                    v-model="afterE"
                ></el-switch>
            </div>
            <div class="buff-config-item">
                <h3 class="buff-config-title">愿力层数</h3>
                <el-slider
                    :min="0"
                    :max="60"
                    show-input
                    input-size="small"
                    :step="1"
                    v-model="qLevel"
                ></el-slider>
            </div>

            <common-table-thunder
                :data="leidianjiangjunQ.q"
            ></common-table-thunder>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import leidianjiangjunA from "./leidianjiangjun_a";
import leidianjiangjunE from "./leidianjiangjun_e";
import leidianjiangjunQ from "./leidianjiangjun_q";

import CommonTableThunder from "../../../CommonTableThunder";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Leidianjiangjun.calculator",
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
            
            afterE: false,
            qLevel: 0,
        }
    },
    computed: {
        leidianjiangjunA() {
            return leidianjiangjunA(this.artifacts, this.configObject, this.enemy);
        },

        leidianjiangjunE() {
            return leidianjiangjunE(this.artifacts, this.configObject, this.enemy);
        },

        leidianjiangjunQ() {
            return leidianjiangjunQ(this.artifacts, this.configObject, this.enemy, {
                afterE: this.afterE,
                qLevel: this.qLevel,
            });
        }
    }
}
</script>