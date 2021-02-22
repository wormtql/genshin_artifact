<template>
    <el-drawer
        title="编辑词条"
        :visible.sync="visible"
        direction="rtl"
        :before-close="handleClose"
    >
        <div class="header" v-if="this.artifact">
            <img :src="thumbnail.url">
            <span>{{ thumbnail.chs }}</span>
        </div>

        <div class="main-tag" v-if="this.artifact">
            <select-artifact-main-tag
                :position="artifact.position"
                v-model="mainTag"
            ></select-artifact-main-tag>
        </div>

        <div class="normal-tag" v-if="this.artifact">
            <select-artifact-normal-tag v-model="normalTags">
            </select-artifact-normal-tag>
        </div>

        <div class="buttons">
            <el-row :gutter="16">
                <el-col :span="12">
                    <el-button type="primary" @click="handleClose">
                        确定
                    </el-button>
                </el-col>
                <el-col :span="12">
                    <el-button @click="handleCancel">
                        取消
                    </el-button>
                </el-col>
            </el-row>
        </div>
    </el-drawer>
</template>

<script>
import { artifactsData } from "@asset/artifacts";
import { convertDisplayTagValue, getArtifactRealValue } from "@util/utils";
import deepCopy from "@util/deepcopy";

import SelectArtifactMainTag from "@c/SelectArtifactMainTag";
import SelectArtifactNormalTag from "@c/SelectArtifactNormalTag";

export default {
    name: "EditArtifactDrawer",
    components: {
        SelectArtifactMainTag,
        SelectArtifactNormalTag
    },
    props: {
        visible: {
            type: Boolean,
        },
        args: {
            type: Object,
            default: () => ({
                position: "",
                index: 0,
            })
        },
    },
    data: function () {
        return {
            mainTag: { name: "", value: "0" },
            normalTags: [
                { name: "", value: "0" },
                { name: "", value: "0" },
                { name: "", value: "0" },
                { name: "", value: "0" },
            ],
        }
    },
    methods: {
        handleClose() {
            let newArtifact = deepCopy(this.artifact);
            newArtifact.mainTag.name = this.mainTag.name;
            newArtifact.mainTag.value = getArtifactRealValue(this.mainTag.name, this.mainTag.value);

            newArtifact.normalTags = [];
            for (let tag of this.normalTags) {
                newArtifact.normalTags.push({
                    name: tag.name,
                    value: getArtifactRealValue(tag.name, tag.value),
                });
            }

            this.$store.commit("setArtifact", {
                position: this.args.position,
                index: this.args.index,
                artifact: newArtifact,
            });
            this.$forceUpdate();

            this.$emit("close");
        },

        handleCancel() {
            this.$emit("close");
        }
    },
    computed: {
        artifact() {
            let x = this.$store.state[this.args.position][this.args.index];
            return x;
        },

        thumbnail() {
            if (this.artifact) {
                let data = artifactsData[this.artifact.setName];
                return data[this.artifact.position];
            }

            return {};
        }
    },
    watch: {
        visible(vis) {
            if (!vis) {
                return;
            }
            let newArt = this.artifact;

            this.mainTag.name = newArt.mainTag.name;
            this.mainTag.value = convertDisplayTagValue(newArt.mainTag.name, newArt.mainTag.value);

            this.normalTags = [];
            for (let tag of newArt.normalTags) {
                this.normalTags.push({
                    name: tag.name,
                    value: convertDisplayTagValue(tag.name, tag.value),
                });
            }
        }
    }
}
</script>

<style scoped>
.header {
    padding: 0 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.header img {
    width: 64px;
    height: 64px;
    border-radius: 50%;
}

.header span {
    font-size: 18px;
    /* margin-left: 32px; */
    /* border-bottom: 3px solid #123456; */
}

.main-tag {
    padding: 0 20px;
    margin-top: 32px;
}

.normal-tag {
    padding: 0 20px;
    margin-top: 32px;
}

.buttons {
    padding: 0 20px;
    margin-top: 32px;
}

.buttons button {
    width: 100%;
}
</style>