<template>
    <div>
<!--        <el-dialog-->
<!--            :title="t('accountPage.chooseSyncBase')"-->
<!--            width="30%"-->
<!--            v-model="showSyncDialog"-->
<!--            @closed="handleSyncDialogClosed"-->
<!--        >-->
<!--            <div>-->
<!--                <el-button-->
<!--                    @click="selectSyncType('local')"-->
<!--                >-->
<!--                    {{ t('accountPage.browserBase') }} ( {{ t('accountPage.lastModifiedAt') + new Date(localLastModified).toLocaleString()}} )-->
<!--                </el-button>-->
<!--            </div>-->
<!--            <div style="margin-top: 10px;">-->
<!--                <el-button-->
<!--                @click="selectSyncType('file')"-->
<!--                >-->
<!--                    {{ t('accountPage.fileBase') }} ( {{ t('accountPage.lastModifiedAt') + new Date(fileLastModified).toLocaleString()}} )-->
<!--                </el-button>-->
<!--            </div>-->
<!--        </el-dialog>-->

<!--        <div class="top-things">-->
<!--            <el-row>-->
<!--                <el-col :span="6">-->
<!--                    <span>{{ t('accountPage.title') }}</span>-->
<!--                </el-col>-->
<!--                <el-col :span="18">-->
<!--                    <div style="float: right">-->
<!--                        <el-button-->
<!--                            type="primary"-->

<!--                            :icon="IconEpSort"-->
<!--                            :disabled="accountStore.syncStatus.value !== 'no sync'"-->
<!--                            @click="handleSync"-->
<!--                        >-->
<!--                            {{ t('accountPage', accountStore.syncStatus.value === 'no sync' ? 'syncButton' : 'syncedButton') }}-->
<!--                        </el-button>-->
<!--                    </div>-->
<!--                </el-col>-->
<!--            </el-row>-->
<!--            <el-divider></el-divider>-->
<!--        </div>-->

        <div class="toolbar">
            <el-button
                type="primary"
                :icon="IconEpPlus"

                @click="addAccount"
            >
                {{ t('accountPage.addAccount') }}
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
                <div class="buttons">
                    <el-popconfirm
                        :title="t('accountPage.confirmDelete')"
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
                                :title="t('accountPage.delete')"
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
import {useI18n} from "@/i18n/i18n"

import { useAccountStore, deleteAccount, changeAccount, reload } from "@/store/pinia/account"
import storeBackend, { BackendMeta } from "@/store/backend"

// i18n
const { t } = useI18n()

const accountStore = useAccountStore()

function addAccount() {
    accountStore.addAccount(t('accountPage.newAccountName'))
}

async function handleDeleteAccount(id: number) {
    const loading = ElLoading.service({
        lock: true,
        text: t('accountPage.deletingAccount')
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
        text: t('accountPage.switchingAccount')
    })

    // ElLoading存在bug，导致滚动条会被拿掉，如果changeAccount太快会导致滚动条突然消失又突然出现，因此这里设置至少切换1s
    let change = changeAccount(id)
    // let timer = new Promise((resolve, reject) => {
    //     setTimeout(() => { resolve(null) }, 1000)
    // })
    // await Promise.all([change, timer])
    await change

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
                reject(new Error(t('accountPage.cancelSyncing')))
            }
        }
        showSyncDialog.value = true
    })
}

async function handleSync() {
    const metas = await storeBackend.prompt()
    if (!metas) {
        // alert(t('accountPage.cancelSyncing'))
        // ElMessage.error(t('accountPage.cancelSyncing'))  // TODO: not work
        return
    }
    let type: string
    try {
        type = await querySyncType(metas[0], metas[1])
    } catch (err) {
        storeBackend.disconnectFileBackend()
        // alert(t('accountPage.cancelSyncing'))
        ElMessage.error({ message: t('accountPage.cancelSyncing') })
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
    padding-right: 20px;
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
