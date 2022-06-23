<template>
    <el-select
        :model-value="props.modelValue"
        @update:modelValue="emits('update:modelValue', $event)"
        :multiple="props.multiple"
        :placeholder="props.placeholder"
    >
        <el-option
            v-if="props.includeAny"
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

<script setup lang="ts">
import { computed } from "vue"
import { artifactTags, mainStatMap } from "@const/artifact"
import type {ArtifactMainStatName, ArtifactPosition} from "@/types/artifact"

interface Emits {
    (e: "update:modelValue", v: ModelValue): void
}

const emits = defineEmits<Emits>()

type ModelValue = "any" | ArtifactMainStatName | ArtifactMainStatName[] | null

interface Props {
    modelValue: ModelValue,
    includeAny?: boolean,
    multiple?: boolean,
    position?: ArtifactPosition | null,
    placeholder?: string
}

const props = withDefaults(defineProps<Props>(), {
    modelValue: null,
    includeAny: true,
    multiple: false,
    position: null,
    placeholder: "请选择"
})

const tagList = computed(() => {
    let list = []
    if (!props.position) {
        for (let name in artifactTags) {
            list.push({
                name,
                title: artifactTags[name].chs
            })
        }
    } else {
        for (let name of mainStatMap[props.position]) {
            list.push({
                name,
                title: artifactTags[name].chs
            })
        }
    }

    return list
})
</script>