<template>
    <div v-loading="loading">
        <el-breadcrumb style="margin-bottom: 12px; height: 24px">
            <el-breadcrumb-item>莫娜数据库</el-breadcrumb-item>
        </el-breadcrumb>

<!--        <el-divider></el-divider>-->

        <div v-if="error" @click="handleClickReload">数据加载发生错误，点击重试</div>
        <div v-else>
            <el-tabs>
                <el-tab-pane label="角色" name="character">
                    <el-row :gutter="12">
                        <el-col :span="4">
                            <div class="tab-full-height mona-scroll-hidden">
                                <div
                                    v-for="(cList, elementName) in characterByElement"
                                    :key="elementName"
                                    @click="handleClickCharacter"
                                >
                                    <p
                                        class="list-group-title"
                                    >{{ element2Chs(elementName) }}</p>
                                    <div
                                        v-for="characterData in cList"
                                        :key="characterData.name"
                                        class="list-item"
                                        :class="{ active: activeCharacterName === characterData.name }"
                                        :x-character-name="characterData.name"
                                    >
                                        <img :src="characterData.avatar" class="list-item-image" >
                                        <p class="list-item-title">{{ characterData.chs }}</p>
                                    </div>
                                </div>
                            </div>
                        </el-col>
                        <el-col :span="20" v-if="analysisResult">
                            <div class="tab-full-height mona-scroll-hidden">
                                <p class="analysis-item-title">武器使用率</p>
                                <div
                                    v-for="item in characterResult[this.activeCharacterName].weapon_usage"
                                    :key="item[0]"
                                    class="bar-item"
                                >
                                    <el-tooltip :content="weaponData[item[0]].chs">
                                        <img :src="weaponData[item[0]].url" class="graph-label-image" >
                                    </el-tooltip>

                                    <bar :fraction="item[1]"></bar>
                                </div>

                                {{ analysisResult.character_result[this.activeCharacterName] }}
                            </div>
                        </el-col>
                    </el-row>


                </el-tab-pane>
                <el-tab-pane label="武器" name="weapon"></el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import { getComputeResultAnalysis } from "@/api/misc"
import { characterByElement } from "@character"
import { weaponData } from "@weapon"
import { element2Chs } from "@util/common"

import Bar from "./Bar"

export default {
    name: "MonaDBPage",
    components: {
        Bar
    },
    data() {
        return {
            loading: true,
            error: false,

            activeCharacterName: "Diluc",
            analysisResult: null,

            characterByElement,
            weaponData
        }
    },
    async created() {
        await this.handleClickReload()
    },
    methods: {
        element2Chs,

        handleClickCharacter(e) {
            if (e.target.hasAttribute("x-character-name")) {
                const name = e.target.getAttribute("x-character-name")
                this.activeCharacterName = name
            }
        },

        async handleClickReload() {
            if (await this.loadAnalysis()) {
                this.loading = false
                this.error = false
            } else {
                this.loading = false
                this.error = true
            }
        },

        async loadAnalysis() {
            const response = await getComputeResultAnalysis()

            if (response.status !== 200) {
                return false
            } else {
                this.analysisResult = response.data.data
                return true
            }
        }
    },
    computed: {
        characterResult() {
            if (this.analysisResult) {
                return this.analysisResult.character_result
            }

            return null
        }


    }
}
</script>

<style lang="scss" scoped>
.list-group-title {
    color: #909399;
    font-size: 12px;
}

.tab-full-height {
    height: calc(100vh - 40px - 24px - 12px - 55px);
}

.list-item {
    height: 48px;
    padding: 6px;
    display: flex;
    align-items: center;
    transition: 300ms;
    border-left: 3px solid transparent;

    &:hover {
        background-color: #f5f7fa;
        cursor: pointer;
    }

    &.active {
        border-left: 3px solid #79bbff;
        background-color: #ecf5ff;
    }

    .list-item-image {
        height: 48px;
        width: 48px;
        border-radius: 50%;
        pointer-events: none;
    }

    .list-item-title {
        font-size: 14px;
        color: #606266;
        margin-left: 24px;
        pointer-events: none;
    }
}

.bar-item {
    display: flex;
    align-items: center;

    .graph-label-image {
        width: 64px;
        height: 64px;
        border-radius: 50%;
        margin-right: 12px;
    }
}

</style>