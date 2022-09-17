<template>
    <el-cascader
        :model-value="props.modelValue"
        :options="options"
        :props="{ expandTrigger: 'hover', emitPath: false }"
        placeholder="角色"
        :show-all-levels="false"
        style="width: 100%"
        @change="emits('update:modelValue', $event)"
    >
        <template #default="{ node, data }">
            <div v-if="!node.isLeaf">
                <span>{{ data.label }}</span>
            </div>
            <div v-else class="option-item flex-row">
                <img :src="data.avatar">
                <span :style="{ color: data.color }">{{ data.label }}</span>
            </div>
        </template>
    </el-cascader>
</template>

<script setup lang="ts">
// @ts-ignore
import { characterByElement } from "@character"
import qualityColors from "@const/quality_colors"
import {useI18n} from "@/i18n/i18n"

const { t } = useI18n()

const props = defineProps<{
    modelValue: string
}>()

const emits = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>()

const options = computed(() => {
    const options = []
    for (const element in characterByElement) {
        options.push({
            label: t('ele', element),
            value: element,
            children: characterByElement[element].map((character: any) => ({
                label: t('character.' + character.name),
                value: character.name,
                avatar: character.avatar,
                color: qualityColors[character.star - 1],
            }))
        })
    }
    return options
})
</script>

<style lang="scss" scoped>
.option-item {
    img {
        width: 28px;
        height: 28px;
        margin-right: 8px;
        border-radius: 50%;
    }
}
</style>
