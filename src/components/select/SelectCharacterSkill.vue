<template>
<!-- use :key to force rerender, or there will be a bug   -->
    <el-select
        :model-value="props.modelValue"
        @update:modelValue="emits('update:modelValue', $event)"
        :key="props.characterName"
    >
        <template #prefix>
            <span>{{ t("misc.skill") }}</span>
        </template>

        <el-option-group
            v-for="(group, index) in skillMap"
            :key="group[0]"
            :label="group[0]"
        >
            <el-option
                v-for="item in group[1]"
                :key="item.index.toString()"
                :label="ta(item.text)"
                :value="item.index"
            >
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script setup lang="ts">
import { characterData } from "@character"
import type {CharacterName} from "@/types/character"
import {deepCopy} from "@/utils/common"
import {useI18n} from "@/i18n/i18n"


const { t, ta } = useI18n()

interface Props {
    characterName: CharacterName,
    modelValue: number
}

const props = defineProps<Props>()

interface Emits {
    (e: "update:modelValue", v: number): void
}

const emits = defineEmits<Emits>()

const skillMap = computed(() => {
    const data = characterData[props.characterName]

    let map: [string, { index: number, text: number }[]][] = []
    if (data.skillMap1.length > 0) {
        map.push([
            // data.skillName1, data.skillMap1
            ta(data.skillName1), data.skillMap1
        ])
    }
    if (data.skillMap2.length > 0) {
        map.push([
            // data.skillName2, data.skillMap2
            ta(data.skillName2), data.skillMap2
        ])
    }
    if (data.skillMap3.length > 0) {
        map.push([
            // data.skillName3, data.skillMap3
            ta(data.skillName3), data.skillMap3
        ])
    }

    return map
})
</script>