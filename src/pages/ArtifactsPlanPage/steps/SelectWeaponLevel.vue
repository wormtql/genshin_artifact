<template>
    <div>
        <!-- 其他参数 -->
        <div
            v-for="arg in argConfigs"
            :key="arg.name"
            class="config-item"
        >
            <h3 class="title">{{ arg.chs }}</h3>
            <custom-form
                :config="arg"
                :value="args[arg.name]"
                @input="$set(args, arg.name, $event)"
            ></custom-form>
        </div>

        <div v-if="star >= 3" class="config-item">
            <h3 class="title">精炼等级</h3>
            <div class="refine-div">
                <span
                    v-for="i in 5"
                    :key="i"
                    class="select-int"
                    :class="{active: refine === i}"
                    @click="refine = i"
                >{{ i }}</span>
            </div>
        </div>

        <div class="config-item">
            <h3 class="title">武器等级</h3>
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
            config.args = this.args;

            this.$emit("select", config);
        }
    },
    computed: {
        w() {
            return weaponsData[this.weaponName];
        },

        star() {
            return this.w.star;
        },

        argConfigs() {
            return this.w.args || [];
        }
    },
    beforeMount() {
        this.args = {};
        if (this.w.args) {
            for (let argConfig of this.w.args) {
                this.$set(this.args, argConfig.name, argConfig.default);
            }
        }
    }
}
</script>

<style scoped>
.config-item {
    /* border-left: 5px solid #a0cfff; */
    /* padding-left: 20px; */
    padding: 18px;
    margin-bottom: 20px;
    box-shadow: 0 0 20px 1px rgba(0, 0, 0, 0.1);
    border-radius: 3px;
}

.title {
    font-size: 16px;
    color: #555555;
    margin: 0;
    margin-bottom: 12px;
}

.special {
    background: #123456;
    color: white;
}

.special:hover {
    background: #345678;
}
</style>