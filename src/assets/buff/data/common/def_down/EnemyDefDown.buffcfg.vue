<template>
    <div>
        <single-value
            title="数值（0-100）"
            v-model="value"
            :percentage="true"
        ></single-value>
    </div>
</template>

<script>
import SingleValue from "@asset/buff/common_config_item/SingleValue";

export default {
    name: "EnemyDefDown.buffcfg",
    components: {
        SingleValue,
    },
    data: function () {
        return {
            value: "10",
        }
    },
    methods: {
        getValue() {
            return (parseFloat(this.value) ?? 10) / 100;
        },

        getStandardConfig() {
            return {
                type: "enemyDefDown",
                value: this.getValue(),
            }
        },

        getBuff() {
            return {
                name: "enemyDefDown",
                args: {
                    value: this.getValue(),
                }
            }
        },

        setBuff(buff) {
            this.value = ((buff.value ?? 0) * 100).toString();
        }
    }
}
</script>