<template>
    <el-select
        size="small"
        :value="value"
        @input="$emit('input', $event)"
    >
        <el-option
            v-for="item in positions"
            :key="item.name"
            :label="item.chs"
            :value="item.name"
        ></el-option>
    </el-select>
</template>

<script>
import {positions} from "@const/misc"

export default {
    name: "SelectArtifactSlot",
    props: ["value"],
    created() {
        this.positions = positions
    }
}
</script>

<style scoped>

</style>