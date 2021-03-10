<template>
    <div class="artifact" :class="{omit: item.omit}">
        <div class="up">
            <span class="name">{{ displayedTitle }}</span>
            <div class="buttons">
                <el-button
                    icon="el-icon-delete"
                    circle
                    size="mini"
                    type="text"
                    title="删除"
                    class="mybutton"
                    @click="$emit('delete')"
                ></el-button>
                <el-button
                    :icon="item.omit ? 'el-icon-unlock' : 'el-icon-lock'"
                    circle
                    size="mini"
                    type="text"
                    :title="item.omit ? '启用' : '禁用'"
                    class="mybutton"
                    @click="$emit('toggle')"
                ></el-button>
                <el-button
                    icon="el-icon-edit"
                    circle
                    size="mini"
                    type="text"
                    title="编辑"
                    class="mybutton"
                    @click="$emit('edit')"
                ></el-button>
            </div>
        </div>
        <div class="down">
            <div>
                <img :src="imageSrc" class="myimage">
            </div>
            <div class="detail-div">
                <p class="main-tag">{{ mainDisplayTag }}</p>
                <p
                    v-for="(tag, index) in normalTags"
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


export default {
    name: "Artifact",
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
                star: 5,
                level: 20,
                omit: false,
            })
        },
    },
    computed: {
        displayedTitle() {
            let item = artifactsData[this.item.setName];
            let title = "not exist";
            if (item[this.item.position]) {
                title = item[this.item.position].chs;
                if (Object.prototype.hasOwnProperty.call(this.item, "level")) {
                    title += "+" + (this.item.level);
                } else {
                    title += "+??";
                }
            }
            title += `(${"*" + (this.item.star || "??")})`;
            return title;
        },

        imageSrc() {
            let item = artifactsData[this.item.setName];
            if (item[this.item.position]) {
                return item[this.item.position].url;
            }
            return ""; // todo
        },

        mainDisplayTag() {
            return displayedTag(this.item.mainTag.name, this.item.mainTag.value);
        },

        normalTags() {
            let temp = [];
            for (let tag of this.item.normalTags) {
                temp.push(displayedTag(tag.name, tag.value));
            }
            return temp;
        }
    }
}
</script>

<style scoped>
.omit {
    background-color: rgba(0, 0, 0, 0.1);
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

.artifact {
    /* border: 1px solid #cccccc; */
    box-shadow: 0 0 10px 1px #00000011;
    transition: 300ms;
}

.up {
    height: 32px;
    padding: 0 8px;
    border-bottom: 1px solid #e9e9e9;
}

.down {
    padding: 8px;
    display: flex;
    align-items: top;
}

.name {
    color: #123456;
    font-size: 12px;
    float: left;
    height: 32px;
    line-height: 32px;
}

.buttons {
    display: flex;
    align-items: center;
    height: 32px;
    float: right;
}

.buttons button {
    padding: 0;
    margin: 0;
    margin-left: 8px;
}

.myimage {
    height: 48px;
    width: 48px;
    border-radius: 50%;
    display: block;
    margin-right: 8px;
}
</style>