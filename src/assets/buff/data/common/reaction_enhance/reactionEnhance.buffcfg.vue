<template>
    <div>
        <single-value
            title="数值（0-1）"
            v-model="value"
        ></single-value>

        <div class="buff-config-item">
            <h3 class="buff-config-title">反应类型</h3>
            <el-checkbox-group
                v-model="reactionTypes"
                size="small"
            >
                <el-checkbox-button label="melt">融化</el-checkbox-button>
                <el-checkbox-button label="vaporize">蒸发</el-checkbox-button>
                <el-checkbox-button label="overload">超载</el-checkbox-button>
                <el-checkbox-button label="superconduct">超导</el-checkbox-button>
                <el-checkbox-button label="electroCharged">感电</el-checkbox-button>
                <el-checkbox-button label="swirl">扩散</el-checkbox-button>
            </el-checkbox-group>
        </div>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "ReactionEnhance.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "0.2",
            reactionTypes: [],
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "reactionEnhance",
                value: parseFloat(this.value) ?? 0.2,
                reactionTypes: this.reactionTypes,
            }
        },

        getBuff() {
            return {
                name: "reactionEnhance",
                args: {
                    value: this.value,
                    reactonTypes: this.reactionTypes,
                }
            }
        },

        setBuff(buff) {
            this.value = (buff.value ?? "0.2").toString(),
            this.reactionTypes = buff.reactonTypes ?? [];
        }
    }
}
</script>