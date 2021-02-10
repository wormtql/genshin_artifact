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
                <choose-main-tag v-model="mainTag" :position="position"></choose-main-tag>
            </div>
            <div class="tag-panel">
                <h3>副属性</h3>
                <choose-normal-tag v-model="normalTags"></choose-normal-tag>
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
import ChooseMainTag from "./ChooseMainTag";
import ChooseNormalTag from "./ChooseNormalTag";

import { getDetailName, getArtifactRealValue } from "../../utils/utils";

function convertPercentage(item) {
    item.value = getArtifactRealValue(item.name, item.value);

    return item;
}


export default {
    name: "NewDialog",
    components: {
        ChooseArtifactSet,
        ChooseArtifactPosition,
        ChooseMainTag,
        ChooseNormalTag,
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
            normalTags: [{
                name: "",
                value: "0",
            },{
                name: "",
                value: "0",
            },{
                name: "",
                value: "0",
            },{
                name: "",
                value: "0",
            }],
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
        
        onCancel: function() {
            this.$emit("close");
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