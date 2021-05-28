<template>
    <div>
        <div class="config-item select-weapon">
            <img :src="w.url" class="image" >
            <h3 class="config-title">选择武器</h3>
            <select-weapon
                :value="weaponName"
                @input="handleChangeWeapon"
                :type="weaponType"
            ></select-weapon>
            <div class="weapon-effect" v-if="w.effect">
                {{ w.effect }}
            </div>
        </div>

        <!-- 其他参数 -->
        <component
            :if="!!w.config"
            :is="configComponent"
            ref="extraConfig"
        ></component>

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
import { weaponsByType, weaponsData } from "@asset/weapons";

import SelectLevel from "@c/select/SelectLevel";
import SelectWeapon from "@c/select/SelectWeapon";

export default {
    name: "SelectWeaponLevel",
    components: {
        SelectLevel,
        SelectWeapon,
    },
    inject: ["notifyChange"],
    data: function() {
        return {
            refine: 1,

            level: {
                level: 1,
                ascend: false,
            },

            weaponName: "liegong",
            weaponType: "bow",
        }
    },
    methods: {
        handleChangeWeapon(weaponName) {
            if (weaponName === this.weaponName) {
                return
            }
            this.weaponName = weaponName;

            if (weaponsData[weaponName].star < 3) {
                if (this.level.level > 70 || (this.level.level === 70 && this.level.ascend)) {
                    this.level.level = 70;
                    this.level.ascend = false;
                }
            }

            this.notifyChange("weapon", weaponName);
        },

        handleClickLevel(e) {
            this.level.level = parseInt(e);
            this.level.ascend = e.indexOf("+") !== -1;
        },

        // called by parent
        setWeaponType(type) {
            if (this.weaponType === type) {
                return;
            }
            this.weaponType = type;
            this.weaponName = weaponsByType[this.weaponType][0].name;
        },

        // called by parent
        getWeaponConfig() {
            return {
                name: this.weaponName,
                refine: this.refine,
                level: this.level.level,
                ascend: this.level.ascend,

                args: this.getExtraConfig(),
            }
        },

        // called by parent
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
        },

        configComponent() {
            if (typeof this.w.config === "function") {
                return this.w.config();
            } else {
                return this.w.config;
            }
        }
    },
}
</script>

<style lang="scss" scoped>
.select-weapon {
    position: relative;
    // overflow: hidden;

    .image {
        position: absolute;
        right: 20px;
        top: -20px;
        // bottom: 0;
        height: 180px;
        // z-index: -1;
        // opacity: 0.3;
        pointer-events: none;
    }

    .weapon-effect {
        margin-right: 180px;
    }
}

.special {
    background: #123456;
    color: white;
}

.special:hover {
    background: #345678;
}

.weapon-effect {
    border-radius: 3px;
    margin-top: 12px;
    padding: 8px 16px;
    font-size: 13px;
    color: #909399;
    background: #f4f4f5;
}
</style>