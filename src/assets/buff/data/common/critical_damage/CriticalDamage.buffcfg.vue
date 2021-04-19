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
        getValue() {
            return (parseFloat(this.value) ?? 20) / 100;
        },

        getStandardConfig() {
            return {
                type: "criticalDamage",
                value: this.getValue(),
            }
        },

        getBuff() {
            return {
                name: "criticalDamage",
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