<template>
    <div>
        <div class="buff-config-item">
            <h3 class="buff-config-title">E技能等级</h3>
            <el-input-number
                :min="1"
                :max="13"
                size="small"
                v-model="skill2Level"
            ></el-input-number>
        </div>
        <div class="buff-config-item">
            <h3 class="buff-config-title">受buff角色的最大元素能量</h3>
            <el-input-number
                :min="40"
                :max="90"
                size="small"
                v-model="energy"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";

let skill = charactersData["leidianjiangjun"].skill;

export default {
    name: "Fengyuanwanye.buffcfg",
    components: {
    },
    data: function () {
        return {
            skill2Level: 6,
            energy: 80,
        }
    },
    methods: {
        getStandardConfig() {
            let bonusPer = skill.e.bonus[this.skill2Level - 1];
            let bonus = bonusPer * this.energy;
            return {
                type: "qBonus",
                value: bonus,
            }
        },

        getBuff() {
            return {
                name: "leidianjiangjun",
                args: {
                    skill2Level: this.skill2Level,
                    energy: this.energy,
                }
            }
        },

        setBuff(buff) {
            this.skill2Level = buff.skill2Level ?? 6;
            this.energy = buff.energy ?? 80;
        }
    }
}
</script>