<template>
    <div v-loading.fullscreen="loading">
        <!-- new artifact dialog -->
        <add-artifact-dialog
            :visible="newDialogVisible"
            @close="newDialogVisible = false"
            @confirm="handleAddArtifact"
        ></add-artifact-dialog>

        <yas-ui-dialog :visible.sync="showYasUIDialog"></yas-ui-dialog>

        <el-dialog
            :visible.sync="showOutputShareDialog"
            title="分享"
            :width="deviceIsPC ? '500px' : '90%'"
        >
            <p>通过分享链接可以快速导入圣遗物，有效期为一天</p>
            <el-input v-model="shareLink"></el-input>

            <template #footer>
                <el-button type="primary" @click="handleCopyShareLink">复制</el-button>
            </template>
        </el-dialog>

        <el-dialog :visible.sync="showImportDialog" title="导入" :width="deviceIsPC ? '60%' : '90%'">
            <import-block ref="fileUploader"></import-block>
            <el-checkbox v-model="importDeleteUnseen" style="margin-top: 12px">删除不存在的圣遗物</el-checkbox>

            <template #footer>
                <el-button @click="showImportDialog = false">取消</el-button>
                <el-button type="primary" @click="handleImportJson">确定</el-button>
            </template>
        </el-dialog>

        <el-drawer title="编辑圣遗物" :visible.sync="showEditArtifactDrawer" direction="rtl" :size="deviceIsPC ? '30%' : '100%'">
            <edit-artifact
                ref="editArtifactDrawer"
                @confirm="handleConfirmEdit"
                @cancel="showEditArtifactDrawer = false"
            ></edit-artifact>
        </el-drawer>

        <el-drawer title="推荐圣遗物" :visible.sync="showArtifactRecommendationDrawer" :size="deviceIsPC ? '30%' : '100%'">
            <el-empty v-if="recommendationList.length === 0"></el-empty>
            <div v-else style="padding: 0 20px">
                <artifact-display
                    v-for="item in recommendationList"
                    :key="item[0]"
                    :item="artifactsById[item[0]]"
                    style="width: 100%; margin-bottom: 16px"
                    :show-back="true"
                    :back-value="item[1]"
                ></artifact-display>
            </div>
        </el-drawer>

        <el-breadcrumb class="hidden-sm-and-down">
            <el-breadcrumb-item>圣遗物（{{ count }}）</el-breadcrumb-item>
        </el-breadcrumb>

        <div class="toolbar-mobile hidden-md-and-up" style="margin-bottom: 12px">
            <el-dropdown @command="handleDropdownCommand" trigger="click">
                <span class="el-dropdown-link"><i class="el-icon-more"></i></span>
                <el-dropdown-menu slot="dropdown">
                    <el-dropdown-item command="add"><i class="el-icon-plus"></i>添加</el-dropdown-item>
                    <el-dropdown-item command="deleteAll"><i class="el-icon-delete"></i>清空</el-dropdown-item>
                    <el-dropdown-item divided command="unlockAll"><i class="el-icon-unlock"></i>启用全部</el-dropdown-item>
                    <el-dropdown-item divided command="recommend"><i class="el-icon-s-opportunity"></i>推荐</el-dropdown-item>
                </el-dropdown-menu>
            </el-dropdown>

            <el-dropdown trigger="click" style="margin-left: 16px" @command="handleDropdownCommand">
                <span class="el-dropdown-link"><i class="el-icon-upload2"></i></span>

                <el-dropdown-menu slot="dropdown">
                    <el-dropdown-item command="importJson"><i class="el-icon-arrow-right"></i>导入</el-dropdown-item>

                    <el-dropdown-item divided command="exportJson"><i class="el-icon-arrow-left"></i>导出莫娜JSON</el-dropdown-item>
                    <el-dropdown-item command="exportShare"><i class="el-icon-arrow-left"></i>分享链接</el-dropdown-item>
                </el-dropdown-menu>
            </el-dropdown>

            <div class="m-center">总数：{{ count }}</div>
        </div>

        <div class="tool-bar hidden-sm-and-down">
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
                <el-button slot="reference" size="mini" icon="el-icon-delete" type="danger" title="清空">
                    清空
                </el-button>
            </el-popconfirm>

            <el-button size="mini" icon="el-icon-unlock" title="启用全部" @click="unlockAllArtifacts"
                >启用全部</el-button
            >

            <el-button size="mini" icon="el-icon-s-opportunity" @click="handleClickRecommendation">推荐</el-button>

            <div class="tool-right">
                <el-button @click="handleYasUIClicked" size="mini" type="primary" v-if="deviceIsPC"> 扫描 </el-button>
                <el-button @click="handleImportJsonClicked" size="mini" type="primary"> 导入 </el-button>
