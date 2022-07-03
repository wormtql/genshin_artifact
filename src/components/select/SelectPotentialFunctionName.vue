<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-option
            v-for="item in potentialFunctionNames"
            :key="item.name"
            :label="t('pfName', item.name)"
            :value="item.name"
        ></el-option>
    </el-select>
</template>

<script>
import { potentialFunctionData } from "@potentialFunction"
import {useI18n} from "@/i18n/i18n";

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
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>

<style scoped>

</style>