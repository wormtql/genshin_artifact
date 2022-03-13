<template>
    <div class="root">
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物潜力</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row>
            <el-col
                :span="6"
                ref="content"
                :style="{ height: contentHeight }"
                class="mona-scroll"
            >
                <select-potential-function-name v-model="potentialFunctionName"></select-potential-function-name>

                <div class="pf-detail">
                    <img :src="pfImage" class="pf-image" >

                    <div>
                        <p class="pf-title">{{ pfChs }}</p>
                        <p class="pf-description">{{ pfDescription }}</p>
                    </div>
                </div>

                <div class="pf-config">
                    <item-config
                        v-model="potentialFunctionConfig"
                        :item-name="potentialFunctionName"
                        :configs="pfConfigConfigs"
                    ></item-config>
                </div>
            </el-col>
            <el-col
                :span="18"
                class="col-right mona-scroll"
                :style="{ height: contentHeight }"
                style="border: 16px solid transparent;"
            >
                <div>
                    <el-button
                        type="primary"
                        size="mini"
                        icon="el-icon-cpu"
                        @click="handleClickStart"
                    >开始计算</el-button>
                </div>

                <div style="display: flex; align-items: center; gap: 32px; margin: 16px 0;">
                    <span class="filter-item">
                        <span class="filter-title">位置</span>
                        <select-artifact-slot
                            v-model="filterSlots"
                            :multiple="true"
                        ></select-artifact-slot>
                    </span>

                    <span class="filter-item">
                        <span class="filter-title">套装</span>
                        <select-artifact-set
                            v-model="filterSetNames"
                            :multiple="true"
                            :multiple-limit="5"
                        ></select-artifact-set>
                    </span>

                    <span class="filter-item">
                        <span class="filter-title">主词条</span>
                        <select-artifact-main-stat
                            v-model="filterMainStatNames"
                            :multiple="true"
                            :include-any="false"
                        ></select-artifact-main-stat>
                    </span>
                </div>

                <div style="margin: 0 0 20px 0">
                    <el-input-number
                        size="small"
                        :min="0"
                        :max="filterLevel[1]"
                        v-model="filterLevel[0]"
                    ></el-input-number>
                    ~
                    <el-input-number
                        size="small"
                        :min="filterLevel[0]"
                        :max="20"
                        v-model="filterLevel[1]"
                    ></el-input-number>
<!--                    <el-slider-->
<!--                        v-model="filterLevel"-->
<!--                        range-->
<!--                        :max="20"-->
<!--                        :min="0"-->
<!--                    ></el-slider>-->
                </div>

                <div class="artifacts-div">
                    <artifact-display-by-id
                        v-for="result in resultsToBeDisplay"
                        :artifact-id="result[0]"
                        :extra="result[1].toFixed(2)"
                        :show-back="true"
                        :back-value="result[1] / results[0][1]"
                    ></artifact-display-by-id>
                </div>

                <div class="pager">
                    <el-pagination
                        :total="filteredResults.length"
                        :page-size="20"
                        :current-page.sync="currentPage"
                    ></el-pagination>
                </div>

            </el-col>
        </el-row>
    </div>
</template>

<script>
import { potentialFunctionData } from "@potentialFunction"
import { getPotentialFunctionDefaultConfig } from "@util/potentialFunction"
import { getArtifactsWasm } from "@util/artifacts"
import { wasmComputeArtifactPotential } from "@wasm"
import { mapGetters } from "vuex"

import SelectPotentialFunctionName from "@c/select/SelectPotentialFunctionName"
import SelectArtifactSlot from "@c/select/SelectArtifactSlot"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"
import ItemConfig from "@c/config/ItemConfig"
import ArtifactDisplayById from "@c/display/ArtifactDisplayById"


