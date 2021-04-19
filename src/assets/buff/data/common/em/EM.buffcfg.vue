<template>
    <div>
        <single-value
            title="数值"
            v-model="value"
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "EM.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "80",
        }
    },
    methods: {
        getValue() {
            return parseFloat(this.value) ?? 80;
        },

        getStandardConfig() {
            return {
                type: "em",
                value: this.getValue(),
            }
        },

        getBuff() {
            return {
                name: "em",
                args: {
                    value: this.getValue()
                }
            }
        },

        setBuff(buff) {
            this.value = buff.value.toString();
        }
    }
}
</script>