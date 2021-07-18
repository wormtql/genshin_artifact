<template>
    <div>
        <single-value
            title="砂糖的元素精通"
            v-model="value"
            style="margin-bottom: 0"
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "Shatang2.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "600",
        }
    },
    methods: {
        getStandardConfig() {
            let em = parseFloat(this.value) ?? 600;
            let value = 0.2 * em;
            return {
                type: "em",
                value,
            }
        },

        getBuff() {
            return {
                name: "shatang2",
                args: {
                    value: (parseFloat(this.value) ?? 600),
                }
            }
        },

        setBuff(buff) {
            this.value = (buff.value ?? 600).toString();
        }
    }
}
</script>