export default {
    name: "NewArtifactPotentialPage",
    components: {
        ArtifactDisplayById,
        SelectArtifactMainStat,
        SelectArtifactSet,
        SelectArtifactSlot,
        SelectPotentialFunctionName,
        ItemConfig,
    },
    data() {
        return {
            potentialFunctionName: "ArtifactEff",
            potentialFunctionConfig: getPotentialFunctionDefaultConfig("ArtifactEff"),

            results: [],        // [[id, score]] in descending order
            currentPage: 1,
            contentHeight: "",

            filterSlots: [],        // defaults to all
            filterSetNames: [],
            filterMainStatNames: [],
            filterLevel: [0, 20],
        }
    },
    mounted() {
        const component = this.$refs["content"]

        this.$nextTick(() => {
            const rect = component.$el.getBoundingClientRect()
            // console.log(rect)
            this.contentHeight = `calc(100vh - ${rect.top}px)`
        })
    },
    methods: {
        handleClickStart() {
            const artifactsWasm = getArtifactsWasm()
            const pfInterface = this.potentialFunctionInterface
            // console.log(pfInterface)

            const loading = this.$loading({
                text: "莫娜占卜中"
            })

            wasmComputeArtifactPotential(pfInterface, artifactsWasm).then(results => {
                // console.log(results)
                this.results = results
            }).catch(e => {
                this.$message.error(e)
            }).finally(() => {
                loading.close()
            })
        }
    },
    computed: {
        ...mapGetters("artifacts", {
            artifactsById: "artifactsById"
        }),

        potentialFunctionInterface() {
            return {
                name: this.potentialFunctionName,
                config: this.potentialFunctionConfig
            }
        },

        pfImage() {
            const item = potentialFunctionData[this.potentialFunctionName]
            // console.log(item)
            return item?.badge
        },

        pfChs() {
            const item = potentialFunctionData[this.potentialFunctionName]
            return item?.chs
        },

        pfDescription() {
            const item = potentialFunctionData[this.potentialFunctionName]
            return item?.description
        },

        pfConfigConfigs() {
            const item = potentialFunctionData[this.potentialFunctionName]
            if (item) {
                return item.config
            } else {
                return []
            }
        },

        filteredResults() {
            let results = []

            for (let item of this.results) {
                const artifact = this.artifactsById[item[0]]
                if (!artifact) {
                    continue
                }
                if (this.filterSlots.length > 0) {
                    if (this.filterSlots.indexOf(artifact.position) === -1) {
                        continue
                    }
                }

                if (this.filterSetNames.length > 0) {
                    if (this.filterSetNames.indexOf(artifact.setName) === -1) {
                        continue
                    }
                }

                if (this.filterMainStatNames.length > 0) {
                    if (this.filterMainStatNames.indexOf(artifact.mainTag.name) === -1) {
                        continue
                    }
                }

                const level = artifact?.level ?? 20
                if (level < this.filterLevel[0] || level > this.filterLevel[1]) {
                    continue
                }

                results.push(item)
            }

            return results
        },

        resultsToBeDisplay() {
            return this.filteredResults.slice((this.currentPage - 1) * 20, Math.min(this.currentPage * 20, this.filteredResults.length))
        }
    },
    watch: {
        potentialFunctionName(newValue, oldValue) {
            if (newValue === oldValue) {
                return
            }

            const item = potentialFunctionData[newValue]
            const configConfigs = item.config ?? []
            if (configConfigs.length === 0) {
                this.potentialFunctionConfig = "NoConfig"
            } else {
                let defaultConfig = {}
                for (const config of configConfigs) {
                    defaultConfig[config.name] = config.default
                }

                this.potentialFunctionConfig = {
                    [newValue]: defaultConfig
                }
            }
        }
    }
}
</script>

<style scoped lang="scss">
.pf-detail {
    margin-top: 20px;
    display: flex;
    align-items: flex-start;

    .pf-image {
        height: 64px;
        width: 64px;
        border-radius: 64px;
        margin-right: 8px;
    }

    .pf-title {
        font-weight: bold;
        font-size: 0.9rem;
        margin: 0 0 12px 0;
    }

    .pf-description {
        font-size: 0.8rem;
        margin: 0;
        color: #606266;
    }
}

.pf-config {
    margin-top: 20px;
}

.filter-item {
    .filter-title {
        font-size: 0.8rem;
        margin-right: 20px;
        color: #606266;
    }
}

.col-right {
    position: relative;
}

.pager {
    position: absolute;
    width: 100%;
    bottom: 0;
}

.artifacts-div {
    display: grid;
    grid-template-columns: repeat(auto-fill, 200px);
    justify-content: space-between;
    align-content: flex-start;
    grid-gap: 12px;
}
</style>