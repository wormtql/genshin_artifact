<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-淬炼之剑</el-radio-button>
            <el-radio-button label="e">逆焰之刃</el-radio-button>
            <el-radio-button label="q">黎明</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <el-switch
                active-text="被Q技能附魔"
                v-model="config.afterQ"
                class="mb-16"
            ></el-switch>
            <div v-if="config.afterQ">
                <common-table-fire
                    :data="dilukeA.a"
                    class="mb-16"
                ></common-table-fire>
                <common-table-fire
                    :data="dilukeA.b"
                    class="mb-16"
                ></common-table-fire>
                <common-table-fire
                    :data="dilukeA.air"
                ></common-table-fire>
            </div>
            <div v-else>
                <common-table-physical
                    :data="dilukeA.a"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="dilukeA.b"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="dilukeA.air"
                ></common-table-physical>
            </div>
        </div>

        <common-table-fire
            v-show="showSkill === 'e'"
            :data="dilukeE"
        ></common-table-fire>

        <common-table-fire
            v-show="showSkill === 'q'"
            :data="dilukeQ"
        ></common-table-fire>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import dilukeA from "./diluke_a";
import dilukeE from "./diluke_e";
import dilukeQ from "./diluke_q";
import CommonTableFire from '../../../CommonTableFire.vue';
import CommonTablePhysical from '../../../CommonTablePhysical.vue';


export default {
    name: "Diluke.calculator",
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
            config: {
                afterQ: false,
            }
        }
    },
    computed: {
        dilukeA() {
            return dilukeA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        dilukeE() {
            return dilukeE(this.artifacts, this.configObject, this.enemy);
        },

        dilukeQ() {
            return dilukeQ(this.artifacts, this.configObject, this.enemy);
        }
    }
}
</script>