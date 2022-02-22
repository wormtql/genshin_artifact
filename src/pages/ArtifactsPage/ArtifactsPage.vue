<template>
    <div>
        <!-- new artifact dialog -->
        <add-artifact-dialog
            :visible="newDialogVisible"
            @close="newDialogVisible = false"
            @confirm="handleAddArtifact"
        ></add-artifact-dialog>

        <el-dialog
            :visible.sync="showImportDialog"
            title="导入"
            width="60%"
        >
            <import-block ref="fileUploader"></import-block>
            <el-checkbox
                v-model="importDeleteUnseen"
                style="margin-top: 12px"
            >删除不存在的圣遗物</el-checkbox>

            <template #footer>
                <el-button @click="showImportDialog = false">取消</el-button>
                <el-button type="primary" @click="handleImportJson">确定</el-button>
            </template>
        </el-dialog>

        <output-json-dialog
            :visible="outputJsonDialogVisible"
            @close="outputJsonDialogVisible = false"
        >
        </output-json-dialog>

        <el-drawer
            title="编辑圣遗物"
            :visible.sync="showEditArtifactDrawer"
            direction="rtl"
        >
            <edit-artifact
                ref="editArtifactDrawer"
                @confirm="handleConfirmEdit"
                @cancel="showEditArtifactDrawer = false"
            ></edit-artifact>
        </el-drawer>

        <!-- <div style="position: sticky"> -->
        <!-- bread crumb -->
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物（{{ count }}）</el-breadcrumb-item>
        </el-breadcrumb>
<!--        <el-divider></el-divider>-->

<!--        <el-alert-->
<!--            title="请注意保存圣遗物数据至本地，以防意外导致数据丢失"-->
<!--            type="warning"-->
<!--            :closable="false"-->
<!--            style="margin-bottom: 16px"-->
<!--        ></el-alert>-->



        <!-- tool bar -->
        <div class="tool-bar">
<!--            <el-tag>数量：{{ $store.getters["artifacts/count"] }}</el-tag>-->
            <el-button
                @click="add"
                type="primary"
                icon="el-icon-plus"
                size="mini"
                style="margin-right: 8px"
            ></el-button>

            <el-popconfirm
                title="确定清除吗，将会同时清除圣遗物套装数据"
                @confirm="handleClickDeleteAll"
                style="margin-right: 8px"
            >
                <el-button
                    slot="reference"
                    size="mini"
                    icon="el-icon-delete"
                    type="danger"
                    title="清空"
                >
                    清空
                </el-button>
            </el-popconfirm>

            <el-button
                size="mini"
                icon="el-icon-unlock"
                title="启用全部"
                @click="$store.commit('artifacts/unlockAll')"
            >启用全部</el-button>

            <div class="tool-right">
                <el-button @click="handleImportJsonClicked">
                    导入json
                </el-button>
                <el-button @click="handleOutputJsonClicked">
                    导出json
                </el-button>
            </div>
        </div>

        <!-- </div> -->
        <div class="filter">
            <span>套装</span>
            <select-artifact-set
                v-model="filterSet"
                :multiple="true"
                :multiple-limit="1000"
            ></select-artifact-set>

            <span style="margin-left: 24px">主词条</span>
            <select-artifact-main-stat
                v-model="filterMainStat"
                :include-any="false"
                :multiple="true"
            ></select-artifact-main-stat>

            <el-checkbox
                v-model="ge16"
                style="margin-left: 24px"
            >只显示16级以上</el-checkbox>
        </div>

        <!-- artifacts display -->
        <el-tabs v-model="activeName">
            <el-tab-pane
                v-for="tab in tabs"
                :key="tab.name"
                class="panel"
                :name="tab.name"
            >
                <div slot="label">
                    <img :src="tab.icon" class="icon">
                </div>

                <div v-if="filteredArtifacts.length > 0">
                    <div class="artifacts-div mona-scroll" ref="artifactsDiv"
                        :style="{ height: contentHeight }"
                    >
                        <artifact-display
                            class="artifact-item"
                            v-for="(item) in artifactToBeDisplayed"
                            :key="item.id"
                            :item="item"
                            :buttons="true"
                            :delete-button="true"
                            :edit-button="true"
                            @delete="handleClickRemoveArtifact(item.id)"
                            @toggle="handleClickToggleArtifact(item.id)"
                            @edit="handleClickEditArtifact(item.id)"
                        ></artifact-display>
                    </div>
<!--                    <el-pagination-->
<!--                        :current-page.sync="currentPage"-->
<!--                        :page-size="pageSize"-->
<!--                        :total="filteredArtifacts.length"-->
<!--                    ></el-pagination>-->
                </div>
                <div v-else>
                    <el-empty></el-empty>
                </div>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import { mapGetters } from "vuex"
import {removeArtifact, toggleArtifact, updateArtifact, importMonaJson} from "@util/artifacts"

import flowerIcon from "@image/misc/flower.png"
import featherIcon from "@image/misc/feather.png"
import sandIcon from "@image/misc/sand.png"
import gobletIcon from "@image/misc/goblet.png"
import headIcon from "@image/misc/head.png"

import AddArtifactDialog from "./AddArtifactDialog"
import OutputJsonDialog from "./OutputJsonDialog"
import SelectArtifactSet from "@c/select/SelectArtifactSet"
import SelectArtifactMainStat from "@c/select/SelectArtifactMainStat"
import ArtifactDisplay from "@c/display/ArtifactDisplay"
import EditArtifact from "./EditArtifact"
import ImportBlock from "@c/misc/ImportBlock";

