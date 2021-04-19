<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">技能等级（包含命之座加成）</h3>
            <el-input-number
                class="skill"
                v-model="skill1"
                :min="1"
                :max="13"
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
            :value="level | levelText"
            title="角色等级"
            @input="handleChangeLevel"
        ></select-level>
    </div>
</template>

<script>
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
            }
        }
    },
    methods: {
        handleChangeLevel(e) {
            this.level.level = parseInt(e);
            this.level.ascend = e.indexOf("+") !== -1;
        },

        getCharacterConfig() {
            return {
                skill1: this.skill1,
                skill2: this.skill2,
                skill3: this.skill3,
                constellation: this.constellation,
                ascend: this.level.ascend,
                level: this.level.level,
            }
        },

        setCharacterConfig(config) {
            this.skill1 = config.skill1,
            this.skill2 = config.skill2;
            this.skill3 = config.skill3;
            this.constellation = config.constellation;

            this.level.ascend = config.ascend;
            this.level.level = config.level;
        }
    },
    filters: {
        levelText(obj) {
            // console.log(obj);
            let a = obj.ascend;
            let lvl = obj.level;

            let temp = [20, 40, 50, 60, 70, 80];
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
.skill {
    margin-right: 18px;
}
</style>