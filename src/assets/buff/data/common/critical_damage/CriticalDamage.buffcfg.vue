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
    name: "CriticalDamage.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "20",
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "critical",
                value: (parseFloat(this.value) ?? 20) / 100,
            }
        }
    }
}
</script>