<template>
    <div class="choose-artifact-position">
        <div
            v-for="position in availablePositions"
            :key="position"
            :class="{active: position === props.modelValue}"
            class="item"
            @click="emits('update:modelValue', position)"
        >
            <img
                :src="selectedArtData[position].url"
                class="image"
            >
            <p class="text">
                {{ ta(selectedArtData[position].text) }}
            </p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, watch } from "vue"

import { artifactsData } from "../../../assets/artifacts"
import type {ArtifactPosition, ArtifactSetName} from "@/types/artifact"
import { positions } from "@/constants/artifact"
import {useI18n} from "@/i18n/i18n"
import {positionToIndex} from "@/utils/artifacts"


const { t, ta } = useI18n()


interface Props {
    modelValue: ArtifactPosition,
    setName: ArtifactSetName
}

const props = withDefaults(defineProps<Props>(), {
    modelValue: "flower",
    setName: "archaicPetra"
})

interface Emits {
    (e: "update:modelValue", v: ArtifactPosition): void
}

const emits = defineEmits<Emits>()

const selectedArtData = computed(() => {
    return (artifactsData as any)[props.setName]
})

const availablePositions = computed((): ArtifactPosition[] => {
    return positions.filter(value => {
        return Object.prototype.hasOwnProperty.call(selectedArtData.value, value)
    })
})

watch(() => props.setName, () => {
    if (availablePositions.value.indexOf(props.modelValue) === -1) {
        const autoSelectedPosition = availablePositions.value[0]
        emits("update:modelValue", autoSelectedPosition)
    }
})
</script>

<style scoped>
.choose-artifact-position {
    display: flex;
    flex-wrap: wrap;
}

.item {
    padding: 8px;
    color: #777777;
    margin-right: 4px;
}

.item:hover {
    background: rgba(0, 0, 0, 0.1);
    cursor: pointer;
    border-radius: 3px;
    transition: 300ms;
}

.item.active {
    /* background: rgba(0, 0, 0, 0.1); */
    border-radius: 3px;
    /* color: white; */
    background: none;
    box-shadow: 0 0 20px 1px #00000033;
}

.text {
    padding: 0;
    margin: 0;
    font-size: 12px;
    text-align: center;
    width: 48px;
    word-break: normal;
}

.image {
    height: 48px;
    width: 48px;
    border-radius: 50%;
}
</style>