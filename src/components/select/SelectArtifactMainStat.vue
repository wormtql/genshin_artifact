<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-option
            v-if="includeAny"
            label="任意"
            value="any"
        ></el-option>
        <el-option
            v-for="item in tagList"
            :key="item.name"
            :label="item.title"
            :value="item.name"
        ></el-option>
    </el-select>
</template>

<script>
import { secondaryTags } from "@asset/tags"

let list = []
for (let name in secondaryTags) {
    list.push({
        name,
        title: secondaryTags[name].chs
    })
}

export default {
    name: "SelectArtifactMainStat",
    props: {
        value: {},
        includeAny: {
            default: true
        }
    },
    created() {
        this.tagList = list
    }
}
</script>