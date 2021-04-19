<template>
    <div>
        <single-value
            title="数值"
            v-model="value"
            :percentage="true"
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "Critical.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "10",
        }
    },
    methods: {
        getValue() {
            return (parseFloat(this.value) ?? 10) / 100;
        },

        getStandardConfig() {
            return {
                type: "critical",
                value: this.getValue(),
            }
        },

        getBuff() {
            return {
                name: "critical",
                args: {
                    value: this.getValue(),
                }
            }
        },

        setBuff(buff) {
            this.value = (buff.value * 100).toString();
        }
    }
}
</script>