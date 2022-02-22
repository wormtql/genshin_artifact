<template>
    <el-select
        :value="value"
        size="small"
        @input="$emit('input', $event)"
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
    props: ["value"],
    created() {
        this.list = list
    }
}
</script>

<style scoped>

</style>