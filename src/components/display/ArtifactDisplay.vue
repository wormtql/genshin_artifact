<template>
    <div
        class="artifact"
        :class="{ selectable: props.selectable, omit: props.item.omit }"
        @click="handleClick"
    >
        <div class="up">
            <div
                v-if="props.showBack"
                :style="{ width: `${props.backValue * 100}%` }"
                class="back"
            ></div>

            <span class="name">
                {{ displayedTitle }}
            </span>

            <div class="buttons" v-if="props.buttons">
                <el-button
                    v-if="props.lockButton"
                    :icon="props.item.omit ? IconEpUnlock : IconEpLock"
                    circle
                    size="small"
                    text
                    :title="props.item.omit ? t('misc.unlock') : t('misc.lock')"
                    class="mybutton"
                    @click.stop="emits('toggle')"
                ></el-button>
                <el-button
                    v-if="props.deleteButton"
                    :icon="IconEpDelete"
                    circle
                    size="small"
                    text
                    :title="t('misc.del')"
                    class="mybutton"
                    @click.stop="emits('delete')"
                ></el-button>
                <el-button
                    v-if="props.editButton"
                    :icon="IconEpEdit"
                    circle
                    size="small"
                    text
                    :title="t('misc.edit')"
                    class="mybutton"
                    @click.stop="emits('edit')"
                ></el-button>
            </div>

            <span class="extra fs-12" v-if="props.extra">{{ props.extra }}</span>
        </div>
        <div class="down">
            <div>
                <img
                    :src="imageSrc"
                    alt="图"
                    class="myimage"
                    :style="{ background: imageBackground }"
                >
            </div>
            <div class="detail-div">
                <p class="main-tag">{{ mainDisplayTag }}</p>
                <p
                    v-for="(tag, index) in secTags"
                    :key="index"
                    class="sec-tag"
                >
                    {{ tag }}
                </p>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue"

import { artifactsData } from "@asset/artifacts"

import colors from "@const/quality_colors"

import IconEpUnlock from "~icons/ep/unlock"
import IconEpLock from "~icons/ep/lock"
import IconEpDelete from "~icons/ep/delete"
import IconEpEdit from "~icons/ep/edit"
import {IArtifact, IArtifactContentOnly} from "@/types/artifact"
import {displayedTag, positionToIndex} from "@/utils/artifacts"
import {useI18n} from "@/i18n/i18n"

// i18n
const { t, ta } = useI18n()

interface Props {
    item: Omit<IArtifact, "id" | "contentHash">,
    selectable?: boolean,
    extra?: string,
    buttons?: boolean,
    deleteButton?: boolean,
    lockButton?: boolean,
    editButton?: boolean,

    showBack?: boolean,
    backValue?: number
}

interface Emits {
    (e: "click"): void,
    (e: "toggle"): void,
    (e: "delete"): void,
    (e: "edit"): void,
}

const props = withDefaults(defineProps<Props>(), {
    item: defaultItem,
    selectable: false,
    extra: "",
    buttons: false,
    deleteButton: false,
    lockButton: true,
    editButton: false,
    showBack: false,
    backValue: 1
})

const emits = defineEmits<Emits>()

const displayedTitle = computed(() => {
    let item = artifactsData[props.item.setName];
    if (!item) {
        throw "no artifact";
    }

    let title = "not exist"
    if (item[props.item.position]) {
        title = ta(item[props.item.position].text)
        if (Object.prototype.hasOwnProperty.call(props.item, "level")) {
            title += "+" + (props.item.level);
        } else {
            title += "+??";
        }
    }

    return title;
})

// const displayedStar = computed(() => {
//     return props.item.star
// })

const imageSrc = computed(() => {
    let item = artifactsData[props.item.setName];
    if (!item) {
        throw "no artifact";
    }

    if (item[props.item.position]) {
        return item[props.item.position].url;
    }

    throw "error no position";
})

const mainDisplayTag = computed(() => {
    // console.log("123", props.item.mainTag.name)
    return displayedTag(props.item.mainTag.name, props.item.mainTag.value)
})

const secTags = computed(() => {
    let temp = [];
    for (let tag of props.item.normalTags) {
        // console.log("456", tag.name)
        temp.push(displayedTag(tag.name, tag.value))
    }
    return temp;
})

const imageBackground = computed(() => {
    let star = props.item.star
    return colors[star - 1]
})



function handleClick() {
    if (props.selectable) {
        emits("click")
    }
}
</script>

<script lang="ts">
import type {IArtifactContentOnly} from "@/types/artifact"
import {IArtifact} from "@/types/artifact";

function defaultItem(): Omit<IArtifact, "id" | "contentHash"> {
    return {
        setName: "luckyDog",
        position: "cup",
        mainTag: {
            name: "attackPercentage",
            value: 0.1,
        },
        normalTags: [
            { name: "defendStatic", value: 20, },
            { name: "attackPercentage", value: 0.3 },
            { name: "attackPercentage", value: 0.3 },
            { name: "attackPercentage", value: 0.3 },
        ],
        level: 20,
        star: 5,
        omit: false,
    }
}
</script>

<style lang="scss" scoped>
.artifact {
    /* border: 1px solid #cccccc; */
    //box-shadow: 0 0 10px 1px #00000011;
    // transition: 300ms;
    display: inline-block;
    border: 1px solid #00000011;
    // width: 200px;
    position: relative;

    &.selectable:hover {
        background: #00000009;
        /* border: 1px solid #123456; */
        cursor: pointer;
    }

    &.omit {
        background-color: rgba(0, 0, 0, 0.1);
    }

    .up {
        height: 32px;
        padding: 0 8px;
        border-bottom: 1px solid #e9e9e9;
        display: flex;
        justify-content: space-between;
        position: relative;

        .back {
            position: absolute;
            left: 0;
            top: 0;
            height: 100%;
            background-color: rgb(240, 249, 235);
            //z-index: -1;
        }

        .name {
            color: #123456;
            font-size: 12px;
            float: left;
            height: 32px;
            line-height: 32px;
            z-index: 10;
            text-overflow: ellipsis;
            white-space: nowrap;
            overflow: hidden;
            //max-width: 110px;
        }

        .extra {
            color: #e7bf4f;
            line-height: 32px;
            margin-left: 16px;
            float: right;
            z-index: 10;
        }

        .buttons {
            display: flex;
            align-items: center;
            height: 32px;
            float: right;

            button {
                padding: 0;
                margin: 0;
                margin-left: 8px;
            }
        }
    }

    .down {
        padding: 8px;
        display: flex;

        .myimage {
            height: 48px;
            width: 48px;
            border-radius: 50%;
            display: block;
            margin-right: 8px;
        }

        .sec-tag {
            font-size: 12px;
            margin: 0;
            padding: 0;
            color: #999999;
        }

        .main-tag {
            font-size: 12px;
            padding: 0;
            margin: 0;
            font-weight: bold;
        }
    }
}
</style>