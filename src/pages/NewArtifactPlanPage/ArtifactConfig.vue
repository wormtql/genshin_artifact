<template>
    <div>
        <el-input
            size="medium"
            v-model="searchString"
            style="margin-bottom: 16px"
            placeholder="搜索"
            clearable
        >
            <template slot="append">
                <i class="el-icon-search"></i>
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
                        <h3 class="artifact-title">{{ item.chs }}</h3>
                        <p style="font-size: 12px;">
                            <span class="effect-title">四件套：</span>
                            <span>{{ item.effect4 }}</span>
                        </p>
                    </div>
                </div>

                <item-config
                    :value="value[item.snake]"
                    :configs="item.config4"
                    :need-item-name="false"
                    @input="handleChangeValue(item.snake, $event)"
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

import ItemConfig from "@c/config/ItemConfig"

export default {
    name: "ArtifactConfig",
    components: {ItemConfig},
    props: ["value"],
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
                const config = d.config4 ?? []
                const name2 = d.name2
                if (config.length > 0) {
                    results.push({
                        name: name2,
                        snake: "config_" + toSnakeCase(name2),
                        config4: config,
                        effect4: d.effect4,
                        thumbnail: getArtifactThumbnail(name),
                        chs: d.chs,
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
                    keys: ["chs", "effect4"]
                })
                const results = fuse.search(this.searchString)
                return results.map(x => x.item)
            }
        }
    },
    methods: {
        handleChangeValue(snake, value) {
            let temp = deepCopy(this.value)
            temp[snake] = value
            this.$emit("input", temp)
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

        .artifact-title {
            font-size: 12px;
            margin: 8px 0 0;
        }
    }
}
</style>