<!--                <el-button @click="handleOutputJsonClicked" size="mini"> 导出 </el-button>-->

                <el-dropdown split-button size="mini" @click="handleOutputJsonClicked" @command="handleOutputCommand">
                    导出
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item command="monaJson">莫娜JSON</el-dropdown-item>
                        <el-dropdown-item command="share">分享链接</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </div>
        </div>

        <!-- </div> -->
        <div class="filter">
            <div class="filter-item">
                <select-artifact-set v-model="filterSet" :multiple="true" :multiple-limit="1000"
                    placeholder="套装"
                ></select-artifact-set>
            </div>

            <div class="filter-item">
                <select-artifact-main-stat
                    v-model="filterMainStat"
                    :include-any="false"
                    :multiple="true"
                    placeholder="主词条"
                ></select-artifact-main-stat>
            </div>

            <el-checkbox v-model="ge16" class="show-only-16">只显示16级以上</el-checkbox>
        </div>

        <!-- artifacts display -->
        <el-tabs v-model="activeName">
            <el-tab-pane v-for="tab in tabs" :key="tab.name" class="panel" :name="tab.name">
                <div slot="label">
                    <img :src="tab.icon" class="icon" />
                </div>

                <div v-if="filteredArtifacts.length > 0">
                    <div class="artifacts-div mona-scroll-hidden">
                        <artifact-display
                            class="artifact-item"
                            v-for="item in artifactToBeDisplayed"
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
import {mapGetters} from 'vuex';
import {
    getArtifactsRecommendation,
    importMonaJson,
    removeArtifact,
    toggleArtifact,
    updateArtifact,
} from '@util/artifacts';
import {positions} from '@const/misc';
import {downloadString} from '@util/common';
import {deviceIsPC} from "@util/device"
import { getRepo, createRepo } from "@/api/repo"

import flowerIcon from '@image/misc/flower.png';
import featherIcon from '@image/misc/feather.png';
import sandIcon from '@image/misc/sand.png';
import gobletIcon from '@image/misc/goblet.png';
import headIcon from '@image/misc/head.png';

import AddArtifactDialog from './AddArtifactDialog';
import YasUiDialog from './YasUIDialog';
import SelectArtifactSet from '@c/select/SelectArtifactSet';
import SelectArtifactMainStat from '@c/select/SelectArtifactMainStat';
import ArtifactDisplay from '@c/display/ArtifactDisplay';
import EditArtifact from './EditArtifact';
import ImportBlock from '@c/misc/ImportBlock';

const tabs = [
    { icon: flowerIcon, name: 'flower' },
    { icon: featherIcon, name: 'feather' },
    { icon: sandIcon, name: 'sand' },
    { icon: gobletIcon, name: 'cup' },
    { icon: headIcon, name: 'head' },
];
Object.freeze(tabs);

const pageSize = 20;

