<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
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
    props: ["modelValue"],
    emits: ["update:modelValue"],
    created() {
        this.potentialFunctionNames = _potentialFunctionNames
    }
}
</script>

<style scoped>

</style>