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
        getStandardConfig() {
            return {
                type: "critical",
                value: (parseFloat(this.value) ?? 10) / 100,
            }
        }
    }
}
</script>