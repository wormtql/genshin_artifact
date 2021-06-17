<template>
    <div
        class="artifact"
        :class="{ selectable, omit: item.omit }"
        @click="handleClick"
        :style="artifactStyle"
    >
        <div class="up">
            <span class="name">
                {{ displayedTitle }}
                <!-- <span style="color: #e7bf4f">
                    <i class="el-icon-star-on"></i>{{ displayedStar }}
                </span> -->
            </span>
            <span class="extra fs-12" v-if="extra">{{ extra }}</span>

            <div class="buttons" v-if="buttons">
                <el-button
                    v-if="lockButton"
                    :icon="item.omit ? 'el-icon-unlock' : 'el-icon-lock'"
                    circle
                    size="mini"
                    type="text"
                    :title="item.omit ? '启用' : '禁用'"
                    class="mybutton"
                    @click="$emit('toggle')"
                ></el-button>
                <el-button
                    v-if="deleteButton"
                    icon="el-icon-delete"
                    circle
                    size="mini"
                    type="text"
                    title="删除"
                    class="mybutton"
                    @click.stop="$emit('delete')"
                ></el-button>
            </div>
        </div>
        <div class="down">
            <div>
                <img
                    :src="imageSrc"
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

<script>
import { displayedTag } from "@util/utils";
import { artifactsData } from "@asset/artifacts";

import colors from "@const/quality_colors";

export default {
    name: "ArtifactDisplay",
    props: {
        item: {
            type: Object,
            default: () => ({
                setName: "luckyDog",
                position: "cup",
                detailName: "worm666",
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
                omit: false,
            })
        },

        selectable: {
            type: Boolean,
            default: false,
        },

        extra: {
            type: String,
            default: "",
        },

        buttons: {
            type: Boolean,
            default: false,
        },

        deleteButton: {
            type: Boolean,
            default: false,
        },

        lockButton: {
            type: Boolean,
            default: true,
        },

        width: {
            type: String,
            default: "unset",
        }
    },
    methods: {
        handleClick() {
            if (this.selectable) {
                this.$emit("click");
            }
        }
    },
    computed: {
        displayedTitle() {
            let item = artifactsData[this.item.setName];
            if (!item) {
                throw "no artifact";
            }

            let title = "not exist"
            if (item[this.item.position]) {
                title = item[this.item.position].chs;
                if (Object.prototype.hasOwnProperty.call(this.item, "level")) {
                    title += "+" + (this.item.level);
                } else {
                    title += "+??";
                }
            }
            

            return title;
        },

        displayedStar() {
            return this.item.star || "??";
        },

        imageSrc() {
            let item = artifactsData[this.item.setName];
            if (!item) {
                throw "no artifact";
            }

            if (item[this.item.position]) {
                return item[this.item.position].url;
            }

            throw "error no position";
        },

        mainDisplayTag() {
            return displayedTag(this.item.mainTag.name, this.item.mainTag.value);
        },

        secTags() {
            let temp = [];
            for (let tag of this.item.normalTags) {
                temp.push(displayedTag(tag.name, tag.value));
            }
            return temp;
        },

        imageBackground() {
            let star = this.item.star ?? 5;
            return colors[star - 1];
        },

        artifactStyle() {
            return {
                width: this.width,
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.artifact {
    /* border: 1px solid #cccccc; */
    box-shadow: 0 0 10px 1px #00000011;
    transition: 300ms;
    display: inline-block;
    // width: 200px;

    &.selectable:hover {
        background: #12345622;
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

        .name {
            color: #123456;
            font-size: 12px;
            float: left;
            height: 32px;
            line-height: 32px;
        }

        .extra {
            color: #e7bf4f;
            line-height: 32px;
            margin-left: 16px;
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
        align-items: top;

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