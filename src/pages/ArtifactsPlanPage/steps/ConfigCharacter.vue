<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">技能等级（包含命之座加成）</h3>
            <el-input-number
                class="skill"
                :value="value.skill1"
                @change="handleChange('skill1', $event)"
                :min="1"
                :max="13"
            ></el-input-number>
            <el-input-number
                class="skill"
                :value="value.skill2"
                @change="handleChange('skill2', $event)"
                :min="1"
                :max="13"
            ></el-input-number>
            <el-input-number
                class="skill"
                :value="value.skill3"
                @change="handleChange('skill3', $event)"
                :min="1"
                :max="13"
            ></el-input-number>
        </div>

        <div class="config-item">
            <h3 class="config-title">命之座</h3>
            <el-input-number
                :value="value.constellation"
                @change="handleChange('constellation', $event)"
                :min="0"
                :max="6"
            ></el-input-number>
        </div>

        <select-level title="角色等级" :value="value | levelText" @input="handleChangeLevel"></select-level>
    </div>
</template>

<script>
import SelectLevel from "@c/select/SelectLevel";

import deepCopy from "@util/deepcopy";

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
    // data: function () {
    //     return {
    //         skill1: 6,
    //         skill2: 6,
    //         skill3: 6,
    //         constellation: 0,
    //     }
    // },
    methods: {
        handleChangeLevel(e) {
            let temp = deepCopy(this.value);
            temp.level = parseInt(e);
            temp.ascend = e.indexOf("+") !== -1;

            this.$emit("input", temp);
        },

        handleChange(field, value) {
            let temp = deepCopy(this.value);
            temp[field] = value;

            this.$emit("input", temp);
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