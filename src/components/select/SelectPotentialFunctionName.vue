<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-option
            v-for="item in potentialFunctionNames"
            :key="item.name"
            :label="item.chs"
            :value="item.name"
        ></el-option>
    </el-select>
</template>

<script>
import { potentialFunctionData } from "@potentialFunction"

let _potentialFunctionNames = []
for (const name in potentialFunctionData) {
    const chs = potentialFunctionData[name].chs
    _potentialFunctionNames.push({
        name, chs
    })
}
Object.freeze(_potentialFunctionNames)

export default {
    name: "SelectPotentialFunctionName",
    props: ["value"],
    created() {
        this.potentialFunctionNames = _potentialFunctionNames
    }
}
</script>

<style scoped>

</style>