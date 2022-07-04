<template>
    <el-dialog
        :title="t('artPage.newArt')"
        :model-value="props.modelValue"
        @update:model-value="$emit('update:modelValue', $event)"
        :width="deviceIsPC ? '80%' : '90%'"
    >
        <h3 class="item-title">{{ t("misc.artSet") }}</h3>
        <choose-artifact-set
            :model-value="setName"
            @update:modelValue="handleSetNameChange"
            class="hidden-sm-and-down"
        ></choose-artifact-set>
        <select-artifact-set
            :model-value="setName"
            @update:modelValue="handleSetNameChange"
            class="hidden-md-and-up"
        ></select-artifact-set>

        <h3 class="item-title">{{ t("misc.artSlot") }}</h3>
        <choose-artifact-position
            v-model="position"
            :setName="setName"
            class="hidden-sm-and-down"
        ></choose-artifact-position>
        <select-artifact-slot v-model="position" class="hidden-md-and-up"></select-artifact-slot>

        <el-row :gutter="16">
            <el-col :sm="24" :md="12">
                <h3 class="item-title">{{ t("misc.quality") }}</h3>
                <el-rate
                    @update:modelValue="handleStarChange"
                    :model-value="star"
                    :max="artifactData.maxStar"
                ></el-rate>
            </el-col>
            <el-col :sm="24" :md="12">
                <h3 style="margin-right: 8px" class="item-title">{{ t("misc.lvl") }}</h3>

                <el-slider
                    v-model="level"
                    :max="star * 4"
                    :min="0"
                    :step="1"
                    show-input
                ></el-slider>
                
            </el-col>
        </el-row>

        <el-row :gutter="16">
            <el-col :sm="24" :md="12">
                <h3 class="item-title">{{ t("misc.mainStat") }}</h3>
                <!-- <choose-main-tag v-model="mainTag" :position="position"></choose-main-tag> -->
                <select-artifact-main-tag
                    :model-value="mainTag"
                    :position="position"
                    @update:modelValue="handleMainTagChange"
                ></select-artifact-main-tag>
            </el-col>
            <el-col :sm="24" :md="12">
                <div class="flex-row">
                    <h3 style="margin-right: 8px" class="item-title">{{ t("misc.subStat") }}</h3>
                    <el-button
                        :icon="IconEpRefresh"
                        circle
                        size="small"
                        @click="shuffleNormalTags"
                        :title="t('misc.random')"
                        text
                    ></el-button>
<!--                    <el-button-->
<!--                        icon="el-icon-question"-->
<!--                        circle-->
<!--                        style="padding: 0"-->
<!--                        title="帮助"-->
<!--                        type="text"-->
<!--                        @click="openHelp"-->
<!--                    ></el-button>-->
                </div>
                
                <select-artifact-normal-tag v-model="normalTags"></select-artifact-normal-tag>
            </el-col>
        </el-row>

        <template #footer>
            <el-button @click="emits('update:modelValue', false)">{{ t("misc.cancel") }}</el-button>
            <el-button type="primary" @click="onConfirm">{{ t("misc.confirm") }}</el-button>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { type Ref } from "vue"

import ChooseArtifactSet from "./ChooseArtifactSet"
import ChooseArtifactPosition from "./ChooseArtifactPosition"
import SelectArtifactNormalTag from "./SelectArtifactNormalTag"
import SelectArtifactMainTag from "./SelectArtifactMainTag"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectArtifactSlot from "@c/select/SelectArtifactSlot"

import { getDetailName, getArtifactRealValue } from "@util/utils"
import randomNormalTag from "@/artifacts_numeric/random_normal_tag"
import { convertDisplayTagValue } from '@util/utils'
import { artifactsData } from "@asset/artifacts"
import { artifactTags } from "@const/artifact"
import type {ArtifactPosition, ArtifactSetName, IArtifactContentOnly, IArtifactTag} from "@/types/artifact"
import { isArtifactExists } from "@/utils/artifacts"
import {deviceIsPC} from "@/utils/device"

import IconEpRefresh from "~icons/ep/refresh"
import {useI18n} from "@/i18n/i18n";

