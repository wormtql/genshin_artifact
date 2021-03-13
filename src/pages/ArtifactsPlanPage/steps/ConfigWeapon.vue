<template>
    <div>
        <!-- 其他参数 -->
        <component :if="needExtraConfig" :is="w.config" ref="extraConfig">
        </component>

        <div v-if="star >= 3" class="config-item">
            <h3 class="config-title">精炼等级</h3>
            <el-input-number v-model="refine" :min="1" :max="5"></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">武器等级</h3>
            <div class="panel">
                <span
                    v-for="i in 19"
                    :key="i"
                    class="select-int"
                    @click="handleClickLevel(i)"
                >{{ i }}</span>
                <span class="select-int special" @click="handleClickLevel('20-')">20-</span>
            </div>
            
            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('20+')">20+</span>
                <span
                    v-for="i in 19"
                    :key="i + 20"
                    class="select-int"
                    @click="handleClickLevel(i + 20)"
                >{{ i + 20 }}</span>
                <span class="select-int special" @click="handleClickLevel('40-')">40-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('40+')">40+</span>
                <span
                    v-for="i in 9"
                    :key="i + 40"
                    class="select-int"
                    @click="handleClickLevel(i + 40)"
                >{{ i + 40 }}</span>
                <span class="select-int special" @click="handleClickLevel('50-')">50-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('50+')">50+</span>
                <span
                    v-for="i in 9"
                    :key="i + 50"
                    class="select-int"
                    @click="handleClickLevel(i + 50)"
                >{{ i + 50 }}</span>
                <span class="select-int special" @click="handleClickLevel('60-')">60-</span>
            </div>

            <div class="panel">
                <span class="select-int special" @click="handleClickLevel('60+')">60+</span>
                <span
                    v-for="i in 9"
                    :key="i + 60"
                    class="select-int"
                    @click="handleClickLevel(i + 60)"
                >{{ i + 60 }}</span>
                <span class="select-int special" @click="handleClickLevel('70-')" v-if="star >= 3">70-</span>
                <span class="select-int" @click="handleClickLevel('70')" v-else>70</span>
            </div>

            <template v-if="star >= 3">
                <div class="panel">
                    <span class="select-int special" @click="handleClickLevel('70+')">70+</span>
                    <span
                        v-for="i in 9"
                        :key="i + 70"
                        class="select-int"
                        @click="handleClickLevel(i + 70)"
                    >{{ i + 70 }}</span>
                    <span class="select-int special" @click="handleClickLevel('80-')">80-</span>
                </div>

                <div class="panel">
                    <span class="select-int special" @click="handleClickLevel('80+')">80+</span>
                    <span
                        v-for="i in 10"
                        :key="i + 80"
                        class="select-int"
                        @click="handleClickLevel(i + 80)"
                    >{{ i + 80 }}</span>
                </div>
            </template>
        </div>
    </div>
</template>

<script>
import { weaponsData } from "@asset/weapons";

import CustomForm from "../CustomForm";

export default {
    name: "SelectWeaponLevel",
    components: {
        CustomForm,
    },
    props: {
        weaponName: {
            type: String,
            required: true,
        }
    },
    data: function() {
        return {
            refine: 1,

            args: {},
        }
    },
    methods: {
        handleClickLevel(key) {
            let config = {};
            let levelStr = key.toString();

            config.refine = this.refine;
            config.level = parseInt(levelStr);
            config.ascend = levelStr.indexOf("+") !== -1;
            config.args = this.getExtraConfig();

            this.$emit("select", config);
        },

        getExtraConfig() {
            if (!this.needExtraConfig) {
                return {};
            }

            let vm = this.$refs.extraConfig;
            if (vm.compact) {
                return vm.compact();
            } else {
                return Object.assign({}, vm.$data);
            }
        }
    },
    computed: {
        w() {
            return weaponsData[this.weaponName];
        },

        star() {
            return this.w ? this.w.star : 0;
        },

        needExtraConfig() {
            return this.w ? !!this.w.config : false;
        }
    },
}
</script>

<style scoped>
.special {
    background: #123456;
    color: white;
}

.special:hover {
    background: #345678;
}
</style>