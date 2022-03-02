<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物组</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-dialog
            :visible.sync="showSelectArtifactDialog"
            title="选择圣遗物"
            width="80%"
        >
            <select-artifact
                :position="selectArtifactPosition"
                @select="handleSelectArtifact"
            ></select-artifact>
        </el-dialog>

        <el-row :gutter="16">
            <el-col :span="4" class="left">
                <div class="toolbar">
                    <my-button1
                        icon="el-icon-plus"
                        @click="handleNewDir"
                        title="新建收藏夹"
                    ></my-button1>
                </div>

                <div class="mona-scroll"
                    :style="{ height: contentHeight }"
                >
                    <my-list
                        v-model="currentDirId"
                        :items="items"
                    ></my-list>
                </div>
            </el-col>
            <el-col :span="20" class="right-col">
                <div class="toolbar2">
                    <div class="button-left">
                        <el-button
                            size="mini"
                            type="primary"
                            icon="el-icon-plus"
                            @click="handleNewKumi"
                        >新建组</el-button>
<!--                        <my-button1-->
<!--                            icon="el-icon-plus"-->
<!--                            @click="handleNewKumi"-->
<!--                            title="新建圣遗物组"-->
<!--                        ></my-button1>-->
                        <el-input
                            v-model="searchString"
                            style="margin-left: 16px"
                            size="mini"
                            placeholder="搜索"
                        >
                            <template #append>
                                <i class="el-icon-search"></i>
                            </template>
                        </el-input>
                    </div>

                    <div class="button-right">
                        <el-button
                            size="mini"
                            type="danger"
                            @click="handleDeleteDir(currentDirId)"
                            v-if="currentDirId !== 0"
                            title="删除收藏夹"
                        ><i class="el-icon-delete"></i></el-button>
<!--                        <my-button1-->
<!--                            v-if="currentDirId !== 0"-->
<!--                            icon="el-icon-edit"-->
<!--                            @click="handleRenameDir(currentDirId)"-->
<!--                            title="重命名收藏夹"-->
<!--                        ></my-button1>-->
                        <el-button
                            v-if="currentDirId !== 0"
                            size="mini"
                            icon="el-icon-edit"
                            @click="handleRenameDir(currentDirId)"
                        >重命名</el-button>
                    </div>
                </div>

                <div class="items mona-scroll" ref="content"
                    :style="{ height: contentHeight }"
                >
                    <div v-if="filteredKumiList.length > 0">
                        <kumi-item
                            v-for="kumi in filteredKumiList"
                            :key="kumi.id"
                            :data="kumi"
                            @delete="handleDeleteKumi(kumi.id)"
                            @edit="handleEditKumi(kumi.id)"
                            @click="handleClickPosition(kumi.id, $event)"
                            @deleteArtifact="handleDeleteArtifact(kumi.id, $event)"
                        ></kumi-item>
                    </div>
                    <div v-else>
                        <el-empty></el-empty>
                    </div>
                </div>
<!--                <kumi-directory-display-->
<!--                    :dirId="currentDirId"-->
<!--                    @delete="handleDeleteDir"-->
<!--                ></kumi-directory-display>-->
<!--                {{ currentKumiList }}-->
            </el-col>
        </el-row>
        
    </div>
</template>

<script>
import { mapGetters } from "vuex"
import {
    getKumisByDir,
    newDir,
    checkName,
    checkKumiName,
    newKumi,
    deleteKumi,
    renameItem,
    deleteDir,
    updateKumiArtifact, removeKumiArtifact
} from "@util/kumi"
import Fuse from "fuse.js"

import MyList from "@c/misc/MyList"
import MyButton1 from "@c/button/MyButton1"
import KumiItem from "./KumiItem"
import SelectArtifact from "@c/select/SelectArtifact"

