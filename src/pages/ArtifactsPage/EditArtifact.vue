<template>
    <div>
        <div class="section title">
            <img :src="image" class="title-image">
            <span class="title-name">{{ title }}</span>
        </div>

        <el-divider></el-divider>

        <div style="padding: 0 20px">
            <el-row :gutter="16">
                <el-col :span="12">
                    <p class="config-title">{{ t("misc.artifactSet") }}</p>
                    <select-artifact-set
                        v-model="setName"
                    ></select-artifact-set>
                </el-col>
                <el-col :span="12">
                    <p class="config-title">{{ t("misc.artSlot") }}</p>
                    <select-artifact-slot
                        :model-value="position"
                        @update:modelValue="handleChangePosition"
                    ></select-artifact-slot>
                </el-col>
            </el-row>

            <el-row :gutter="16">
                <el-col :span="12">
                    <p class="config-title">{{ t("misc.quality") }}</p>
                    <el-rate
                        v-model="star"
                    ></el-rate>
                </el-col>
                <el-col :span="12">
                    <p class="config-title">{{ t("misc.lvl") }}</p>
                    <el-input-number
                        v-model="level"
                        :max="20"
                        :min="0"
                    ></el-input-number>
                </el-col>
            </el-row>
        </div>

        <el-divider></el-divider>

        <div class="section">
            <p class="config-title">{{ t("misc.stat") }}</p>

            <input-artifact-main-stat v-model="mainStat"
                style="margin-bottom: 24px"
            ></input-artifact-main-stat>


            <div class="sub-stat-div">
                <div
                    v-for="(sub, index) in subStats"
                    :key="index"
                    class="sub-stat-item"
                >
                    <input-artifact-sub-stat
                        v-model="subStats[index]"
                    ></input-artifact-sub-stat>

                    <el-button
                        :icon="IconEpDelete"
                        type="danger"
                        @click="handleRemoveSubStat(index)"
                    ></el-button>
                </div>
            </div>
        </div>

        <el-divider></el-divider>

        <div class="section">
            <el-row :gutter="12">
                <el-col :span="12">
                    <el-button type="primary" class="button"
                        @click="emits('confirm', artifactId)"
                    >
                        {{ t("misc.confirm") }}
                    </el-button>
                </el-col>
                <el-col :span="12">
                    <el-button class="button"
                               @click="emits('cancel')"
                    >
                        {{ t("misc.cancel") }}
                    </el-button>
                </el-col>
            </el-row>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue"

import {getArtifact, getArtifactImage, positionToIndex} from "@/utils/artifacts"
import { artifactsData } from "@artifact"
import { artifactTags, mainStatMap } from "@/constants/artifact"
import { positions } from "@/constants/artifact"

import InputArtifactSubStat from "@/components/input/InputArtifactSubStat.vue"
import InputArtifactMainStat from "@c/input/InputArtifactMainStat"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectArtifactSlot from "@c/select/SelectArtifactSlot"
import type {
    ArtifactPosition,
    ArtifactSetName,
    IArtifactTag,
    ArtifactSubStatName,
    ArtifactMainStat
} from "@/types/artifact"
import type { ArtifactStatName } from "@/types/artifact"

import IconEpDelete from "~icons/ep/delete"
import {useI18n} from "@/i18n/i18n";


const { t, ta } = useI18n()

interface Emits {
    (e: "update:modelValue", v: any): void,
    (e: "confirm", id: number): void,
    (e: "cancel"): void
}

const emits = defineEmits<Emits>()


const artifactId = ref(-1)

const setName = ref<ArtifactSetName>("berserker")
const star = ref(5)
const level = ref(20)
const position = ref<ArtifactPosition>("flower")
const mainStat = ref<any>({ name: "attackStatic", value: 0 })

interface ArtifactStatNullable {
    name: null | ArtifactSubStatName,
    value: number
}
const subStats = ref<ArtifactStatNullable[]>([
    { name: null, value: 0 },
    { name: null, value: 0 },
    { name: null, value: 0 },
    { name: null, value: 0 },
])

function convertStat(stat: IArtifactTag) {
    const data = artifactTags[stat.name]
    if (data.percentage) {
        // return { name: stat.name, value: stat.value * 100 }
        return { name: stat.name, value: (stat.value * 100).toFixed(1) }
    } else {
        return { name: stat.name, value: stat.value }
    }
}

function handleChangePosition(p: ArtifactPosition) {
    position.value = p

    const mainStatList = mainStatMap[p]
    if (mainStatList.indexOf(mainStat.value.name) < 0) {
        const newMainStatName = mainStatList[0]
        const newMainStatValue = artifactTags[newMainStatName].max["5"]

        mainStat.value = convertStat({
            name: newMainStatName,
            value: newMainStatValue
        })
    }
}

function handleRemoveSubStat(index: number) {
    subStats.value[index] = { name: null, value: 0 }
}

function convertStatBack(stat: any) {
    if (!stat.name || !stat.value) {
        return null
    }

    const data = artifactTags[stat.name as ArtifactStatName]
    let value = parseFloat(stat.value)
    if (data.percentage) {
        value /= 100
    }
    if (isNaN(value)) {
        return null
    }

    return { name: stat.name, value }
}


const image = computed(() => {
    return getArtifactImage(setName.value, position.value)
})

const title = computed(() => {
    const data = artifactsData[setName.value]
    return ta(data[position.value]?.text)
})

watch(() => setName.value, (newValue, oldValue) => {
    if (newValue === oldValue) {
        return
    }

    const data = artifactsData[newValue]
    if (!data[position.value]) {
        for (let p of positions) {
            if (data[p]) {
                position.value = p
                break
            }
        }
    }
})


function setId(id: number) {
    const artifact = getArtifact(id)

    if (artifact) {
        artifactId.value = artifact.id
        setName.value = artifact.setName
        star.value = artifact.star
        level.value = artifact.level
        position.value = artifact.position
        mainStat.value = convertStat(artifact.mainTag)

        let ss: any = []
        for (let stat of artifact.normalTags) {
            ss.push(convertStat(stat))
        }
        while (ss.length < 4) {
            ss.push({ name: null, value: 0 })
        }
        subStats.value = ss
    }
}

function getNewArtifact() {
    const ms = convertStatBack(mainStat.value)
    let ss = []
    for (let stat of subStats.value) {
        const convertResult = convertStatBack(stat)
        if (convertResult) {
            ss.push(convertResult)
        }
    }

    return {
        setName: setName.value,
        star: star.value,
        level: level.value,
        position: position.value,
        mainTag: ms,
        normalTags: ss
    }
}

defineExpose({
    setId,
    getNewArtifact
})
</script>

<style scoped lang="scss">
.sub-stat-item {
    margin-bottom: 8px;
    display: flex;
    align-items: center;
    gap: 18px;

    &:last-of-type {
        margin-bottom: 0;
    }
}

.title {
    display: flex;
    align-items: center;
    justify-content: space-between;

    .title-image {
        width: 64px;
        height: 64px;
    }

    .title-name {
        font-size: 14px;
    }
}

.config-title {
    font-size: 12px;
}

.section {
    padding: 0 24px;
}

.button {
    width: 100%;
}
</style>