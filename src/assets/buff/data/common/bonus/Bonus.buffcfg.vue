<template>
    <div>
        <single-value
            title="数值"
            v-model="value"
            percentage
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "Bonus.buffcfg",
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
                type: "bonus",
                value: (parseFloat(this.value) ?? 10) / 100,
            }
        }
    }
}
</script>