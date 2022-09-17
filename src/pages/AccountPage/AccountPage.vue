<template>
    <div>
        <el-dialog
            title="选择同步基准"
            width="30%"
            v-model="showSyncDialog"
            @closed="handleSyncDialogClosed"
        >
            <div>
                <el-button
                    @click="selectSyncType('local')"
                >
                    浏览器存储（最后修改于{{ new Date(localLastModified).toLocaleString() }}）
                </el-button>
            </div>
            <div>
                <el-button
                    @click="selectSyncType('file')"
                >
                    本地目录存储（最后修改于{{ new Date(fileLastModified).toLocaleString() }}）
                </el-button>
            </div>
        </el-dialog>

        <div class="top-things">
            <el-row>
                <el-col :span="6">
                    <span>账号</span>
                </el-col>
                <el-col :span="18">
                    <div style="float: right">
                        <el-button
                            type="primary"
                            size="small"
                            :icon="IconEpSort"
                            :disabled="accountStore.syncStatus.value !== 'no sync'"
                            @click="handleSync"
                        >
                            {{ accountStore.syncStatus.value !== 'no sync' ? '已' : '' }}同步本地目录
                        </el-button>
                    </div>
                </el-col>
            </el-row>
            <el-divider style="margin: 12px 0"></el-divider>
        </div>

        <div class="toolbar">
            <el-button
                type="primary"
                :icon="IconEpPlus"
                size="small"
                @click="addAccount"
            >
                添加账号
            </el-button>
        </div>

        <div class="body">
            <div
                v-for="{id, name} in accountStore.allAccounts"
                :key="id"
                class="item is-active"
                :class="{ active: id === accountStore.currentAccountId.value }"
                @click="handleChangeAccount(id)"
            >
                <click-edit-label
                    :value="name"
                    @input="newName => handleChangeName(id, name, newName)"
                    fontsize="16px"
                    :editable="true"
                    style="display: inline-block;"
                ></click-edit-label>
                <div class="buttons flex-row">
                    <el-popconfirm
                        title="确定删除？"
                        @confirm="handleDeleteAccount(id)"
                    >
                        <template #reference>
                            <el-button
                                v-show="id !== accountStore.currentAccountId.value"
                                :icon="IconEpDelete"
                                type="danger"
                                link
                                size="default"
                                circle
                                class="button"
                                title="删除"
                                @click.stop=""
                            ></el-button>
                        </template>
                    </el-popconfirm>
                    <!-- <el-button
                        icon="el-icon-download"
                        type="text"
                        size="medium"
                        circle
                        @click="handleDownload(name, item)"
                        class="button"
                        title="导出"
                    ></el-button> -->
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ElMessage } from "element-plus"
import ClickEditLabel from "@c/misc/ClickEditLabel"
import IconEpPlus from "~icons/ep/plus"
import IconEpSort from "~icons/ep/sort"
import IconEpDelete from "~icons/ep/delete"

import { useAccountStore, deleteAccount, changeAccount, reload } from "@/store/pinia/account"
import storeBackend, { BackendMeta } from "@/store/backend"

const accountStore = useAccountStore()

function addAccount() {
    accountStore.addAccount('新账户')
}

async function handleDeleteAccount(id: number) {
    const loading = ElLoading.service({
        lock: true,
        text: "删除账号中"
    })
    await deleteAccount(id)
    loading.close()
}

async function handleChangeAccount(id: number) {
    if (id === accountStore.currentAccountId.value) {
        return
    }
    const loading = ElLoading.service({
        lock: true,
        text: "切换账号中"
    })
    await changeAccount(id)
    loading.close()
}

function handleChangeName(id: number, oldName: string, newName: string) {
    if (newName !== oldName && newName !== '') {
        accountStore.changeAccountName(id, newName)
    }
}


const showSyncDialog = ref(false)
const localLastModified = ref(0)
const fileLastModified = ref(0)
const selectSyncType = ref<((t: 'local' | 'file') => void) | null>(null)
const handleSyncDialogClosed = ref<(() => void) | null>(null)

function querySyncType(localMeta: BackendMeta | null, fileMeta: BackendMeta | null) {
    return new Promise<string>((resolve, reject) => {
        if (!fileMeta) {
            resolve('local')
            return
        }
        if (!localMeta) {
            resolve('')
            return
        }
        if (localMeta.lastModified === fileMeta.lastModified) {
            resolve('')
            return
        }
        localLastModified.value = localMeta.lastModified
        fileLastModified.value = fileMeta.lastModified
        let resolved = false
        selectSyncType.value = (type) => {
            resolved = true
            showSyncDialog.value = false
            resolve(type)
        };
        handleSyncDialogClosed.value = () => {
            if (!resolved) {
                storeBackend.disconnectFileBackend()
                reject(Error('取消同步'))
            }
        }
        showSyncDialog.value = true
    })
}

async function handleSync() {
    const metas = await storeBackend.prompt()
    if (!metas) {
        alert('取消同步')
        ElMessage.error('取消同步')  // TODO: not work
        return
    }
    let type: string
    try {
        type = await querySyncType(metas[0], metas[1])
    } catch (err) {
        storeBackend.disconnectFileBackend()
        alert('取消同步')
        ElMessage.error({ message: '取消同步' })
        return
    }
    await storeBackend.sync(type)
    await reload()
    accountStore.syncStatus.value = 'synced'
}

</script>

<style lang="scss" scoped>
.body {
    display: flex;
    flex-wrap: wrap;
}

.toolbar {
    margin-bottom: 20px;
}

$height: 60px;

.item {
    height: $height;
    width: 100%;
    padding-left: 20px;
    cursor: pointer;
    line-height: $height;
    color: #303133;
    transition: 200ms;

    &.active {
        color: #409EFF;
        background: #ecf5ff;
    }

    &:hover {
        background: #ecf5ff;
    }

    .buttons {
        float: right;
        height: $height;

        .button {
            margin: 0;
        }
    }
}
</style>