export default {
    name: 'ArtifactsPage',
    components: {
        ImportBlock,
        AddArtifactDialog,
        SelectArtifactSet,
        SelectArtifactMainStat,
        ArtifactDisplay,
        EditArtifact,
        YasUiDialog,
    },
    created: function () {
        this.tabs = tabs;
        this.pageSize = pageSize;
        this.deviceIsPC = deviceIsPC
    },
    data: function () {
        return {
            activeName: 'flower',

            newDialogVisible: false,
            showEditArtifactDrawer: false,
            showImportDialog: false,
            showYasUIDialog: false,
            showArtifactRecommendationDrawer: false,
            showOutputShareDialog: false,

            recommendationList: [],
            recommendationInCalculation: false,

            filterSet: [],
            filterMainStat: [],
            ge16: true,
            // currentPage: 1,

            importDeleteUnseen: false,

            loading: false,
            shareLink: ""
        };
    },
    beforeRouteEnter(to, from, next) {
        next(vm => {
            const code = to.query.code
            console.log(code)

            if (code) {
                vm.loading = true
                getRepo(code).then(response => {
                    if (response.status === 200) {
                        const data = response.data
                        const content = data.content

                        const doImport = () => {
                            // vm.$notify.info({
                            //     title: "正在导入",
                            //     message: "检测到可导入内容，正在执行导入"
                            // })
                            setImmediate(() => {
                                vm.importJson(content)
                                vm.loading = false
                            })
                            // vm.importJson(content)
                        }

                        const artifactsCount = vm.$store.getters["artifacts/count"]
                        if (artifactsCount > 0) {
                            vm.$confirm("本地已经存在圣遗物，是否继续导入", "提示", {
                                confirmButtonText: "继续",
                                cancelButtonText: "取消",
                                type: "warning"
                            }).then(() => {
                                doImport()
                            }).catch(() => {})
                        } else {
                            // console.log(123)
                            doImport()
                        }
                    }
                }).finally(() => {
                    // vm.loading = false
                })
            }
        })
    },
    methods: {
        unlockAllArtifacts() {
            this.$store.commit('artifacts/unlockAll')
        },

        handleCopyShareLink() {
            if (window.navigator) {
                navigator.clipboard.writeText(this.shareLink)
                this.$message.success("复制成功")
                this.showOutputShareDialog = false
            }
        },

        handleDropdownCommand(command) {
            switch (command) {
                case "add":
                    this.add()
                    break
                case "deleteAll":
                    this.$confirm("确实删除所有圣遗物？（将同时删除所有套装）", "提示", {
                        confirmButtonText: "确定",
                        cancelButtonText: "取消",
                        type: "warning"
                    }).then(() => {
                        this.handleClickDeleteAll()
                    }).catch(() => {})
                    break
                case "unlockAll":
                    this.unlockAllArtifacts()
                    break
                case "recommend":
                    this.handleClickRecommendation()
                    break
                case "importJson":
                    this.handleImportJsonClicked()
                    break
                case "exportJson":
                    this.handleOutputJsonClicked()
                    break
                case "exportShare":
                    this.shareArtifact()
                    break
            }
        },

        handleClickDeleteAll() {
            this.$store.commit('artifacts/removeAllArtifacts');
        },

        handleClickRemoveArtifact(id) {
            removeArtifact(id);
        },

        handleClickToggleArtifact(id) {
            toggleArtifact(id);
        },

        handleClickEditArtifact(id) {
            // console.log(id)
            this.showEditArtifactDrawer = true;

            this.$nextTick(() => {
                let component = this.$refs['editArtifactDrawer'];
                if (!component) {
                    return;
                }
                component.setId(id);
            });
        },

        handleConfirmEdit(id) {
            let component = this.$refs['editArtifactDrawer'];
            if (!component) {
                return;
            }
            let newArtifact = component.getNewArtifact();

            updateArtifact(id, newArtifact);

            this.showEditArtifactDrawer = false;
        },

        add: function () {
            this.newDialogVisible = true;
        },

        handleAddArtifact: function (item) {
            this.newDialogVisible = false;

            this.activeName = item.position;

            this.$store.commit('artifacts/addArtifact', item);
        },

        handleImportJsonClicked() {
            this.showImportDialog = true;
        },

        handleYasUIClicked() {
            this.showYasUIDialog = true;
        },

        async importJson(text, deleteUnseen) {
            try {
                const rawObj = JSON.parse(text)
                await importMonaJson(rawObj, deleteUnseen)
            } catch (e) {
                this.$message.error("格式不正确")
            }
        },

        handleImportJson() {
            const component = this.$refs.fileUploader;
            if (!component) {
                return;
            }

            const loading = this.$loading({
                lock: true,
                text: '导入中',
            });

            component
                .getReadPromise()
                .then((text) => {
                    this.importJson(text, this.importDeleteUnseen)
                })
                .catch((e) => {
                    this.$message.error(e);
                })
                .finally(() => {
                    loading.close();
                });
        },

        getArtifactString() {
            let temp = {
                version: '1',
            }

            for (let position in positions) {
                temp[position] = this.$store.state.artifacts[position]
            }

            return JSON.stringify(temp)
        },

        handleOutputJsonClicked() {
            const str = this.getArtifactString()
            downloadString(str, 'application/json', 'artifacts_mona');
        },

        shareArtifact() {
            this.$notify.info({
                title: "创建中",
                message: "莫娜正在创建分享链接",
                duration: 2000
            })

            const str = this.getArtifactString()
            createRepo(str).then(response => {
                // console.log(response)
                if (response.status === 200) {
                    // console.log("success")
                    const code = response.data.code
                    const link = `https://mona-uranai.com/artifacts?code=${code}`
                    this.shareLink = link
                    this.showOutputShareDialog = true
                }
            })
        },

        handleOutputCommand(command) {
            switch (command) {
                case "monaJson": {
                    this.handleOutputJsonClicked()
                    break
                }
                case "share": {
                    this.shareArtifact()
                    break
                }
            }
        },

        handleClickRecommendation() {
            const presetLength = this.$store.getters['presets/allFlat'].length;
            if (presetLength === 0) {
                this.$message.error('添加计算预设以使用该功能');
                return;
            }

            this.showArtifactRecommendationDrawer = true;

            getArtifactsRecommendation().then((result) => {
                let temp = result.slice(0, 50);
                const maxValue = temp.map((item) => item[1]).reduce((p, c) => Math.max(p, c), 0);

                for (let i = 0; i < temp.length; i++) {
                    temp[i][1] /= maxValue;
                }

                this.recommendationList = temp;
            });
        },
    },
    computed: {
        ...mapGetters('artifacts', ['allArtifacts', 'artifactsById', 'count']),

        artifactsCurrentSlotFlat() {
            const items = this.allArtifacts[this.activeName];
            return items;
        },

        filteredArtifacts() {
            let results = [];

            for (let artifact of this.artifactsCurrentSlotFlat) {
                const setName = artifact.setName;
                const mainStatName = artifact.mainTag.name;
                const level = artifact.level ?? 20;

                if (this.filterSet.length > 0 && this.filterSet.indexOf(setName) === -1) {
                    continue;
                }
                if (this.filterMainStat.length > 0 && this.filterMainStat.indexOf(mainStatName) === -1) {
                    continue;
                }
                if (this.ge16 && level < 16) {
                    continue;
                }

                results.push(artifact);
            }

            return results;
        },

        artifactToBeDisplayed() {
            // return this.artifactsCurrentSlotFlat
            return this.filteredArtifacts;
            // const start = (this.currentPage - 1) * pageSize
            // const end = Math.min(start + pageSize, this.filteredArtifacts.length)
            //
            // return this.filteredArtifacts.slice(start, end)
        },
    },
};
</script>

<style scoped lang="scss">
@media only screen and (min-width: 992px) {
    .filter {
        margin-bottom: 12px;
        display: flex;
        align-items: center;

        .filter-item {
            display: inline-block;
            vertical-align: top;
        }

        .show-only-16 {
            margin-left: 16px;
        }
    }
}

@media only screen and (max-width: 992px) {
    .filter {
        margin-bottom: 12px;

        .filter-item {
            .el-select {
                width: 100%;
            }
        }

        .show-only-16 {
            margin-top: 4px;
        }
    }


}

.toolbar-mobile {
    position: relative;
    .m-center {
        position: absolute;
        left: 50%;
        transform: translateX(-50%) translateY(-50%);
        top: 50%;
        //top: 0;
        font-size: 12px;
        color: #606166;
    }
}

.icon {
    padding: 0 12px;
}

.artifacts-div {
    box-sizing: border-box;
    //height: 100vh;
    display: grid;
    gap: 4px;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    grid-auto-columns: 1fr;
    grid-auto-rows: min-content;

    .artifact-item {
        width: 100%;
        box-sizing: border-box;
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
