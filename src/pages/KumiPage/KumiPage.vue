<template>
    <div>
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

        <div class="m-toolbar hidden-md-and-up">
            <el-dropdown trigger="click" @command="handleCommand">
                <span><i class="el-icon-more"></i></span>

                <el-dropdown-menu slot="dropdown">
                    <el-dropdown-item command="add"><i class="el-icon-plus"></i>新建收藏夹</el-dropdown-item>
                    <el-dropdown-item command="addKumi"><i class="el-icon-plus"></i>新建圣遗物组</el-dropdown-item>
                    <el-dropdown-item command="delete" v-if="currentDirId !== 0"><i class="el-icon-delete"></i>删除"{{ currentDirTitle }}"</el-dropdown-item>
                    <el-dropdown-item command="rename" v-if="currentDirId !== 0"><i class="el-icon-edit"></i>重命名</el-dropdown-item>
                    <el-dropdown-item
                        v-for="(item, index) in items"
                        :key="item.index"
                        :divided="index === 0"
                        :command="'use ' + item.index"
                    ><i class="el-icon-menu"></i>{{ item.title }}</el-dropdown-item>
                </el-dropdown-menu>
            </el-dropdown>

            <div class="m-middle">
                {{ currentDirTitle }}
            </div>
        </div>

        <div class="pc-container">
            <div class="left mona-scroll-hidden hidden-sm-and-down">
                <div class="toolbar">
                    <el-button size="mini" type="primary" icon="el-icon-plus" @click="handleNewDir">
                        新建收藏夹
                    </el-button>
<!--                    <my-button1-->
<!--                        icon="el-icon-plus"-->
<!--                        @click="handleNewDir"-->
<!--                        title="新建收藏夹"-->
<!--                    ></my-button1>-->
                </div>

                <div class="mona-scroll-hidden left-list">
                    <my-list
                        v-model="currentDirId"
                        :items="items"
                    ></my-list>
                </div>
            </div>
            <div class="right mona-scroll-hidden">
                <div class="hidden-md-and-up">
                    <el-input
                        v-model="searchString"
                        size="mini"
                        style="margin-bottom: 12px"
                        placeholder="搜索"
                    >
                        <template #append>
                            <i class="el-icon-search"></i>
                        </template>
                    </el-input>
                </div>
                <div class="toolbar2 hidden-sm-and-down">
                    <div class="button-left">
                        <el-button
                            size="mini"
                            type="primary"
                            icon="el-icon-plus"
                            @click="handleNewKumi"
                        >新建组</el-button>
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
                        <el-button
                            v-if="currentDirId !== 0"
                            size="mini"
                            icon="el-icon-edit"
                            @click="handleRenameDir(currentDirId)"
                        >重命名</el-button>
                    </div>
                </div>

                <div class="items mona-scroll-hidden">
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
            </div>
        </div>

<!--        <el-row :gutter="16" style="height: 100%">-->
<!--            <el-col :span="4" class="left hidden-sm-and-down">-->
<!--                -->
<!--            </el-col>-->
<!--            <el-col :sm="24" :md="20" class="right-col">-->
<!--                <div class="hidden-md-and-up">-->
<!--                    <el-input-->
<!--                        v-model="searchString"-->
<!--                        size="mini"-->
<!--                        style="margin-bottom: 12px"-->
<!--                        placeholder="搜索"-->
<!--                    >-->
<!--                        <template #append>-->
<!--                            <i class="el-icon-search"></i>-->
<!--                        </template>-->
<!--                    </el-input>-->
<!--                </div>-->
<!--                <div class="toolbar2 hidden-sm-and-down">-->
<!--                    <div class="button-left">-->
<!--                        <el-button-->
<!--                            size="mini"-->
<!--                            type="primary"-->
<!--                            icon="el-icon-plus"-->
<!--                            @click="handleNewKumi"-->
<!--                        >新建组</el-button>-->
<!--                        <el-input-->
<!--                            v-model="searchString"-->
<!--                            style="margin-left: 16px"-->
<!--                            size="mini"-->
<!--                            placeholder="搜索"-->
<!--                        >-->
<!--                            <template #append>-->
<!--                                <i class="el-icon-search"></i>-->
<!--                            </template>-->
<!--                        </el-input>-->
<!--                    </div>-->

