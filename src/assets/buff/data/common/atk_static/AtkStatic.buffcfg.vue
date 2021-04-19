<template>
    <div>
        <single-value title="数值" v-model="value"></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "AtkStatic.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "100",
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "atk-static",
                value: parseFloat(this.value) ?? 100,
            }
        },

        getBuff() {
            return {
                name: "atkStatic",
                args: {
                    value: parseFloat(this.value) ?? 100, 
                }
            }
        },

        setBuff(buff) {
            this.value = buff.value.toString();
        }
    }
}
</script>