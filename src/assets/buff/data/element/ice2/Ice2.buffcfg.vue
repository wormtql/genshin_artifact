<template>
    <div>
        <single-value
            title="等效暴击率"
            v-model="value"
            percentage
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "Ice2.buffcfg",
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
                name: "ice2",
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