const tabs = [
    { icon: flowerIcon, name: "flower" },
    { icon: featherIcon, name: "feather" },
    { icon: sandIcon, name: "sand" },
    { icon: gobletIcon, name: "cup" },
    { icon: headIcon, name: "head" },
]
Object.freeze(tabs)

const pageSize = 20

export default {
    name: "ArtifactsPage",
    components: {
        ImportBlock,
        AddArtifactDialog,
        OutputJsonDialog,
        SelectArtifactSet,
        SelectArtifactMainStat,
        ArtifactDisplay,
        EditArtifact,
    },
    created: function () {
        this.tabs = tabs
        this.pageSize = pageSize
    },
    mounted() {
        this.$nextTick(() => {
            const component = this.$refs["artifactsDiv"]?.[0]
            if (!component) {
                return
            }

            const rect = component.getBoundingClientRect()
            console.log(rect.top)
            this.contentHeight = `calc(100vh - ${rect.top}px)`
        })

    },
    data: function() {
        return {
            activeName: "flower",

            newDialogVisible: false,
            outputJsonDialogVisible: false,
            showEditArtifactDrawer: false,
            showImportDialog: false,

            filterSet: [],
            filterMainStat: [],
            ge16: true,
            // currentPage: 1,

            contentHeight: "",

            importDeleteUnseen: false
        }
    },
    methods: {
        handleClickDeleteAll() {
            this.$store.commit("artifacts/removeAllArtifacts");
        },

        handleClickRemoveArtifact(id) {
            removeArtifact(id)
        },

        handleClickToggleArtifact(id) {
            toggleArtifact(id)
        },

        handleClickEditArtifact(id) {
            // console.log(id)
            this.showEditArtifactDrawer = true

            this.$nextTick(() => {
                let component = this.$refs["editArtifactDrawer"]
                if (!component) {
                    return
                }
                component.setId(id)
            })
        },

        handleConfirmEdit(id) {
            let component = this.$refs["editArtifactDrawer"]
            if (!component) {
                return
            }
            let newArtifact = component.getNewArtifact()

            updateArtifact(id, newArtifact)

            this.showEditArtifactDrawer = false
        },

        add: function() {
            this.newDialogVisible = true;
        },

        handleAddArtifact: function(item) {
            this.newDialogVisible = false;

            this.activeName = item.position;

            this.$store.commit("artifacts/addArtifact", item);
        },

        handleImportJsonClicked() {
            this.showImportDialog = true
        },

        handleImportJson() {
            const component = this.$refs.fileUploader
            if (!component) {
                return
            }

            const loading = this.$loading({
                lock: true,
                text: "导入中",
            })

            component.getReadPromise().then(text => {
                // console.log(text)
                try {
                    const rawObj = JSON.parse(text)
                    importMonaJson(rawObj, this.importDeleteUnseen)
                } catch(e) {
                    return Promise.reject("格式不正确")
                }
            }).catch(e => {
                this.$message.error(e)
            }).finally(() => {
                loading.close()
            })
        },

        handleOutputJsonClicked() {
            this.outputJsonDialogVisible = true;
        },
    },
    computed: {
        ...mapGetters("artifacts", [
            "allArtifacts",
            "count"
        ]),

        artifactsCurrentSlotFlat() {
            const items = this.allArtifacts[this.activeName]
            return items
        },

        filteredArtifacts() {
            let results = []

            for (let artifact of this.artifactsCurrentSlotFlat) {
                const setName = artifact.setName
                const mainStatName = artifact.mainTag.name
                const level = artifact.level ?? 20

                if (this.filterSet.length > 0 && this.filterSet.indexOf(setName) === -1) {
                    continue
                }
                if (this.filterMainStat.length > 0 && this.filterMainStat.indexOf(mainStatName) === -1) {
                    continue
                }
                if (this.ge16 && level < 16) {
                    continue
                }

                results.push(artifact)
            }

            return results
        },

        artifactToBeDisplayed() {
            // return this.artifactsCurrentSlotFlat
            return this.filteredArtifacts
            // const start = (this.currentPage - 1) * pageSize
            // const end = Math.min(start + pageSize, this.filteredArtifacts.length)
            //
            // return this.filteredArtifacts.slice(start, end)
        },
    }
}
</script>

<style scoped lang="scss">
.filter {
    margin-bottom: 12px;

    span {
        font-size: 12px;
        color: #606266;
        margin-right: 8px;
    }
}

.icon {
    padding: 0 12px;
}

.artifacts-div {
    //display: flex;
    //flex-wrap: wrap;
    //align-items: flex-start;
    //justify-content: space-between;
    //gap: 12px;
    padding-right: 20px;
    padding-bottom: 20px;
    box-sizing: border-box;

    display: grid;
    grid-template-columns: repeat(auto-fill, 200px);
    justify-content: space-between;
    align-content: flex-start;
    grid-gap: 12px;

    //&::after {
    //    content: "";
    //    flex: auto;
    //}

    .artifact-item {
        //width: 20%;
        //width: 200px;
    }
}

.tool-bar {
    margin-bottom: 16px;
    margin-top: 16px;
}

.tool-bar .tool-right {
    float: right;
}
</style>