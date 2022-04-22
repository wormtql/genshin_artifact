<template>
    <div class="artifact-bar">
        <div v-if="isSet4" class="header-item">
            <img class="tn" :src="setThumbnails[0]">×4
        </div>
        <div v-else-if="isSet2" class="header-item">
            <img class="tn" :src="setThumbnails[0]">×2
        </div>
        <div v-else-if="isSet22" class="header-item">
            <img class="tn" :src="setThumbnails[0]">
            <img class="tn" :src="setThumbnails[1]">
        </div>
        <div v-else class="header-item"><span style="margin-left: 48px">散件</span></div>

        <div class="bar" :style="barStyle">{{ (item[1] * 100).toFixed(1) }}%</div>
    </div>
</template>

<script>
import { getArtifactThumbnail } from "@util/artifacts"
import { deepCopy } from "@util/common"
import { convertArtifactNameBack } from "@util/converter"

export default {
    name: "ArtifactBar",
    props: ["item"],
    computed: {
        isSet4() {
            return !!this.item[0]["Set4"]
        },

        isSet22() {
            return !!this.item[0]["Set22"]
        },

        isSet2() {
            return !!this.item[0]["Set2"]
        },

        isChiri() {
            return this.item[0] === "Chiri"
        },

        setThumbnails() {
            let names = []
            if (this.isSet4) {
                names.push(this.item[0]["Set4"])
            } else if (this.isSet2) {
                names.push(this.item[0]["Set2"])
            } else if (this.isSet22) {
                names = deepCopy(this.item[0]["Set22"])
            }

            return names.map(name => getArtifactThumbnail(convertArtifactNameBack(name)))
        },

        barStyle() {
            return {
                width: `calc((100% - 96px - 12px) * ${this.item[1]})`
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.header-item {
    display: flex;
    align-items: center;
    font-size: 12px;
    color: #606166;
    width: 96px;
    margin-right: 12px;
    height: 48px;
}
.tn {
    width: 48px;
    height: 48px;
    border-radius: 50%;
}

.artifact-bar {
    display: flex;
    align-items: center;
}

$bar-height: 32px;
.bar {
    height: $bar-height;
    background-color: #409EFF;
    //border-radius: 3px;
    font-size: 12px;
    color: #606166;
    line-height: $bar-height;
    padding-left: 12px;
    box-sizing: border-box;
}
</style>