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


<!--        {{ p(skillMap) }}-->
        <el-option-group
            v-for="(group, index) in skillMap"
            :key="group[0]"
            :label="group[0]"
        >
<!--            {{ p(group[1]) }}-->
            <el-option
                v-for="item in group[1]"
                :key="item.index.toString()"
                :label="t('dmgName', item.text)"
                :value="item.index"
            >
<!--                {{ item.chs }}-{{ item.index }}-->
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script setup lang="ts">
import { characterData } from "@character"
import type {CharacterName} from "@/types/character"
import {deepCopy} from "@/utils/common"
import {useI18n} from "@/i18n/i18n";


const { t } = useI18n()

interface Props {
    characterName: CharacterName,
    modelValue: number
}

const props = defineProps<Props>()

interface Emits {
    (e: "update:modelValue", v: number): void
}

const emits = defineEmits<Emits>()

// watch(() => props.characterName, name => {
//     console.log("name changed", name)
// })

const skillMap = computed(() => {
    const data = characterData[props.characterName]

    let map: [string, { index: number, text: number }[]][] = []
    if (data.skillMap1.length > 0) {
        map.push([
            // data.skillName1, data.skillMap1
            t("characterSkill", props.characterName, 0), data.skillMap1
        ])
    }
    if (data.skillMap2.length > 0) {
        map.push([
            // data.skillName2, data.skillMap2
            t("characterSkill", props.characterName, 1), data.skillMap2
        ])
    }
    if (data.skillMap3.length > 0) {
        map.push([
            // data.skillName3, data.skillMap3
            t("characterSkill", props.characterName, 2), data.skillMap3
        ])
    }
    // console.log(map)

    // return deepCopy(map)
    return map
})

function p(x: any) {
    console.log(x)
}
</script>