<!--                    <div class="button-right">-->
<!--                        <el-button-->
<!--                            size="mini"-->
<!--                            type="danger"-->
<!--                            @click="handleDeleteDir(currentDirId)"-->
<!--                            v-if="currentDirId !== 0"-->
<!--                            title="删除收藏夹"-->
<!--                        ><i class="el-icon-delete"></i></el-button>-->
<!--                        <el-button-->
<!--                            v-if="currentDirId !== 0"-->
<!--                            size="mini"-->
<!--                            icon="el-icon-edit"-->
<!--                            @click="handleRenameDir(currentDirId)"-->
<!--                        >重命名</el-button>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="items mona-scroll-hidden">-->
<!--                    <div v-if="filteredKumiList.length > 0">-->
<!--                        <kumi-item-->
<!--                            v-for="kumi in filteredKumiList"-->
<!--                            :key="kumi.id"-->
<!--                            :data="kumi"-->
<!--                            @delete="handleDeleteKumi(kumi.id)"-->
<!--                            @edit="handleEditKumi(kumi.id)"-->
<!--                            @click="handleClickPosition(kumi.id, $event)"-->
<!--                            @deleteArtifact="handleDeleteArtifact(kumi.id, $event)"-->
<!--                        ></kumi-item>-->
<!--                    </div>-->
<!--                    <div v-else>-->
<!--                        <el-empty></el-empty>-->
<!--                    </div>-->
<!--                </div>-->
<!--            </el-col>-->
<!--        </el-row>-->
        
    </div>
</template>

<script>
import {mapGetters} from "vuex"
import {
    checkKumiName,
    checkName,
    deleteDir,
    deleteKumi,
    getKumisByDir,
    newDir,
    newKumi,
    removeKumiArtifact,
    renameItem,
    updateKumiArtifact
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
    data() {
        return {
            currentDirId: 0,

            showSelectArtifactDialog: false,
            selectArtifactPosition: "flower",
            underSelectionKumiId: 0,

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

        currentDirTitle() {
            for (let dir of this.directories) {
                if (dir.id === this.currentDirId) {
                    return dir.title
                }
            }

            return ""
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
        handleCommand(command) {
            if (command === "add") {
                this.handleNewDir()
            } else if (command.startsWith("use")) {
                this.currentDirId = parseInt(command.split(" ")[1])
            } else if (command === "delete") {
                this.handleDeleteDir(this.currentDirId)
            } else if (command === "rename") {
                this.handleRenameDir(this.currentDirId)
            } else if (command === "addKumi") {
                this.handleNewKumi()
            }
        },

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
.m-toolbar {
    margin-bottom: 12px;
    position: relative;
    .m-middle {
        position: absolute;
        top: 0;
        left: 50%;
        transform: translateX(-50%);
        color: #606166;
        font-size: 12px;
    }
}

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

.left {
    height: 100%;
}

@media only screen and (max-width: 992px) {
    .right-col {
        height: 100vh;
    }
}
//.items {
//    overflow: auto;
//    padding-right: 20px;
//}

@media only screen and (max-width: 992px) {
    .right {

    }
}

@media only screen and (min-width: 992px) {
    .pc-container {
        display: flex;
        height: 100%;
        align-items: flex-start;

        .left {
            width: 200px;
            height: calc(100vh - 2 * 24px);
        }

        .right {
            width: calc(100% - 150px);
            height: calc(100vh - 2 * 24px);
        }
    }
}

/*.left {*/
/*    margin-right*/
/*}*/
</style>