<template>
    <div>
        <single-value
            title="班尼特的基础攻击力"
            v-model="baseAtk"
        ></single-value>

        <div class="buff-config-item">
            <p class="buff-config-title">是否1命</p>
            <el-switch
                v-model="isConste1"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>

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
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "Bannite.buffcfg",
    components: {
        SingleValue
    },
    data: function () {
        return {
            skillLevel: 6,
            baseAtk: "900",
            isConste1: true,
        }
    },
    methods: {
        getStandardConfig() {
            let baseAtk = parseFloat(this.baseAtk) ?? 900;
            let ratio = pathAccess(charactersData, "bannite.skill.q.ratio")[this.skillLevel - 1];

            if (this.isConste1) {
                ratio += 0.2;
            }

            return {
                type: "atk-static",
                value: baseAtk * ratio,
            }
        }
    }
}
</script>