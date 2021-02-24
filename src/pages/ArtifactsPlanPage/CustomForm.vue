<template>
    <div>
        <div v-if="config.type === 'int'" class="container">
            <span
                v-for="item in (config.max - config.min + 1)"
                :key="item"
                class="select-int"
                :class="{active: value === item - 1 + config.min}"
                @click="handleClickSelectInt(item - 1 + config.min)"
            >
                {{ item + config.min - 1 }}
            </span>
        </div>

        <div v-if="config.type === 'bool'" class="container">
            <el-switch
                active-text="是"
                inactive-text="否"
                :value="value"
                @change="handleClickSelectBool($event)"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "CustomForm",
    props: {
        value: {
            required: true,
        },

        config: {
            type: Object,
        }
    },
    methods: {
        handleClickSelectInt(item) {
            this.$emit("input", item);
        },

        handleClickSelectBool(item) {
            this.$emit("input", item);
        }
    }
}
</script>

<style scoped>
</style>