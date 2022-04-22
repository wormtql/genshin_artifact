<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        size="small"
        :multiple="multiple"
        :placeholder="placeholder"
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
import { artifactTags, mainStatMap } from "@const/artifact"

export default {
    name: "SelectArtifactMainStat",
    props: {
        value: {},
        includeAny: {
            default: true
        },
        multiple: { default: false },
        position: { default: null },
        placeholder: { type: String, default: "请选择" },
    },
    // created() {
    //     this.tagList = list
    // },
    computed: {
        tagList() {
            let list = []
            if (!this.position) {
                for (let name in artifactTags) {
                    list.push({
                        name,
                        title: artifactTags[name].chs
                    })
                }
            } else {
                for (let name of mainStatMap[this.position]) {
                    list.push({
                        name,
                        title: artifactTags[name].chs
                    })
                }
            }

            return list
        }
    }
}
</script>