/// #if !USE_CDN
// import {ElMessageBox} from "element-plus"
/// #endif

const { t } = useI18n()


interface Props {
    modelValue: boolean
}

const props = withDefaults(defineProps<Props>(), {
    modelValue: false
})


interface Emits {
    (e: "update:modelValue", v: boolean): void,
    (e: "confirm", v: IArtifactContentOnly): void
}
const emits = defineEmits<Emits>()

function convertPercentage(item: IArtifactTag) {
    item.value = getArtifactRealValue(item.name, item.value);

    return item;
}

const setName: Ref<ArtifactSetName> = ref("archaicPetra")
const position: Ref<ArtifactPosition> = ref("flower")

interface IArtifactTagString {
    name: string,
    value: string
}

const mainTag: Ref<IArtifactTagString> = ref({
    name: "lifeStatic",
    value: "4780"
})
const normalTags: Ref<IArtifactTagString[]> = ref([])

const star = ref(5)
const level = ref(20)

const artifactData = computed((): any => {
    return artifactsData[setName.value];
})

function handleStarChange(e: number) {
    if (e >= artifactData.value.minStar && e <= artifactData.value.maxStar) {
        star.value = e;
        level.value = star.value * 4;
    }
}

function handleMainTagChange(e: IArtifactTagString) {
    if (e.name !== mainTag.value.name) {
        // let maxValue = secondaryTags[e.name].max[5];
        const maxValue = artifactTags[e.name].max[5]

        mainTag.value.value = convertDisplayTagValue(e.name, maxValue);
        mainTag.value.name = e.name;
    } else {
        mainTag.value = e;
    }
}

function handleSetNameChange(e: ArtifactSetName) {
    setName.value = e;
    star.value = artifactData.value.maxStar;
    level.value = star.value * 4;
}

function getArtifact(): IArtifactContentOnly {
    return {
        setName: setName.value,
        position: position.value,
        mainTag: getArtifactMainTag(),
        normalTags: getArtifactNormalTags(),
        star: star.value,
        level: level.value,
    }
}

function getArtifactMainTag(): IArtifactTag {
    let temp: IArtifactTag = {
        name: mainTag.value.name,
        value: parseFloat(mainTag.value.value)
    }

    if (isNaN(temp.value)) {
        temp.value = 0;
    }

    convertPercentage(temp);
    return temp
}

function getArtifactNormalTags(): IArtifactTag[] {
    let temp = [];
    for (let item of normalTags.value) {
        if (item.value === "" || item.value === "0") {
            continue;
        }

        let tag = {
            name: item.name,
            value: parseFloat(item.value)
        } as IArtifactTag
        convertPercentage(tag)
        temp.push(tag);
    }

    return temp;
}

function shuffleNormalTags() {
    let temp = randomNormalTag(5, 20, [mainTag.value.name]);
    for (let i = 0, l = temp.length; i < l; i++) {
        temp[i].value = convertDisplayTagValue(temp[i].name, temp[i].value);
    }

    normalTags.value = temp;
}

function onConfirm() {
    let result = getArtifact()

    if (isArtifactExists(result)) {
        ElMessageBox.confirm(
            t("artPage.dup"),
            t("misc.hint"),
            {
                confirmButtonText: t("misc.cont"),
                cancelButtonText: t("misc.cancel"),
                type: "warning",
            }
        ).then(() => {
            emits("confirm", result)
        }).catch(() => {})
    } else {
        emits("confirm", result)
    }
}


// export default {
//     provide() {
//         return {
//             star: this.star,
//         }
//     },
//     methods: {
//
//
//         openHelp() {
//             let text = "使用随机副词条，可以快速构建大量合理的圣遗物，可以方便地确定某个角色适合什么圣遗物。例如，快速构建角斗士、乐团、魔女4件套，从而计算胡桃适合什么样的圣遗物组合";
//             this.$alert(text, "帮助");
//         }
//     },
//     computed: {
//
//     }
// }
</script>

<style scoped lang="scss">
@media only screen and (max-width: 992px) {
    .el-select {
        width: 100%!important;
    }
}

.item-title {
    font-size: 14px;
}
</style>