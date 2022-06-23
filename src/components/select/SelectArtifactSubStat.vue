<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-option
            v-for="item in list"
            :key="item.name"
            :label="item.title"
            :value="item.name"
        ></el-option>
    </el-select>
</template>

<script>
import { subStats, artifactTags } from "@const/artifact"

let list = []
for (let name of subStats) {
    list.push({
        name,
        title: artifactTags[name].chs
    })
}
Object.freeze(list)

export default {
    name: "SelectArtifactSubStat",
    props: ["modelValue"],
    emits: ["update:modelValue"],
    created() {
        this.list = list
    }
}
</script>

<style scoped>

</style>