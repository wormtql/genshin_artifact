<template>
    <div>
        <single-value
            title="九条裟罗的基础攻击力"
            :percentage="false"
            v-model="baseATK"
        ></single-value>
        <div class="buff-config-item">
            <h3 class="buff-config-title">E技能等级</h3>

            <el-input-number
                size="small"
                :min="1"
                :max="13"
                v-model="skillLevel"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
import SingleValue from '../../common_config_item/SingleValue.vue';

import { charactersData } from "@asset/characters";

let skill = charactersData["jiutiaoshaluo"].skill;

export default {
    name: "Jiutiaoshaluo.buffcfg",
    components: {
        SingleValue
    },
    data: function () {
        return {
            skillLevel: 6,
            baseATK: "600",
        }
    },
    methods: {
        getStandardConfig() {
            let p = skill.e.bonus[this.skillLevel - 1];
            let value = p * (parseFloat(this.baseATK) ?? 600);

            return {
                type: "atk-static",
                value,
            }
        },

        getBuff() {
            return {
                name: "jiutiaoshaluo",
                args: {
                    baseATK: parseFloat(this.baseATK) ?? 600,
                    skillLevel: this.skillLevel,
                }
            }
        },

        setBuff(buff) {
            this.skillLevel = buff.skillLevel ?? 6;
            this.baseATK = (buff.baseATK ?? 600).toString();
        }
    }
}
</script>