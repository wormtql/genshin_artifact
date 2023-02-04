<template>
    <div class="item br-3" @click="$emit('click')">
        <div class="header">
            <span class="fs-12">{{ props.name }}</span>
            <div v-if="toolbar" class="buttons flex-row">
<!--                <el-button-->
<!--                    v-if="props.calculateIcon"-->
<!--                    :icon="IconEpCPU"-->
<!--                    text-->
<!--                    size="small"-->
<!--                    circle-->
<!--                    @click.stop="$emit('cpu')"-->
<!--                    class="button"-->
<!--                    title="快速计算"-->
<!--                ></el-button>-->
                <el-button
                    :icon="IconEpDelete"
                    text
                    size="small"
                    circle
                    @click.stop="$emit('delete')"
                    class="button"
                    :title="t('misc.del')"
                ></el-button>
                <el-button
                    :icon="IconFa6SolidDownload"
                    text
                    size="small"
                    circle
                    @click.stop="$emit('download')"
                    class="button"
                    :title="t('misc.export')"
                ></el-button>
            </div>
        </div>
        <div class="body">
            <div class="detail-div fs-12">
                <img :src="characterAvatar" class="c-avatar br-50p">
<!--                <span>{{ characterChs }}</span>-->
                <span>{{ t("a", characterLocaleIndex) }}</span>
            </div>
            <div class="detail-div fs-12">
                <img :src="wData.url" class="w-avatar br-50p">
<!--                <span>{{ wData.chs }}</span>-->
                <span>{{ t("weapon", props.item.weapon.name) }}</span>
            </div>
            <div class="detail-div fs-12">
                <img :src="tfData.badge" class="tf-avatar br-50p">
<!--                <span>{{ tfData.chs }}</span>-->
                <span>{{ t("tfName", tfData.name) }}</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { characterData } from "@character"
import { weaponData } from "@weapon"
import { targetFunctionData } from "@targetFunction"

import IconFa6SolidDownload from "~icons/fa6-solid/download"
import IconEpDelete from "~icons/ep/delete"
import IconEpCPU from "~icons/ep/cpu"
import {IPreset} from "@/types/preset"
import {useI18n} from "@/i18n/i18n";


const { t } = useI18n()

interface Props {
    item: IPreset,
    name: string,
    toolbar?: boolean,
    calculateIcon?: boolean,
}

const props = withDefaults(defineProps<Props>(), {
    toolbar: true,
    calculateIcon: true,
    name: "name",
})

const characterName = computed(() => {
    return props.item.character.name
})

const characterLocaleIndex = computed(() => {
    return characterData[characterName.value].nameLocale
})

const characterAvatar = computed(() => {
    const data = characterData[characterName.value]
    return data.avatar
})

const characterChs = computed(() => {
    const data = characterData[characterName.value]
    return data.chs
})

// const cData = computed(() => {
//     return characterData[characterName]
// })

const wData = computed(() => {
    return weaponData[props.item.weapon.name]
})

const tfData = computed(() => {
    return targetFunctionData[props.item.targetFunction.name]
})
</script>

<style lang="scss" scoped>
.item {
    border: 1px solid #00000011;
    display: inline-block;
    transition: 300ms;

    &:hover {
        background-color: #00000008;
        cursor: pointer;
    }

    .header {
        height: 32px;
        border-bottom: 1px solid #00000011;

        span {
            line-height: 32px;
            padding: 4px 8px;
            color: #123456;
        }

        .buttons {
            float: right;
            height: 32px;

            .button {
                margin: 0;
            }
        }
    }

    .body {
        padding: 8px;
        //display: flex;
        display: grid;
        gap: 4px;
        grid-template-columns: repeat(3, 1fr);

        .detail-div {
            display: flex;
            flex-direction: column;
            align-items: center;
            text-align: center;

            span {
                padding-top: 8px;
            }
        }
    }
    
}

.c-avatar, .w-avatar, .tf-avatar {
    display: block;
    height: 64px;
    width: 64px;
    border: 1px solid #e9e9e9;
}
</style>