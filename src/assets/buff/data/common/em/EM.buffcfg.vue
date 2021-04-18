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
        getStandardConfig() {
            return {
                type: "em",
                value: parseFloat(this.value) ?? 80,
            }
        }
    }
}
</script>