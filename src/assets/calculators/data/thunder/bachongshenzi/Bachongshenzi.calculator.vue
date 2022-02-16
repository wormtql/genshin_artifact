<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击·狐灵食罪式</el-radio-button>
            <el-radio-button label="e">野干役咒·杀生樱</el-radio-button>
            <el-radio-button label="q">大密法·天狐显真</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <div>
                <common-table-thunder
                    :data="bachongshenziA.a"
                    class="mb-16"
                ></common-table-thunder>
                <common-table-thunder
                    :data="bachongshenziA.b"
                    class="mb-16"
                ></common-table-thunder>
                <common-table-thunder
                    :data="bachongshenziA.air"
                ></common-table-thunder>
            </div>
        </div>

        <div v-show="showSkill === 'e'">
            <common-table-thunder
                :data="bachongshenziE.e"
            ></common-table-thunder>
        </div>

        <div v-show="showSkill === 'q'">
            <common-table-thunder
                :data="bachongshenziQ.q"
            ></common-table-thunder>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import bachongshenziA from "./bachongshenzi_a";
import bachongshenziE from "./bachongshenzi_e";
import bachongshenziQ from "./bachongshenzi_q";

// import CommonTablePhysical from "../../../CommonTablePhysical";
// import CommonTableRock from "../../../CommonTableRock";
import CommonTableThunder from "@asset/calculators/CommonTableThunder";

export default {
    name: "Bachongshenzi.calculator",
    components: {
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
            config: {
                afterQ: false,
            }
        }
    },
    computed: {
        bachongshenziA() {
            return bachongshenziA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        bachongshenziE() {
            return bachongshenziE(this.artifacts, this.configObject, this.enemy, this.config);
        },

        bachongshenziQ() {
            return bachongshenziQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>