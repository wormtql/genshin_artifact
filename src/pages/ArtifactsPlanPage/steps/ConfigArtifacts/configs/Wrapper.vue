<template>
    <div class="item">
        <img :src="artifactThumbnail" class="back-image">
        <div class="left">
            <img :src="artifactThumbnail" class="front-image">
        </div>
        <div class="right">
            <h3 class="config-title">{{ artifactTitle }}</h3>
            <template
                v-for="i in 5"
            >
                <div
                    v-if="artifactEffectText[i]"
                    :key="i"
                    class="effect-div"
                >
                    <span class="effect-count">{{ i }}件套：</span>
                    <span class="effect-description">{{ artifactEffectText[i] }}</span>
                </div>
            </template>

            <div class="config-div">
                <slot></slot>
            </div>
        </div>
    </div>
</template>

<script>
import { artifactsData } from "@asset/artifacts";
import { getArtifactThumbnailURL } from "@util/utils";

export default {
    name: "ArtifactConfigWrapper",
    props: {
        artifactName: {},
    },
    computed: {
        artifact() {
            return artifactsData[this.artifactName];
        },

        artifactEffectText() {
            return this.artifact.effectText.chs;
        },

        artifactThumbnail() {
            return getArtifactThumbnailURL(this.artifactName);
        },

        artifactTitle() {
            return this.artifact.chs;
        },
    }
}
</script>

<style lang="scss" scoped>
.item {
    display: flex;
    position: relative;
    overflow: hidden;

    .back-image {
        position: absolute;
        right: 0;
        bottom: -60px;
        z-index: -1;
        opacity: 0.1;
        pointer-events: none;
    }

    .left {
        width: 64px;
        
        .front-image {
            width: 64px;
            height: 64px;
        }
    }

    .right {
        flex: 1;
        padding-left: 16px;

        .effect-div {
            font-size: 13px;
            color: #606166;

            .effect-count {
                color: #67C23A;
                padding-right: 4px;
            }
        }

        .config-div {
            margin-top: 16px;
        }
    }
}
</style>