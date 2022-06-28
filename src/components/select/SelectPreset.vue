<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-option
            v-for="item in list"
            :key="item"
            :label="item"
            :value="item"
        ></el-option>
    </el-select>
</template>

<script setup>
import {usePresetStore} from "@/store/pinia/preset"

const presetStore = usePresetStore()

const props = defineProps({
    modelValue: String
})

const list = computed(() => {
    let results = []
    for (let preset of presetStore.allFlat.value) {
        results.push(preset.name)
    }
    return results
})
</script>

<style scoped>

</style>