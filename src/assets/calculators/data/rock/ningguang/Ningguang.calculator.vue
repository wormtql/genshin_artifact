<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·千金掷</el-radio-button>
            <el-radio-button label="e">璇玑屏</el-radio-button>
            <el-radio-button label="q">天权崩玉</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <common-table-rock
                :data="ningguangA.a"
                class="mb-16"
            ></common-table-rock>
            <common-table-rock
                :data="ningguangA.b"
                class="mb-16"
            ></common-table-rock>
            <common-table-rock
                :data="ningguangA.air"
            ></common-table-rock>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-rock
                :data="ningguangE.e"
            ></common-table-rock>
            <p class="single-item">继承生命：{{ ningguangE.hp }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-rock
                :data="ningguangQ.q"
            ></common-table-rock>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import ningguangA from "./ningguang_a";
import ningguangE from "./ningguang_e";
import ningguangQ from "./ningguang_q";

import CommonTableRock from "../../../CommonTableRock";

export default {
    name: "Ningguang.calculator",
    components: {
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
        ningguangA() {
            return ningguangA(this.artifacts, this.configObject, this.enemy);
        },

        ningguangE() {
            return ningguangE(this.artifacts, this.configObject, this.enemy);
        },

        ningguangQ() {
            return ningguangQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>