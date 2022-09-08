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
import {useI18n} from "../../i18n/i18n"


export default {
    name: "SelectArtifactSubStat",
    props: ["modelValue"],
    emits: ["update:modelValue"],
    computed: {
        list() {
            const { t } = useI18n()
            let result = []
            for (let name of subStats) {
                result.push({
                    name,
                    // title: artifactTags[name].chs
                    title: t("stat", name)
                })
            }
            return result
        }
    }
}
</script>

<style scoped>

</style>