<template>
    <div>
        <div class="section title">
            <img :src="image" class="title-image">
            <span class="title-name">{{ title }}</span>
        </div>

        <el-divider></el-divider>

        <div style="padding: 0 20px">
            <el-row :gutter="16">
                <el-col :span="12">
                    <p class="config-title">套装</p>
                    <select-artifact-set
                        v-model="setName"
                    ></select-artifact-set>
                </el-col>
                <el-col :span="12">
                    <p class="config-title">位置</p>
                    <select-artifact-slot
                        v-model="position"
                    ></select-artifact-slot>
                </el-col>
            </el-row>

            <el-row :gutter="16">
                <el-col :span="12">
                    <p class="config-title">品质</p>
                    <el-rate
                        v-model="star"
                    ></el-rate>
                </el-col>
                <el-col :span="12">
                    <p class="config-title">等级</p>
                    <el-input-number
                        v-model="level"
                        :max="20"
                        :min="0"
                        size="small"
                    ></el-input-number>
                </el-col>
            </el-row>
        </div>

        <el-divider></el-divider>

        <div class="section">
            <p class="config-title">词条</p>

            <input-artifact-main-stat v-model="mainStat"
                style="margin-bottom: 24px"
            ></input-artifact-main-stat>


            <div class="sub-stat-div">
                <div
                    v-for="(sub, index) in subStats"
                    :key="index"
                    class="sub-stat-item"
                >
                    <input-artifact-sub-stat
                        v-model="subStats[index]"
                    ></input-artifact-sub-stat>

                    <el-button
                        size="mini"
                        icon="el-icon-delete"
                        type="danger"
                        @click="handleRemoveSubStat(index)"
                    ></el-button>
                </div>
            </div>
        </div>


        <el-divider></el-divider>

        <div class="section">
            <el-row :gutter="12">
                <el-col :span="12">
                    <el-button type="primary" class="button"
                        @click="$emit('confirm', artifactId)"
                    >
                        确定
                    </el-button>
                </el-col>
                <el-col :span="12">
                    <el-button class="button"
                               @click="$emit('cancel')"
                    >
                        取消
                    </el-button>
                </el-col>
            </el-row>
        </div>
    </div>
</template>

<script>
import flowerIcon from "@image/misc/flower.png"
import {getArtifact, getArtifactImage, getArtifactImageByArtifact} from "@util/artifacts"
import { artifactsData } from "@artifact"
import { artifactTags } from "@const/artifact"
import { positions } from "@const/misc"

import InputArtifactSubStat from "@c/input/InputArtifactSubStat"
import InputArtifactMainStat from "@c/input/InputArtifactMainStat"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectArtifactSlot from "@c/select/SelectArtifactSlot"


export default {
    name: "EditArtifact",
    components: {
        InputArtifactSubStat,
        InputArtifactMainStat,
        SelectArtifactSet,
        SelectArtifactSlot,
    },
    data() {
        return {
            artifactId: -1,

            setName: "berserker",
            star: 5,
            level: 20,
            position: "flower",
            mainStat: { name: "attackStatic", value: 0 },
            subStats: [
                { name: null, value: 0 },
                { name: null, value: 0 },
                { name: null, value: 0 },
                { name: null, value: 0 },
            ]
        }
    },
    methods: {
        convertStat(stat) {
            const data = artifactTags[stat.name]
            if (data.percentage) {
                // return { name: stat.name, value: stat.value * 100 }
                return { name: stat.name, value: (stat.value * 100).toFixed(1) }
            } else {
                return { name: stat.name, value: stat.value }
            }
        },

        convertStatBack(stat) {
            if (!stat.name) {
                return null
            }

            const data = artifactTags[stat.name]
            let value = parseFloat(stat.value)
            if (data.percentage) {
                value /= 100
            }

            return { name: stat.name, value }
        },

        setId(id) {
            const artifact = getArtifact(id)

            this.artifactId = artifact.id
            this.setName = artifact.setName
            this.star = artifact.star
            this.level = artifact.level
            this.position = artifact.position
            this.mainStat = this.convertStat(artifact.mainTag)

            let subStats = []
            for (let stat of artifact.normalTags) {
                subStats.push(this.convertStat(stat))
            }
            while (subStats.length < 4) {
                subStats.push({ name: null, value: 0 })
            }
            this.subStats = subStats
        },

        getNewArtifact() {
            const mainStat = this.convertStatBack(this.mainStat)
            let subStats = []
            for (let stat of this.subStats) {
                const convertResult = this.convertStatBack(stat)
                if (convertResult) {
                    subStats.push(convertResult)
                }
            }

            return {
                setName: this.setName,
                star: this.star,
                level: this.level,
                position: this.position,
                mainTag: mainStat,
                normalTags: subStats
            }
        },

        handleRemoveSubStat(index) {
            this.$set(this.subStats, index, { name: null, value: 0 })
        }
    },
    computed: {
        image() {
            return getArtifactImage(this.setName, this.position)
        },

        title() {
            const data = artifactsData[this.setName]
            return data[this.position].chs
        }
    },
    watch: {
        setName(newValue, oldValue) {
            if (newValue === oldValue) {
                return
            }

            const data = artifactsData[newValue]
            if (!data[this.position]) {
                for (let position in positions) {
                    if (data[[position]]) {
                        this.position = position
                        break
                    }
                }
            }
        }
    }
}
</script>

<style scoped lang="scss">
.sub-stat-item {
    margin-bottom: 8px;
    display: flex;
    align-items: center;
    gap: 18px;

    &:last-of-type {
        margin-bottom: 0;
    }
}

.title {
    display: flex;
    align-items: center;
    justify-content: space-between;

    .title-image {
        width: 64px;
        height: 64px;
    }

    .title-name {
        font-size: 14px;
    }
}

.config-title {
    font-size: 12px;
}

.section {
    padding: 0 24px;
}

.button {
    width: 100%;
}
</style>