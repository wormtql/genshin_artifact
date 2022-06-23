<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        :multiple="multiple"
    >
        <el-option
            v-for="item in positions"
            :key="item"
            :label="item"
            :value="item"
        ></el-option>
    </el-select>
</template>

<script>
import {positions} from "@/constants/artifact"

// const items = []

export default {
    name: "SelectArtifactSlot",
    props: {
        value: {},
        multiple: { default: false }
    },
    created() {
        this.positions = positions
    }
}
</script>

<style scoped>

</style>