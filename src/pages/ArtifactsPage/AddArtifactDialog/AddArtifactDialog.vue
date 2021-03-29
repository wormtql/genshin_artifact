<template>
    <el-dialog
        title="添加圣遗物"
        :visible.sync="visible"
        width="80%"
        :before-close="onCancel"
    >
        <h3>支持 ctrl / cmd + V 粘贴圣遗物截图自动识别属性</h3>
        <h3>套装</h3>
        <choose-artifact-set
            :value="setName"
            @input="handleSetNameChange"
        ></choose-artifact-set>

        <h3>位置</h3>
        <choose-artifact-position
            v-model="position"
            :setName="setName"
        ></choose-artifact-position>

        <el-row :gutter="16">
            <el-col :span="12">
                <h3>品质</h3>
                <el-rate
                    @input="handleStarChange"
                    :value="star"
                    :max="artifactData.maxStar"
                ></el-rate>
            </el-col>
            <el-col :span="12">
                <div class="flex-row">
                    <h3 style="margin-right: 8px">等级</h3>
                    <el-button
                        icon="el-icon-d-arrow-left"
                        circle
                        size="mini"
                        title="0级"
                        @click="level = 0"
                    ></el-button>
                    <el-button
                        icon="el-icon-d-arrow-right"
                        circle
                        size="mini"
                        title="满级"
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

        <el-row :gutter="16">
            <el-col :span="12">
                <h3>主属性</h3>
                <!-- <choose-main-tag v-model="mainTag" :position="position"></choose-main-tag> -->
                <select-artifact-main-tag
                    :value="mainTag"
                    :position="position"
                    @input="handleMainTagChange"
                ></select-artifact-main-tag>
            </el-col>
            <el-col :span="12">
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

                <select-artifact-normal-tag
                    v-model="normalTags"
                ></select-artifact-normal-tag>
            </el-col>
        </el-row>

        <template #footer>
            <el-button @click="onCancel">取消</el-button>
            <el-button type="primary" @click="onConfirm">确定</el-button>
        </template>
    </el-dialog>
</template>

<script>
import axios from 'axios';
import ChooseArtifactSet from './ChooseArtifactSet';
import ChooseArtifactPosition from './ChooseArtifactPosition';
// import ChooseMainTag from "./ChooseMainTag";
// import ChooseNormalTag from "./ChooseNormalTag";
import SelectArtifactNormalTag from '@c/SelectArtifactNormalTag';
import SelectArtifactMainTag from '@c/SelectArtifactMainTag';

import { getDetailName, getArtifactRealValue } from '@util/utils';
import randomNormalTag from '@/artifacts_numeric/random_normal_tag';
import { convertDisplayTagValue } from '@util/utils';
import {
    artifactsData,
    artifactsNamesSet,
    artifactsNameTypeMap,
    artifactsPositionTypeMap,
} from '@asset/artifacts';
import { secondaryTags, attributesSet, getAttributeType } from '@asset/tags';

function convertPercentage(item) {
    item.value = getArtifactRealValue(item.name, item.value);

    return item;
}

