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
    name: "DefPercentage.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "10",
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "def-percentage",
                value: (parseFloat(this.value) ?? 10) / 100,
            }
        },

        getBuff() {
            return {
                name: "defPercentage",
                args: {
                    value: (parseFloat(this.value) ?? 10) / 100,
                }
            }
        },

        setBuff(buff) {
            this.value = ((buff.value ?? 0.1) * 100).toString();
        }
    }
}
</script>