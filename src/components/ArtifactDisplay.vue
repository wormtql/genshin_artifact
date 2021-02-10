<template>
    <div class="artifact">
        <div class="up">
            <span class="name">{{ displayedTitle }}</span>
        </div>
        <div class="down">
            <div>
                <img :src="imageSrc" class="myimage">
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

function displayedTitle(setName, position) {
    let item = artifactsData[setName];
    if (!item) {
        throw "no artifact";
    }

    if (item[position]) {
        return item[position].chs;
    }

    throw "error no position";
}

function imageSrc(setName, position) {
    let item = artifactsData[setName];
    if (!item) {
        throw "no artifact";
    }

    if (item[position]) {
        return item[position].url;
    }

    throw "error no position";
}

export default {
    name: "ArtifactDisplay",
    created: function () {
        let setName = this.item.setName;
        let position = this.item.position;
        this.displayedTitle = displayedTitle(setName, position);
        this.imageSrc = imageSrc(setName, position);

        this.mainDisplayTag = displayedTag(this.item.mainTag.name, this.item.mainTag.value);
        let secTags = [];
        for (let tag of this.item.normalTags) {
            secTags.push(displayedTag(tag.name, tag.value));
        }
        this.secTags = secTags;
    },
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
    },
}
</script>

<style scoped>
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
    display: inline-block;
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

.myimage {
    height: 48px;
    width: 48px;
    border-radius: 50%;
    display: block;
    margin-right: 8px;
}
</style>