<template>
    <div class="buff-root">
        <div class="top" :class="{ lock }">
            <p class="buff-title">{{ ta(data.nameLocale) }}</p>
            <div class="buttons">
                <el-button
                    text
                    circle
                    class="button"
                    :icon="IconEpDelete"
                    @click="emits('delete')"
                    type="primary"
                ></el-button>
                <el-button
                    text
                    circle
                    class="button"
                    :icon="lock ? IconEpUnlock : IconEpLock"
                    @click="emits('toggle')"
                    type="primary"
                ></el-button>
            </div>
        </div>

        <div class="detail">
            <img :src="data.badge">
            <div>
                <p v-if="description"><span v-html="description"></span></p>
            </div>
        </div>
        
        <item-config
            v-if="data.config.length > 0"
            :model-value="props.buffConfig"
            @update:modelValue="handleChangeConfig"
            :item-name="props.buff.name"
            :configs="data.config"
        ></item-config>
    </div>
</template>

<script setup lang="ts">
import { buffData } from "@buff"

import ItemConfig from "@c/config/ItemConfig"
import {type BuffEntry} from "@/composables/buff"

import IconEpDelete from "~icons/ep/delete"
import IconEpLock from "~icons/ep/lock"
import IconEpUnlock from "~icons/ep/unlock"
import {useI18n} from "@/i18n/i18n";

const { t, ta } = useI18n()

interface Props {
    buff: BuffEntry,
    buffConfig: any
}

const props = defineProps<Props>()

interface Emits {
    (e: "update:buffConfig", v: any): void,
    (e: "delete"): void,
    (e: "toggle"): void,
}

const emits = defineEmits<Emits>()

function handleChangeConfig(v: any) {
    emits("update:buffConfig", v)
}

const data = computed((): any => {
    return buffData[props.buff.name]
})

const lock = computed(() => {
    return props.buff.lock
})

const description = computed(() => {
    return data.value.description ? ta(data.value.description) : null
})
</script>

<style lang="scss" scoped>
.buff-root {
    //background-color: rgb(239, 246, 253);
    margin-bottom: 12px;

    .top {
        border-bottom: 1px solid #DCDFE6;
        padding: 0 12px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        transition-duration: 200ms;

        &.lock {
            // background-image: linear-gradient(45deg, #ff5272 25%, #ffffff 25%, #ffffff 50%, #ff5272 50%, #ff5272 75%, #ffffff 75%, #ffffff 100%);
            background: linear-gradient(45deg, #ffd4e2 25%, #fff 25%, #fff 50%, #ffd4e2 50%, #ffd4e2 75%, #fff 75%, #fff 100%);
            background-size: 40px 40px;
        }

        .buff-title {
            font-weight: bold;
            font-size: 14px;
            margin: 0px;
            color: #666666;
            margin-right: 24px;
        }

        .buttons {
            display: flex;
            //.button {
            //    padding: 0;
            //}
        }
    }

    .detail {
        display: flex;
        padding: 8px;

        img {
            height: 64px;
            width: 64px;
            border-radius: 50%;
            margin-right: 12px;
        }

        p {
            margin: 0;
            font-size: 12px;
            color: #606266;
        }
    }
}
</style>