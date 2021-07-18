<template>
    <div>
        <select-element-type
            :elements="['fire', 'ice', 'water', 'thunder', 'rock', 'wind']"
            v-model="element"
            style="margin-bottom: 8px"
        ></select-element-type>
        <single-value
            title="数值"
            v-model="value"
            :percentage="true"
            style="margin-bottom: 0"
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";
import SelectElementType from "@c/select/SelectElementType";

export default {
    name: "ElementalBonus.buffcfg",
    components: {
        SingleValue,
        SelectElementType,
    },
    data: function () {
        return {
            value: "10",
            element: "fire",
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "elementalBonus",
                value: (parseFloat(this.value) ?? 10) / 100,
                element: this.element,
            }
        },

        getBuff() {
            return {
                name: "elementalBonus",
                args: {
                    value: (parseFloat(this.value) ?? 10) / 100,
                    element: this.element,
                }
            }
        },

        setBuff(buff) {
            this.value = (buff.value * 100).toString();
            this.element = buff.element;
        }
    }
}
</script>