export default {
    name: "KumiPage",
    components: {
        MyButton1,
        MyList,
        KumiItem,
        SelectArtifact
    },
    mounted() {
        const content = this.$refs["content"]
        const rect = content.getBoundingClientRect()

        this.contentHeight = `calc(100vh - ${rect.top}px)`
        // console.log(rect)
    },
    data() {
        return {
            currentDirId: 0,

            showSelectArtifactDialog: false,
            selectArtifactPosition: "flower",
            underSelectionKumiId: 0,

            contentHeight: "100px",
            searchString: ""
        }
    },
    computed: {
        ...mapGetters("kumi", [
            "directories",
            "kumiByDir"
        ]),

        items() {
            let temp = []
            for (let dir of this.directories) {
                temp.push({
                    index: dir.id,
                    title: dir.title
                })
            }
            return temp
        },

        currentKumiList() {
            return getKumisByDir(this.currentDirId)
        },

        fuse() {
            return new Fuse(this.currentKumiList, {
                keys: ["title"]
            })
        },

        filteredKumiList() {
            if (this.searchString === "") {
                return this.currentKumiList
            }
            const results = this.fuse.search(this.searchString)
            return results.map(x => x.item)
        }
    },
    methods: {
        // dir
        handleNewDir() {
            this.getInputDirName("新建", "收藏夹名").then(name => {
                newDir(name)
            }).catch(this.handleInputResult)
        },

        handleDeleteDir(id) {
            if (id === 0) {
                return
            }
            deleteDir(id)
        },

        handleRenameDir(id) {
            this.getInputDirName("重命名", "新收藏夹名").then(name => {
                renameItem(id, name)
            }).catch(this.handleInputResult)
        },

        // kumi
        handleNewKumi() {
            this.getInputKumiName("新建", "圣遗物组名").then(name => {
                newKumi(this.currentDirId, name)
            }).catch(this.handleInputResult)
        },

        handleDeleteKumi(id) {
            deleteKumi(id)
        },

        handleEditKumi(id) {
            this.getInputKumiName("重命名", "新圣遗物组名").then(name => {
                // console.log(name)
                renameItem(id, name)
            }).catch(this.handleInputResult)
        },

        // artifacts
        handleSelectArtifact(id) {
            this.showSelectArtifactDialog = false
            updateKumiArtifact(this.underSelectionKumiId, id)
        },

        handleDeleteArtifact(kumiId, artifactId) {
            // console.log(artifactId)
            removeKumiArtifact(kumiId, artifactId)
        },

        handleClickPosition(kumiId, position) {
            this.selectArtifactPosition = position
            this.showSelectArtifactDialog = true
            this.underSelectionKumiId = kumiId
        },

        // utils
        handleInputResult(result) {
            if (result.error) {
                this.$message.error(result.reason)
            }
        },

        getInputKumiName(title, hint) {
            return new Promise((resolve, reject) => {
                this.$prompt(hint ?? "请输入名称", title ?? "名称", {
                    confirmButtonText: "确定",
                    cancelButtonText: "取消"
                }).then(({ value }) => {
                    const checkResult = checkKumiName(this.currentDirId, value)
                    if (checkResult.error) {
                        reject({
                            error: true,
                            reason: checkResult.reason
                        })
                    } else {
                        resolve(value)
                    }
                }).catch(() => {
                    reject({
                        error: false,
                        reason: null
                    })
                })
            })
        },

        getInputDirName(title, hint) {
            return new Promise((resolve, reject) => {
                this.$prompt(hint ?? "请输入名称", title ?? "名称", {
                    confirmButtonText: "确定",
                    cancelButtonText: "取消"
                }).then(({ value }) => {
                    const checkResult = checkName(value)
                    if (checkResult.error) {
                        reject({
                            error: true,
                            reason: checkResult.reason
                        })
                    } else {
                        resolve(value)
                    }
                }).catch(() => {
                    reject({
                        error: false,
                        reason: null
                    })
                })
            })
        },
    }
}
</script>

<style scoped lang="scss">
.toolbar {
    margin-bottom: 16px;
    display: flex;
    align-items: center;
}

.toolbar2 {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;

    .button-left, .button-right {
        display: flex;
        align-items: center;
    }
}

.right-col {
    //display: flex;
    //flex-direction: column;
    //
    //.items {
    //    flex-grow: 1;
    //}

}

.items {
    overflow: auto;
    padding-right: 20px;
}

/*.left {*/
/*    margin-right*/
/*}*/
</style>