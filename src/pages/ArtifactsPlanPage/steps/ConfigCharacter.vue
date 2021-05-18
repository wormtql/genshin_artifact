<template>
    <div>
        <component
            v-if="needExtraConfig"
            :is="c.config"
            ref="extraConfig"
        >
        </component>

        <div class="config-item">
            <h3 class="config-title">技能等级（包含命之座加成）</h3>
            <el-input-number
                class="skill"
                v-model="skill1"
                :min="1"
                :max="11"
                size="small"
            ></el-input-number>
            <el-input-number
                class="skill"
                v-model="skill2"
                :min="1"
                :max="13"
                size="small"
            ></el-input-number>
            <el-input-number
                class="skill"
                v-model="skill3"
                :min="1"
                :max="13"
                size="small"
            ></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">命之座</h3>
            <el-input-number
                v-model="constellation"
                :min="0"
                :max="6"
                size="small"
            ></el-input-number>
        </div>

        <select-level
            :value="levelText"
            title="角色等级"
            @input="handleChangeLevel"
        ></select-level>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";

import SelectLevel from "@c/select/SelectLevel";

// import deepCopy from "@util/deepcopy";

export default {
    name: "ConfigCharacter",
    components: {
        SelectLevel,
    },
    props: {
        value: {
            type: Object,
            required: true,
        }
    },
    data: function () {
        return {
            skill1: 6,
            skill2: 6,
            skill3: 6,
            constellation: 0,

            level: {
                ascend: false,
                level: 1,
            },

            characterName: "anbo",
        }
    },
    methods: {
        handleChangeLevel(e) {
            this.level.level = parseInt(e);
            this.level.ascend = e.indexOf("+") !== -1;
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
        },

        getCharacterConfig() {
            return {
                skill1: this.skill1,
                skill2: this.skill2,
                skill3: this.skill3,
                constellation: this.constellation,
                ascend: this.level.ascend,
                level: this.level.level,

                args: this.getExtraConfig(),
            }
        },

        setCharacterConfig(config) {
            this.characterName = config.name;

            this.skill1 = config.skill1,
            this.skill2 = config.skill2;
            this.skill3 = config.skill3;
            this.constellation = config.constellation;

            this.level.ascend = config.ascend;
            this.level.level = config.level;

            this.$nextTick(() => {
                this.$refs.extraConfig.setData(config.args);
            })
        },

        setCharacterName(name) {
            this.characterName = name;
        }
    },
    computed: {
        levelText() {
            let a = this.level.ascend;
            let lvl = this.level.level;

            let temp = [20, 40, 50, 60, 70, 80];
            if (temp.indexOf(lvl) === -1) {
                return lvl.toString();
            } else {
                return `${lvl}${a ? "+" : "-"}`;
            }
        },

        c() {
            return charactersData[this.characterName];
        },

        needExtraConfig() {
            return !!this.c.config;
        }
    }
}
</script>

<style scoped>
.skill {
    margin-right: 18px;
}
</style>