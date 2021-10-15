<template>
    <div>
        <el-radio-group
            v-model="showSkill"
            size="small"
            style="margin: 16px 0"
        >
            <el-radio-button label="a">普通攻击-往生秘传枪法</el-radio-button>
            <el-radio-button label="e">蝶引来生</el-radio-button>
            <el-radio-button label="q">安神秘法</el-radio-button>
        </el-radio-group>

        <div v-show="showSkill === 'a'">
            <el-switch
                v-model="config.afterE"
                active-text="彼岸蝶舞"
                class="mb-16"
            ></el-switch>

            <div v-if="config.afterE">
                <common-table-fire
                    :data="hutaoA.a"
                    class="mb-16"
                ></common-table-fire>
                <common-table-fire
                    :data="hutaoA.b"
                    class="mb-16"
                ></common-table-fire>
                <common-table-fire
                    :data="hutaoA.air"
                ></common-table-fire>
            </div>
            <div v-else>
                <common-table-physical
                    :data="hutaoA.a"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="hutaoA.b"
                    class="mb-16"
                ></common-table-physical>
                <common-table-physical
                    :data="hutaoA.air"
                ></common-table-physical>
            </div>
        </div>

        <div v-show="showSkill === 'e'">
            <el-switch
                v-model="config.afterE"
                active-text="彼岸蝶舞"
                class="mb-16"
            ></el-switch>
            <div v-if="config.afterE">
                <common-table-fire
                    :data="hutaoE.e"
                ></common-table-fire>
            </div>
            <div v-else>
                <common-table-fire
                    :data="hutaoE.e"
                ></common-table-fire>
            </div>
            <p class="single-item">增加攻击力：{{ hutaoE.atkBonus }}</p>
        </div>

        <div v-show="showSkill === 'q'">
            <el-switch
                v-model="config.afterE"
                active-text="彼岸蝶舞"
                class="mb-16"
            ></el-switch>
            <div v-if="config.afterE">
                <common-table-fire
                    :data="hutaoQ.q"
                ></common-table-fire>
            </div>
            <div v-else>
                <common-table-fire
                    :data="hutaoQ.q"
                ></common-table-fire>
            </div>
            <p class="single-item">治疗量：{{ hutaoQ.cure1 }}</p>
            <p class="single-item">低血量治疗量：{{ hutaoQ.cure2 }}</p>
        </div>
    </div>
</template>

<script>
import Enemy from "@asset/enemies/enemy";
import hutaoA from "./hutao_a";
import hutaoE from "./hutao_e";
import hutaoQ from "./hutao_q";
import CommonTableFire from '../../../CommonTableFire.vue';
import CommonTablePhysical from "../../../CommonTablePhysical";


export default {
    name: "Hutao.calculator",
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
                afterE: false,
            }
        }
    },
    computed: {
        hutaoA() {
            return hutaoA(this.artifacts, this.configObject, this.enemy, this.config);
        },

        hutaoE() {
            return hutaoE(this.artifacts, this.configObject, this.enemy, this.config);
        },

        hutaoQ() {
            return hutaoQ(this.artifacts, this.configObject, this.enemy, this.config);
        },
    }
}
</script>