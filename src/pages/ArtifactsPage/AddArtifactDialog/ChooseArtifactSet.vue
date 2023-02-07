<template>
    <div class="choose-artifact-set">
        <div
            v-for="(artData) in artifactsData"
            :key="artData.eng"
            :class="{active: artData.eng === props.modelValue}"
            class="item"
            @click="emits('update:modelValue', artData.eng)"
        >
            <el-image
                class="image"
                :src="getArtifactThumbnailURL(artData.eng)"
            >
            </el-image>
            <p class="text">
                {{ ta(artData.nameLocale) }}
            </p>
        </div>
        
        
    </div>
</template>

<script setup lang="ts">
import { artifactsData } from "../../../assets/artifacts"
import { getArtifactThumbnailURL } from "@/utils/utils"
import type {ArtifactSetName} from "@/types/artifact"
import {useI18n} from "@/i18n/i18n"

const { t, ta } = useI18n()

interface Props {
    modelValue: ArtifactSetName
}

const props = withDefaults(defineProps<Props>(), {
    modelValue: "archaicPetra"
})

interface Emits {
    (e: "update:modelValue", v: ArtifactSetName): void
}

const emits = defineEmits<Emits>()
</script>

<style lang="scss" scoped>
.image {
    width: 48px;
    height: 48px;
    // border-radius: 50%;
    // box-shadow: 0 0 10px 5px #00000022;
}

.text {
    font-size: 12px;
    padding: 0;
    margin: 0;
    text-align: center;
    width: 48px;
    word-break: normal;
    //white-space: break-spaces;
}

.choose-artifact-set {
    display: flex;
    flex-wrap: wrap;
}

.item {
    margin-right: 4px;
    padding: 8px;
    color: #777777;

    &:hover {
        background: rgba(0, 0, 0, 0.1);
        cursor: pointer;
        border-radius: 3px;
        transition: 300ms;
    }

    &.active {
        /* background: rgba(0, 0, 0, 0.1); */
        border-radius: 3px;
        /* color: white; */
        background: none;
        box-shadow: 0 0 20px 1px #00000033;
    }
}
</style>