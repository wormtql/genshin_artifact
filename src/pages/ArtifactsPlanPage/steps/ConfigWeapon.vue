<template>
    <div>
        <!-- 其他参数 -->
        <component
            :if="!!w.config"
            :is="w.config"
            ref="extraConfig"
        >
        </component>

        <div v-if="w.star >= 3" class="config-item">
            <h3 class="config-title">精炼等级</h3>
            <el-input-number
                v-model="refine"
                :min="1"
                :max="5"
                size="small"
            ></el-input-number>
        </div>

        <select-level
            :star="w.star"
            :value="levelText"
            @input="handleClickLevel"
            title="武器等级"
        ></select-level>
    </div>
</template>

<script>
import { weaponsData } from "@asset/weapons";
// import deepCopy from "@util/deepcopy";

import SelectLevel from "@c/select/SelectLevel";

export default {
    name: "SelectWeaponLevel",
    components: {
        SelectLevel,
    },
    data: function() {
        return {
            refine: 1,

            level: {
                level: 1,
                ascend: false,
            },

            weaponName: "liegong",
        }
    },
    methods: {
        getWeaponConfig() {
            return {
                refine: this.refine,
                level: this.level.level,
                ascend: this.level.ascend,

                args: this.getExtraConfig(),
            }
        },

        setWeaponConfig(weapon) {
            this.refine = weapon.refine;
            this.level.level = weapon.level;
            this.level.ascend = weapon.ascend;
            this.weaponName = weapon.name;

            this.$nextTick(() => {
                if (this.w.config) {
                    this.$refs.extraConfig.setData(weapon.args);
                }
            })
            
        },

        handleClickLevel(e) {
            this.level.level = parseInt(e);
            this.level.ascend = e.indexOf("+") !== -1;
        },

        getExtraConfig() {
            if (!this.w.config) {
                return {};
            }

            let vm = this.$refs.extraConfig;
            if (vm.compact) {
                return vm.compact();
            } else {
                return Object.assign({}, vm.$data);
            }
        },

        // for parent to call
        setWeaponName(name) {
            if (name !== this.weaponName) {
                this.weaponName = name;

                if (weaponsData[name].star < 3) {
                    if (this.level.level > 70 || (this.level.level === 70 && this.level.ascend)) {
                        this.level.level = 70;
                        this.level.ascend = false;
                    }
                }
            }
        }
    },
    computed: {
        w() {
            return weaponsData[this.weaponName];
        },

        levelText() {
            let a = this.level.ascend;
            let lvl = this.level.level;

            let temp = this.w.star >= 3 ? [20, 40, 50, 60, 70, 80] : [20, 40, 50, 60];
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