export default {
    name: 'NewDialog',
    components: {
        ChooseArtifactSet,
        ChooseArtifactPosition,
        SelectArtifactNormalTag,
        SelectArtifactMainTag,
    },
    props: {
        visible: {
            type: Boolean,
        },
    },
    data: function() {
        return {
            // 套装名
            setName: 'archaicPetra',
            // 位置名
            position: 'flower',
            // 主属性名
            mainTag: {
                name: 'lifeStatic',
                value: '4780',
            },
            // 副属性
            normalTags: [],
            // 星级
            star: 5,
            level: 20,
        };
    },
    created() {
        document.addEventListener('paste', this.handlePaste);
    },
    beforeDestroy() {
        document.removeEventListener('paste', this.handlePaste);
    },
    methods: {
        handleStarChange(e) {
            if (
                e >= this.artifactData.minStar &&
                e <= this.artifactData.maxStar
            ) {
                this.star = e;
                this.level = this.star * 4;
            }
        },

        handleMainTagChange(e) {
            if (e.name !== this.mainTag.name) {
                let maxValue = secondaryTags[e.name].max[5];

                this.mainTag.value = convertDisplayTagValue(e.name, maxValue);
                this.mainTag.name = e.name;
            } else {
                this.mainTag = e;
            }
        },

        handleSetNameChange(e) {
            this.setName = e;
            this.star = this.artifactData.maxStar;
            this.level = this.star * 4;
        },

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
                omit: this.level < 16,
                star: this.star,
                level: this.level,
                id: 0, // it's a placeholder, id is determined in Vuex store
            };
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
                if (item.value === '' || item.value === '0') {
                    continue;
                }

                temp.push(
                    convertPercentage({
                        name: item.name,
                        value: parseFloat(item.value),
                    }),
                );
            }

            return temp;
        },

        onConfirm() {
            let result = this.getArtifact();
            this.$emit('confirm', result);
        },

        onCancel() {
            this.$emit('close');
        },

        shuffleNormalTags() {
            let temp = randomNormalTag(5, 20, [this.mainTag.name]);
            for (let i = 0, l = temp.length; i < l; i++) {
                temp[i].value = convertDisplayTagValue(
                    temp[i].name,
                    temp[i].value,
                );
            }

            this.normalTags = temp;
        },

        openHelp() {
            let text =
                '使用随机副词条，可以快速构建大量合理的圣遗物，可以方便地确定某个角色适合什么圣遗物。例如，快速构建角斗士、乐团、魔女4件套，从而计算胡桃适合什么样的圣遗物组合';
            this.$alert(text, '帮助');
        },

        handlePaste(e) {
            if (this.visible) {
                const { items, types } =
                    e.clipboardData || e.originalEvent.clipboardData;
                if (types.indexOf('Files') > -1) {
                    for (let index = 0; index < items.length; index++) {
                        const item = items[index];
                        if (item.kind === 'file') {
                            const file = item.getAsFile();
                            if (file) {
                                const reader = new FileReader();
                                const fetchBaiduOcr = this.fetchBaiduOcr.bind(
                                    this,
                                );
                                reader.onloadend = function handleLoad() {
                                    if (this.result) {
                                        const [, imageData] = this.result.split(
                                            'base64,',
                                        );
                                        fetchBaiduOcr(imageData);
                                    }
                                };
                                reader.readAsDataURL(file);
                            }
                        }
                    }
                    e.preventDefault();
                }
            }
        },

        async fetchBaiduOcr(imageData) {
            if (imageData) {
                // 该默认 access token 将于 2021-4-21 过期
                // 如何获取 access token: https://cloud.baidu.com/doc/OCR/s/Ck3h7y2ia
                const baiduOcrAccessToken =
                    window.localStorage.getItem('boat') ||
                    '24.131a626ece49e226990bc403a1ae7cc4.2592000.1619089888.282335-10664786';
                const result = await axios.post(
                    `https://aip.baidubce.com/rest/2.0/ocr/v1/general?access_token=${baiduOcrAccessToken}`,
                    `image=${encodeURIComponent(imageData)}`,
                    {
                        headers: {
                            'Content-Type': 'application/x-www-form-urlencoded',
                        },
                    },
                );
                if (result.status === 200 && result.data.words_result_num > 0) {
                    this.parseOcrResultWords(
                        result.data.words_result.map(({ words }) => words),
                    );
                } else {
                    console.warn('OCR 识别失败!', result.status, result.data);
                }
            }
        },

        parseOcrResultWords(words) {
            const startIndex = words.findIndex(word =>
                artifactsNamesSet.has(word),
            );
            if (startIndex !== -1) {
                const artifact = {
                    name: words[startIndex],
                    type: words[startIndex + 1],
                    mainAttribute: {
                        type: words[startIndex + 2],
                        value: words[startIndex + 3].replace(',', ''),
                    },
                    star: words[startIndex + 4].length,
                    level: words[startIndex + 5],
                    secondaryAttributes: [],
                };
                for (let i = startIndex + 6; i < words.length; i++) {
                    const matchResult = words[i].match(
                        /([\u4e00-\u9fa5]+).*?([0-9.,%]+)/,
                    );
                    if (matchResult) {
                        const [, attributeName, attributeValue] = matchResult;
                        if (attributeName && attributesSet.has(attributeName)) {
                            artifact.secondaryAttributes.push({
                                type: attributeName,
                                value: attributeValue.replace(',', ''),
                            });
                        }
                    }
                }

                this.setName = artifactsNameTypeMap[artifact.name];
                this.position = artifactsPositionTypeMap[artifact.type];
                this.star = artifact.star;
                this.level = artifact.level;
                this.mainTag = {
                    name: getAttributeType(
                        artifact.mainAttribute.type,
                        artifact.mainAttribute.value,
                    ),
                    value: parseFloat(artifact.mainAttribute.value) || 0,
                };
                this.normalTags = artifact.secondaryAttributes.map(
                    ({ type, value }) => ({
                        name: getAttributeType(type, value),
                        value: parseFloat(value) || 0,
                    }),
                );
            } else {
                console.warn('结果中找不到圣遗物名');
            }
        },
    },
    computed: {
        artifactData() {
            return artifactsData[this.setName];
        },
    },
};
</script>

<style scoped>
</style>