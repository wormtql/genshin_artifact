<template>
    <el-select
        :model-value="props.modelValue"
        @update:modelValue="emits('update:modelValue', $event)"
        :key="props.characterName"
    >
        <template #prefix>
            <span>技能</span>
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
                :label="item.chs"
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

    let map: [string, { index: number, chs: string }[]][] = []
    if (data.skillMap1.length > 0) {
        map.push([
            data.skillName1, data.skillMap1
        ])
    }
    if (data.skillMap2.length > 0) {
        map.push([
            data.skillName2, data.skillMap2
        ])
    }
    if (data.skillMap3.length > 0) {
        map.push([
            data.skillName3, data.skillMap3
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