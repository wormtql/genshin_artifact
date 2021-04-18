<template>
    <div>
        <div class="buff-config-item">
            <p class="buff-config-title">技能等级</p>
            <el-input-number
                v-model="skillLevel"
                :min="1"
                :max="13"
                size="mini"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";

export default {
    name: "Mona.buffcfg",
    data: function () {
        return {
            skillLevel: 6,
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "bonus",
                value: charactersData["mona"].skill.q.bonus[this.skillLevel - 1],
            }
        }
    }
}
</script>