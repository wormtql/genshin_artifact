<template>
    <div>
        <!-- 其他参数 -->
        <component
            :if="needExtraConfig"
            :is="w.config"
            ref="extraConfig"
            v-model="value.args"
        >
        </component>

        <div v-if="star >= 3" class="config-item">
            <h3 class="config-title">精炼等级</h3>
            <el-input-number
                :value="value.refine"
                :min="1"
                :max="5"
                @change="handleChangeRefine"
            ></el-input-number>
        </div>

        <select-level
            :star="star"
            :value="levelText"
            @input="handleClickLevel"
            title="武器等级"
        ></select-level>
    </div>
</template>

<script>
import { weaponsData } from "@asset/weapons";
import deepCopy from "@util/deepcopy";

import SelectLevel from "@c/select/SelectLevel";

export default {
    name: "SelectWeaponLevel",
    components: {
        SelectLevel,
    },
    props: {
        weaponName: {
            type: String,
            required: true,
        },

        value: {
            type: Object,
            required: true,
        }
    },
    // data: function() {
    //     return {
    //         refine: 1,

    //         args: {},
    //     }
    // },
    methods: {
        handleClickLevel(e) {
            let temp = deepCopy(this.value);
            temp.level = parseInt(e);
            temp.ascend = e.indexOf("+") !== -1;

            this.$emit("input", temp);
        },

        handleChangeRefine(e) {
            let temp = deepCopy(this.value);
            temp.refine = e;

            this.$emit("input", temp);
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
    watch: {
        weaponName(name) {
            if (this.$parent.lock) {
                return;
            }
            let data = weaponsData[name];
            if (data && data.config) {
                let init = {};
                if (data.config.first) {
                    init = data.config.first();
                }
                let temp = deepCopy(this.value);
                temp.args = init;
                this.$emit("input", temp);
            } else {
                let temp = deepCopy(this.value);
                temp.args = {};
                this.$emit("input", temp);
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
        },

        levelText() {
            let a = this.value.ascend;
            let lvl = this.value.level;

            let temp = this.star >= 3 ? [20, 40, 50, 60, 70, 80] : [20, 40, 50, 60];
            if (temp.indexOf(lvl) === -1) {
                return lvl.toString();
            } else {
                return `${lvl}${a ? "+" : "-"}`;
            }
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