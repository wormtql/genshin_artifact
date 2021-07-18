<template>
    <div>
        <select-element-type
            :elements="['fire', 'ice', 'water', 'thunder']"
            v-model="element"
            style="margin-bottom: 8px"
        ></select-element-type>
        <single-value
            title="枫原万叶的元素精通"
            v-model="value"
            style="margin-bottom: 0"
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";
import SelectElementType from "@c/select/SelectElementType";

export default {
    name: "Fengyuanwanye.buffcfg",
    components: {
        SingleValue,
        SelectElementType,
    },
    data: function () {
        return {
            value: "600",
            element: "fire",
        }
    },
    methods: {
        getStandardConfig() {
            let em = parseFloat(this.value) ?? 600;
            let value = 0.04 * em / 100;
            return {
                type: "elementalBonus",
                value,
                element: this.element,
            }
        },

        getBuff() {
            return {
                name: "fengyuanwanye",
                args: {
                    value: (parseFloat(this.value) ?? 600),
                    element: this.element,
                }
            }
        },

        setBuff(buff) {
            this.value = (buff.value ?? 600).toString();
            this.element = buff.element;
        }
    }
}
</script>