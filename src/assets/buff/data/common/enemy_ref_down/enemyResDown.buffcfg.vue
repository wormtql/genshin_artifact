<template>
    <div>
        <single-value
            title="数值（0-100）"
            v-model="value"
            :percentage="true"
        ></single-value>

        <div class="buff-config-item">
            <h3 class="buff-config-title">元素类型</h3>
            <select-element-multi
                v-model="elementTypes"
            ></select-element-multi>
        </div>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";
import SelectElementMulti from "@c/select/SelectElementMulti";

export default {
    name: "EnemyResDown.buffcfg",
    components: {
        SingleValue,
        SelectElementMulti,
    },
    data: function () {
        return {
            value: "10",
            elementTypes: [],
        }
    },
    methods: {
        getStandardConfig() {
            return {
                type: "enemyResDown",
                value: (parseFloat(this.value) ?? 10) / 100,
                elementTypes: this.elementTypes,
            }
        },

        getBuff() {
            return {
                name: "enemyResDown",
                args: {
                    value: this.value,
                    elementTypes: this.elementTypes,
                }
            }
        },

        setBuff(buff) {
            this.value = (buff.value ?? "10").toString(),
            this.elementTypes = buff.reactonTypes ?? [];
        }
    }
}
</script>