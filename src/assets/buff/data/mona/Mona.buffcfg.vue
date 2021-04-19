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

import pathAccess from "@util/pathAccess";

export default {
    name: "Mona.buffcfg",
    data: function () {
        return {
            skillLevel: 6,
        }
    },
    methods: {
        getValue() {
            return pathAccess(charactersData, "mona.skill.q.bonus")[this.skillLevel - 1];
        },

        getStandardConfig() {
            return {
                type: "bonus",
                value: this.getValue(),
            }
        },

        getBuff() {
            return {
                name: "mona",
                args: {
                    skillLevel: this.skillLevel,
                }
            }
        },

        setBuff(buff) {
            this.skillLevel = buff.skillLevel;
        }
    }
}
</script>