<template>
    <div>
        <el-dialog
            v-model="showSelectArtifactDialog"
            title="选择圣遗物"
            :width="deviceIsPC ? '80%' : '90%'"
        >
            <select-artifact
                :position="artifactPositionToBeSelected"
                @select="handleSelectArtifact"
            ></select-artifact>
        </el-dialog>

        <div class="m-toolbar hidden-md-and-up">
            <el-dropdown trigger="click" @command="handleCommand">
                <el-button :icon="IconEpMore" size="small" text circle></el-button>

                <template #dropdown>
                    <el-dropdown-menu>
                        <el-dropdown-item command="add" :icon="IconEpFolderAdd">新建收藏夹</el-dropdown-item>
                        <el-dropdown-item command="addKumi" :icon="IconEpPlus">新建圣遗物组</el-dropdown-item>
                        <el-dropdown-item command="delete" v-if="currentDirId !== 0" :icon="IconEpDelete">删除"{{ currentDirTitle }}"</el-dropdown-item>
                        <el-dropdown-item command="rename" v-if="currentDirId !== 0" :icon="IconEpEdit">重命名</el-dropdown-item>
                        <el-dropdown-item
                            v-for="(item, index) in dirItems"
                            :key="item.index"
                            :divided="index === 0"
                            :command="'use ' + item.index"
                            :icon="IconEpFolder"
                        >{{ item.title }}</el-dropdown-item>
                    </el-dropdown-menu>
                </template>
            </el-dropdown>

            <div class="m-middle">
                {{ currentDirTitle }}
            </div>
        </div>

        <div class="pc-container">
            <div class="left mona-scroll-hidden hidden-sm-and-down">
                <div class="toolbar">
                    <el-button type="primary" :icon="IconEpPlus" @click="handleNewDir">
                        新建收藏夹
                    </el-button>
                </div>

                <div class="mona-scroll-hidden left-list">
                    <my-list
                        v-model="currentDirId"
                        :items="dirItems"
                    ></my-list>
                </div>
            </div>
            <div class="right mona-scroll-hidden">
                <div class="hidden-md-and-up">
                    <el-input
                        v-model="searchString"
                        style="margin-bottom: 12px"
                        placeholder="搜索"
                    >
                        <template #append>
                            <i-ep-search />
                        </template>
                    </el-input>
                </div>
                <div class="toolbar2 hidden-sm-and-down">
                    <div class="button-left">
                        <el-button
                            type="primary"
                            :icon="IconEpPlus"
                            @click="handleNewKumi"
                        >新建组</el-button>
                        <el-input
                            v-model="searchString"
                            style="margin-left: 16px"
                            placeholder="搜索"
                        >
                            <template #append>
                                <el-icon><i-ep-search /></el-icon>
                            </template>
                        </el-input>
                    </div>

                    <div class="button-right">
                        <el-button-group>
                            <el-button
                                v-if="currentDirId !== 0"
                                :icon="IconEpEdit"
                                @click="handleRenameDir(currentDirId)"
                            >重命名</el-button>
                            <el-button
                                type="danger"
                                @click="handleDeleteDir(currentDirId)"
                                v-if="currentDirId !== 0"
                                title="删除收藏夹"
                                :icon="IconEpDelete"
                            ></el-button>

                        </el-button-group>

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
        
    </div>
</template>

<script setup lang="ts">
import Fuse from "fuse.js"

import MyList from "@/components/misc/MyList"
import KumiItem from "./KumiItem"
import SelectArtifact from "@/components/select/SelectArtifact"
import {useKumiStore} from "@/store/pinia/kumi"
import {ElMessageBox} from "element-plus"
import type {KumiItem as TypeKumiItem} from "@/types/kumi"
import {deviceIsPC} from "@/utils/device"

import IconEpPlus from "~icons/ep/plus"
import IconEpDelete from "~icons/ep/delete"
import IconEpEdit from "~icons/ep/edit"
import IconEpMore from "~icons/ep/more"
import IconEpFolder from "~icons/ep/folder"
import IconEpFolderAdd from "~icons/ep/folder-add"
import type {ArtifactPosition} from "@/types/artifact"


