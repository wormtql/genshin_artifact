<template>
    <div>
        <el-input
            v-model="searchString"
            style="margin-bottom: 16px"
            :placeholder="t('misc.search')"
            clearable
        >
            <template #append>
                <i-ep-search></i-ep-search>
            </template>
        </el-input>

        <div class="config-items mona-scroll">
            <div
                v-for="item in dataSearched"
                :key="item.name"
                class="item"
            >
                <div class="top">
                    <img :src="item.thumbnail" class="image" >
                    <div>
                        <h3 class="artifact-title">{{ item.title }}</h3>
                        <div>
                            <p v-if="hasConfig2(item)" style="font-size: 12px;">
                                <span class="effect-title">{{ t("misc.art2") }}</span>
                                <span class="effect-body">{{ item.effect2 }}</span>
                            </p>
                            <p v-if="hasConfig4(item)" style="font-size: 12px;">
                                <span class="effect-title">{{ t("misc.art4") }}</span>
                                <span class="effect-body">{{ item.effect4 }}</span>
                            </p>
                        </div>
                    </div>
                </div>

                <item-config
                    v-if="hasConfig2(item)"
                    :model-value="modelValue[item.snake]"
                    :configs="item.config2"
                    :need-item-name="false"
                    @update:modelValue="handleChangeValue(item.snake, $event)"
                    style="margin-bottom: 8px"
                ></item-config>
                <item-config
                    v-if="hasConfig4(item)"
                    :model-value="modelValue[item.snake]"
                    :configs="item.config4"
                    :need-item-name="false"
                    @update:modelValue="handleChangeValue(item.snake, $event)"
                ></item-config>
            </div>
        </div>
    </div>
</template>

<script>
import Fuse from "fuse.js"

import { artifactsData } from "@artifact"

import { toSnakeCase, deepCopy } from "@util/common"
import { getArtifactThumbnail } from "@util/artifacts"

import ItemConfig from "@/components/config/ItemConfig"
import {useI18n} from "@/i18n/i18n";

export default {
    name: "ArtifactConfig",
    components: {ItemConfig},
    props: ["modelValue"],
    emits: ["update:modelValue"],
    data() {
        return {
            searchString: ""
        }
    },
    computed: {
        data() {
            let results = []
            for (let name in artifactsData) {
                const d = artifactsData[name]
                const config4 = d.config4 ?? []
                const config2 = d.config2 ?? []
                const name2 = d.name2
                if (config4.length > 0 || config2.length > 0) {
                    results.push({
                        name: name2,
                        title: this.ta(d.nameLocale),
                        eng: d.eng,
                        snake: "config_" + toSnakeCase(name2),
                        config4: config4,
                        config2: config2,
                        effect4: this.ta(d.effect4),
                        effect2: this.ta(d.effect2),
                        thumbnail: getArtifactThumbnail(name),
                        // chs: d.chs,
                    })
                }
            }
            return results
        },

        dataSearched() {
            if (this.searchString === "") {
                return this.data
            } else {
                const fuse = new Fuse(this.data, {
                    keys: ["title", "effect4", "effect2"]
                })
                const results = fuse.search(this.searchString)
                return results.map(x => x.item)
            }
        },
    },
    methods: {
        handleChangeValue(snake, value) {
            // console.log(snake, value)
            let temp = deepCopy(this.modelValue)
            temp[snake] = value
            this.$emit("update:modelValue", temp)
        },

        hasConfig2(item) {
            return item.config2 && item.config2.length > 0
        },

        hasConfig4(item) {
            return item.config4 && item.config4.length > 0
        }
    },
    setup() {
        const { t, ta } = useI18n()

        return {
            t, ta
        }
    }
}
</script>

<style scoped lang="scss">
.config-items {
    max-height: 60vh;

    .item {
        margin-bottom: 24px;

        &:last-of-type {
            margin-bottom: 0;
        }

        .top {
            display: flex;
        }

        .image {
            width: 64px;
            height: 64px;
            margin-right: 12px;
            //right: 0;
            //bottom: 0;
        }

        .effect-title {
            color: #6eb7ff;
        }

        .effect-body {
            word-break: normal;
        }

        .artifact-title {
            font-size: 12px;
            margin: 8px 0 0;
        }
    }
}
</style>
