<template>
    <el-dialog
        title="添加圣遗物"
        :visible.sync="visible"
        width="80%"
        :before-close="onCancel"
    >
        <h3>套装</h3>
        <choose-artifact-set v-model="setName"></choose-artifact-set>

        <h3>位置</h3>
        <choose-artifact-position
            v-model="position"
            :setName="setName"
        ></choose-artifact-position>

        <div>
            <div class="tag-panel">
                <h3>主属性</h3>
                <!-- <choose-main-tag v-model="mainTag" :position="position"></choose-main-tag> -->
                <select-artifact-main-tag v-model="mainTag" :position="position"></select-artifact-main-tag>
            </div>
            <div class="tag-panel">
                <div class="flex-row">
                    <h3 style="margin-right: 8px">副属性</h3>
                    <el-button
                        icon="el-icon-refresh"
                        circle
                        size="mini"
                        @click="shuffleNormalTags"
                        title="随机"
                    ></el-button>
                    <el-button
                        icon="el-icon-question"
                        circle
                        style="padding: 0"
                        title="帮助"
                        type="text"
                        @click="openHelp"
                    ></el-button>
                </div>
                
                <select-artifact-normal-tag v-model="normalTags"></select-artifact-normal-tag>
            </div>
        </div>

        <template #footer>
            <el-button @click="onCancel">取消</el-button>
            <el-button type="primary" @click="onConfirm">确定</el-button>
        </template>
    </el-dialog>
</template>

<script>
import ChooseArtifactSet from "./ChooseArtifactSet";
import ChooseArtifactPosition from "./ChooseArtifactPosition";
// import ChooseMainTag from "./ChooseMainTag";
// import ChooseNormalTag from "./ChooseNormalTag";
import SelectArtifactNormalTag from "@c/SelectArtifactNormalTag";
import SelectArtifactMainTag from "@c/SelectArtifactMainTag";

import { getDetailName, getArtifactRealValue } from "@util/utils";
import randomNormalTag from "@/artifacts_numeric/random_normal_tag";
import { convertDisplayTagValue } from '../../../utils/utils';

function convertPercentage(item) {
    item.value = getArtifactRealValue(item.name, item.value);

    return item;
}


export default {
    name: "NewDialog",
    components: {
        ChooseArtifactSet,
        ChooseArtifactPosition,
        // ChooseMainTag,
        // ChooseNormalTag,
        SelectArtifactNormalTag,
        SelectArtifactMainTag,
    },
    props: {
        visible: {
            type: Boolean
        }
    },
    data: function() {
        return {
            // 套装名
            setName: "archaicPetra",
            // 位置名
            position: "flower",
            // 主属性名
            mainTag: {
                name: "lifeStatic",
                value: "1000",
            },
            // 副属性
            normalTags: [],
        }
    },
    methods: {
        /**
         * return the final artifact object
         */
        getArtifact: function() {
            return {
                setName: this.setName,
                position: this.position,
                detailName: getDetailName(this.setName, this.position),
                mainTag: this.getArtifactMainTag(),
                normalTags: this.getArtifactNormalTags(),
                omit: false,
                id: 0,  // it's a placeholder, id is determined in Vuex store
            }
        },

        getArtifactMainTag() {
            let temp = Object.assign({}, this.mainTag);
            temp.value = parseFloat(temp.value);

            if (isNaN(temp.value)) {
                temp.value = 0;
            }

            return convertPercentage(temp);
        },

        getArtifactNormalTags() {
            let temp = [];
            for (let item of this.normalTags) {
                if (item.value === "" || item.value === "0") {
                    continue;
                }

                temp.push(convertPercentage({
                    name: item.name,
                    value: parseFloat(item.value),
                }));
            }

            return temp;
        },

        onConfirm() {
            let result = this.getArtifact();
            this.$emit("confirm", result);
        },
        
        onCancel() {
            this.$emit("close");
        },

        shuffleNormalTags() {
            let temp = randomNormalTag(5, 20, [this.mainTag.name]);
            for (let i = 0, l = temp.length; i < l; i++) {
                temp[i].value = convertDisplayTagValue(temp[i].name, temp[i].value);
            }

            this.normalTags = temp;
        },

        openHelp() {
            let text = "使用随机副词条，可以快速构建大量合理的圣遗物，可以方便地确定某个角色适合什么圣遗物。例如，快速构建角斗士、乐团、魔女4件套，从而计算胡桃适合什么样的圣遗物组合";
            this.$alert(text, "帮助");
        }
    },
}
</script>

<style scoped>
.summary {
    margin-bottom: 8px;
}

.tag-panel {
    width: 50%;
    display: inline-block;
    vertical-align: top;
    
    box-sizing: border-box;
}

.tag-panel:nth-of-type(1) {
    padding-right: 16px;
}
</style>