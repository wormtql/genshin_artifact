<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-炎舞</el-radio-button>
            <el-radio-button label="e">热情拂扫</el-radio-button>
            <el-radio-button label="q">叛逆刮弦</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-physical
                :data="xinyanA.a"
                style="margin-bottom: 16px"
            ></common-table-physical>
            <common-table-physical
                :data="xinyanA.b"
            ></common-table-physical>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-fire
                :data="xinyanE.dmg"
            ></common-table-fire>

            <p class="single-item">
                一级护盾：{{ xinyanE.shield1 }}
            </p>
            <p class="single-item">
                二级护盾：{{ xinyanE.shield2 }}
            </p>
            <p class="single-item">
                三级护盾：{{ xinyanE.shield3 }}
            </p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-physical
                :data="xinyanQ.physical"
                style="margin-bottom: 16px"
            ></common-table-physical>
            <common-table-fire
                :data="xinyanQ.fire"
            ></common-table-fire>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import xinyanA from "./xinyan_a";
import xinyanE from "./xinyan_e";
import xinyanQ from "./xinyan_q";

import CommonTableFire from "../../../CommonTableFire";
import CommonTablePhysical from "../../../CommonTablePhysical";

export default {
    name: "Xinyan.calculator",
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
        xinyanA() {
            return xinyanA(this.artifacts, this.configObject, this.enemy);
        },

        xinyanE() {
            return xinyanE(this.artifacts, this.configObject, this.enemy);
        },

        xinyanQ() {
            return xinyanQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>