<template>
    <el-drawer
        title="编辑词条"
        :visible.sync="visible"
        direction="rtl"
        :before-close="handleCancel"
    >
        <div class="header" v-if="this.artifact">
            <img :src="thumbnail.url">
            <span>{{ thumbnail.chs }}</span>
        </div>

        <el-divider></el-divider>

        <div style="padding: 0 20px">
            <el-row :gutter="16" v-if="this.artifact">
                <el-col :span="12">
                    <p class="title">品质</p>
                    <el-rate
                        @input="handleStarChange"
                        :value="star"
                        :max="artifactData.maxStar"
                    ></el-rate>
                </el-col>
                <el-col :span="12">
                    <div class="flex-row">
                        <p style="margin-right: 8px" class="title">等级</p>
                        <el-button icon="el-icon-d-arrow-left" circle size="mini" title="0级"
                            @click="level = 0"
                        ></el-button>
                        <el-button icon="el-icon-d-arrow-right" circle size="mini" title="满级"
                            @click="level = star * 4"
                        ></el-button>
                    </div>
                    
                    <el-input-number
                        v-model="level"
                        :max="star * 4"
                        :min="0"
                        size="small"
                    ></el-input-number>
                    
                </el-col>
            </el-row>
        </div>

        <el-divider></el-divider>

        <div class="main-tag" v-if="this.artifact">
            <p class="title">词条</p>
            <select-artifact-main-tag
                :position="artifact.position"
                v-model="mainTag"
            ></select-artifact-main-tag>
        </div>

        <div class="normal-tag" v-if="this.artifact">
            <select-artifact-normal-tag v-model="normalTags">
            </select-artifact-normal-tag>
        </div>

        <el-divider></el-divider>

        <div class="buttons">
            <el-row :gutter="12">
                <el-col :span="12">
                    <el-button type="primary" @click="handleConfirm">
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
            star: 5,
            level: 1,
        }
    },
    methods: {
        handleStarChange(e) {
            if (e >= this.artifactData.minStar && e <= this.artifactData.maxStar) {
                this.star = e;
                this.level = this.star * 4;
            }
        },

        setInit(item) {
            this.mainTag.name = item.mainTag.name;
            this.mainTag.value = convertDisplayTagValue(item.mainTag.name, item.mainTag.value);

            this.normalTags = [];
            for (let tag of item.normalTags) {
                this.normalTags.push({
                    name: tag.name,
                    value: convertDisplayTagValue(tag.name, tag.value),
                });
            }

            this.star = item.star || 5;
            this.level = item.level || 20;
        },

        handleConfirm() {
            this.doUpdate();
            this.$emit("close");
        },

        doUpdate() {
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

            newArtifact.level = this.level;
            newArtifact.star = this.star;

            this.$store.commit("setArtifact", {
                position: this.args.position,
                index: this.args.index,
                artifact: newArtifact,
            });
        },

        handleCancel() {
            this.$emit("close");
        }
    },
    computed: {
        artifact() {
            if (this.args.index < 0) {
                return null;
            }
            let x = this.$store.state[this.args.position][this.args.index];
            return x;
        },

        artifactData() {
            return artifactsData[this.artifact.setName] || {};
        },

        thumbnail() {
            if (this.artifact) {
                let data = artifactsData[this.artifact.setName];
                return data[this.artifact.position];
            }

            return {};
        },
    },
    // watch: {
    //     visible(vis) {
    //         if (!vis) {
    //             return;
    //         }
            
    //         let newArt = this.artifact;
    //         setInit(newArt);
    //     }
    // }
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
    /* margin-top: 32px; */
}

.buttons button {
    width: 100%;
}

.title {
    font-size: 12px;

}
</style>