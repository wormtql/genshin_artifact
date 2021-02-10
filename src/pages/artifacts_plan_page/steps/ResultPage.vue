<template>
    <div>
        <div class="main-div">
            <div class="left">
                <h3 class="title">最佳搭配</h3>
                <div class="artifact-div" v-if="!calculating">
                    <artifact-display
                        v-for="(art, index) in filteredArtifacts"
                        :key="index"
                        :item="art"
                        class="artifact"
                    ></artifact-display>
                </div>

                <h3 class="title">最大值</h3>
                <p class="max-value" v-if="!calculating">{{ resultData.value.toFixed(3) }}</p>
            </div>
            <div class="right">
                <attribute-panel :panel="resultData.attribute" style="width: 300px" v-if="!calculating"></attribute-panel>
            </div>
        </div>
    </div>
</template>

<script>
import ArtifactDisplay from "@c/ArtifactDisplay";
import AttributePanel from "@c/AttributePanel";

export default {
    name: "ResultPage",
    components: {
        ArtifactDisplay,
        AttributePanel,
    },
    props: {
        calculating: {
            type: Boolean,
            default: true,
        },
        resultData: {
            type: Object
        }
    },
    computed: {
        filteredArtifacts() {
            return this.resultData.artifacts.filter(item => item);
        }
    }
}
</script>

<style scoped>
.title {
    /* background:rgb(74, 99, 211); */
    padding: 0px 16px;
    display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}

.artifact-div {
    display: flex;
    flex-wrap: wrap;
}

.artifact {
    margin: 0 16px 16px 0;
}

.max-value {
    padding: 0;
    margin: 0;
    font-size: 14px;
}

.left {
    flex: 1;
}

.main-div {
    display: flex;
}
</style>