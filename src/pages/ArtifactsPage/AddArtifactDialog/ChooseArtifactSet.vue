<template>
    <div class="choose-artifact-set">
        <div
            v-for="(artData) in artifactsData"
            :key="artData.eng"
            :class="{active: artData.eng === value}"
            class="item"
            @click="$emit('input', artData.eng)"
        >
            <el-image
                class="image"
                :src="getArtifactThumbnailURL(artData.eng)"
            >
            </el-image>
            <p class="text">
                {{ artData.chs }}
            </p>
        </div>
        
        
    </div>
</template>

<script>
import { artifactsData } from "../../../assets/artifacts";
import { getArtifactThumbnailURL } from "../../../utils/utils";

// import colors from "@const/quality_colors";

export default {
    name: "SetChoose",
    inject: ["star"],
    created: function () {
        this.artifactsData = Object.values(artifactsData);
        this.artifactsData.sort((a, b) => {
            return b.maxStar - a.maxStar;
        });
        this.getArtifactThumbnailURL = getArtifactThumbnailURL;
    },
    props: {
        value: {
            type: String
        }
    },
    // computed: {
    //     glowColor() {
    //         return colors[this.star - 1];
    //     },

    //     boxShadow() {
    //         console.log(this.star);
    //         return `0 0 20px 1px ${this.glowColor}`;
    //     }
    // }
}
</script>

<style lang="scss" scoped>
.image {
    width: 48px;
    height: 48px;
    // border-radius: 50%;
    // box-shadow: 0 0 10px 5px #00000022;
}

.text {
    font-size: 12px;
    padding: 0;
    margin: 0;
    text-align: center;
    width: 48px;
}

.choose-artifact-set {
    display: flex;
    flex-wrap: wrap;
}

.item {
    margin-right: 4px;
    padding: 8px;
    color: #777777;

    &:hover {
        background: rgba(0, 0, 0, 0.1);
        cursor: pointer;
        border-radius: 3px;
        transition: 300ms;
    }

    &.active {
        /* background: rgba(0, 0, 0, 0.1); */
        border-radius: 3px;
        /* color: white; */
        background: none;
        box-shadow: 0 0 20px 1px #00000033;
    }
}
</style>