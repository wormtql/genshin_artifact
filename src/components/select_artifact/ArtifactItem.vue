<template>
    <el-card :style="cardStyle" shadow="never" class="art" @click.native="$emit('click')">
        <div slot="header" class="header">
            <span>{{ item.detailName }}</span>
        </div>
        
        <el-row :gutter="16">
            <el-col :span="10">
                <el-image
                    :src="`${publicPath}detail/${item.setName}/${item.position}.png`"
                    style="width: 70px; height: 70px"
                ></el-image>
            </el-col>
            <el-col :span="14">
                <h3 class="primaryTag">{{ `${chsSecondaryTag(item.primary.tag)}: ${item.primary.value}` }}</h3>

                <el-tag
                    size="mini"
                    class="secondaryTag"
                    v-for="(tag, index) in item.secondary"
                    :key="index"
                >
                    {{ `${chsSecondaryTag(tag.tag)}: ${tag.value}` }}
                </el-tag>
            </el-col>
        </el-row>
        
    </el-card>
</template>

<script>
import { chsSecondaryTag } from "@/common/chs";

export default {
    name: "ArtifactItem",
    props: {
        selected: {
            type: Boolean,
            default: false,
        },
        item: {
            type: Object,
            default: () => ({
                setName: "luckyDog",
                position: "cup",
                detailName: "冒险家的金杯",
                primary: {
                    tag: "attack1",
                    value: 100,
                },
                secondary: [
                    { tag: "defend1", value: 20, },
                    { tag: "attack2", value: 30 },
                ],
                omit: false,
            })
        },
    },
    data: function() {
        return {
            publicPath: process.env.BASE_URL,
        }
    },
    methods: {
        chsSecondaryTag,
    },
    computed: {
        cardStyle: function() {
            return {
                width: "250px",
                // height: "165px",
                background: this.selected ? "rgb(217, 236, 255)" : "unset",
            };
        }
    }
}
</script>

<style scoped>
.art {
    cursor: pointer;
}

.primaryTag {
    padding: 0;
    margin: 0;
    font-size: 0.9rem;
}

.secondaryTag {
    margin: 0 4px 2px 0;
    /* padding: 0; */
    /* margin: 0; */
    /* font-size: 0.75rem; */
}
</style>