const kumiStore = useKumiStore()


// dirs
interface DirItem {
    index: number,
    title: string
}

const currentDirId = ref(0)

const dirItems = computed((): DirItem[] => {
    let temp = []
    for (let dir of kumiStore.dirs.value) {
        temp.push({
            index: dir.id,
            title: dir.title
        })
    }
    // console.log(temp)
    return temp
})

const currentDirTitle = computed(() => {
    const item = kumiStore.itemById(currentDirId.value)
    if (item) {
        return item.title
    }

    return "title"
})

function handleNewDir() {
    ElMessageBox.prompt("请输入新收藏夹名", "新建收藏夹", {
        confirmButtonText: "确定",
        cancelButtonText: "取消"
    }).then(({ value }) => {
        if (value) {
            kumiStore.createDir(value)
        }
    }).catch(() => {})
}

function handleDeleteDir(id: number) {
    if (id === 0) {
        return
    }
    kumiStore.deleteDir(id)
}

function handleRenameDir(id: number) {
    ElMessageBox.prompt("请输入新收藏夹名", "重命名收藏夹", {
        confirmButtonText: "确定",
        cancelButtonText: "取消"
    }).then(({ value }) => {
        if (value) {
            kumiStore.rename(id, value)
        }
    }).catch(() => {})
}


// kumis
const currentKumiList = computed((): TypeKumiItem[] => {
    return kumiStore.kumisByDirId.value[currentDirId.value]
})

function handleNewKumi() {
    ElMessageBox.prompt("请输入新圣遗物组名", "新建圣遗物组", {
        confirmButtonText: "确定",
        cancelButtonText: "取消"
    }).then(({ value }) => {
        if (value) {
            kumiStore.createKumi(currentDirId.value, value)
        }
    }).catch(() => {})
}

function handleDeleteKumi(id: number) {
    kumiStore.deleteKumi(id)
}

function handleEditKumi(id: number) {
    ElMessageBox.prompt("请输入新圣遗物组名", "重命名圣遗物组", {
        confirmButtonText: "确定",
        cancelButtonText: "取消"
    }).then(({ value }) => {
        if (value) {
            kumiStore.rename(id, value)
        }
    }).catch(() => {})
}


// search
const searchString = ref("")
const fuse = computed(() => {
    return new Fuse(currentKumiList.value, {
        keys: ["title"]
    })
})

const filteredKumiList = computed((): TypeKumiItem[] => {
    if (searchString.value === "") {
        return currentKumiList.value
    }
    const results = fuse.value.search(searchString.value)
    return results.map(x => x.item)
})


// artifact
const showSelectArtifactDialog = ref(false)
const artifactPositionToBeSelected = ref<ArtifactPosition>("flower")
const underSelectionKumiId = ref(0)

function handleSelectArtifact(id: number) {
    showSelectArtifactDialog.value = false
    kumiStore.addArtifact(underSelectionKumiId.value, id)
}

function handleDeleteArtifact(kumiId: number, artifactId: number) {
    kumiStore.deleteArtifact(kumiId, artifactId)
}

function handleClickPosition(kumiId: number, position: ArtifactPosition) {
    artifactPositionToBeSelected.value = position
    showSelectArtifactDialog.value = true
    underSelectionKumiId.value = kumiId
}


// command
function handleCommand(command: string) {
    if (command === "add") {
        handleNewDir()
    } else if (command.startsWith("use")) {
        currentDirId.value = parseInt(command.split(" ")[1])
    } else if (command === "delete") {
        handleDeleteDir(currentDirId.value)
    } else if (command === "rename") {
        handleRenameDir(currentDirId.value)
    } else if (command === "addKumi") {
        handleNewKumi()
    }
}

</script>

<style scoped lang="scss">
.m-toolbar {
    margin-bottom: 12px;
    position: relative;
    .m-middle {
        position: absolute;
        left: 50%;
        top: 50%;
        transform: translate(-50%, -50%);
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