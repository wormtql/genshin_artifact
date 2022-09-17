<template>
<!-- use :key to force rerender, or there will be a bug   -->
    <el-cascader
        :key="props.characterName"
        :model-value="props.modelValue"
        :options="options"
        :props="{ expandTrigger: 'hover', emitPath: false }"
        @change="emits('update:modelValue', $event)"
        style="width: 61.8%"
    ></el-cascader>
</template>

<script setup lang="ts">
// @ts-ignore
import { characterData } from "@character"
import type {CharacterName} from "@/types/character"
import {deepCopy} from "@/utils/common"
import {useI18n} from "@/i18n/i18n"

const { t } = useI18n()

const props = defineProps<{
    characterName: CharacterName,
    modelValue: number
}>()

interface Emits {
    (e: "update:modelValue", v: number): void
}

const emits = defineEmits<Emits>()

const options = computed(() => {
    const data = characterData[props.characterName]
    const options = []
    for (let i = 0; i < 3; i++) {
        const j = i + 1
        const skillMap = data[`skillMap${j}`]
        if (skillMap.length > 0) {
            options.push({
                label: t("characterSkill", props.characterName, i),
                value: data[`skillName${j}`],
                children: skillMap.map((item: any) => ({
                    label: t('dmgName', item.text),
                    value: item.index,
                })),
            })
        }
    }
    return options
})
</script>
