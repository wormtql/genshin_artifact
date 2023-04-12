<template>
    <div>
        <!-- new artifact dialog -->
        <add-artifact-dialog
            v-model="newDialogVisible"
            @confirm="handleConfirmAddArtifact"
        ></add-artifact-dialog>

        <yas-ui-dialog v-model:visible="showYasUIDialog"></yas-ui-dialog>

        <el-dialog
            v-model="showOutputShareDialog"
            title="分享"
            :width="deviceIsPC ? '500px' : '90%'"
        >
            <p>{{ t("artPage.shareDesc") }}</p>
            <el-input v-model="shareLink"></el-input>

            <template #footer>
                <el-button type="primary" @click="handleCopyShareLink">{{ t("misc.copy") }}</el-button>
            </template>
        </el-dialog>

        <el-dialog v-model="showImportDialog" :title="t('misc.import')" :width="deviceIsPC ? '60%' : '90%'">
            <import-block ref="fileUploader" accept="application/json"></import-block>
            <el-checkbox v-model="importDeleteUnseen" style="margin-top: 12px">{{ t("artPage.deleteUnseen") }}</el-checkbox>
            <el-checkbox v-model="importBackupKumiDir" style="margin-top: 12px">{{ t("artPage.backupKumiDir") }}</el-checkbox>

            <template #footer>
                <el-button @click="showImportDialog = false">{{ t("misc.cancel") }}</el-button>
                <el-button type="primary" @click="handleImportJson">{{ t("misc.confirm") }}</el-button>
            </template>
        </el-dialog>

        <el-drawer :title="t('artPage.edit')" v-model="showEditArtifactDrawer" direction="rtl" :size="deviceIsPC ? '30%' : '100%'">
            <edit-artifact
                ref="editArtifactComponent"
                @confirm="handleConfirmEdit"
                @cancel="showEditArtifactDrawer = false"
            ></edit-artifact>
        </el-drawer>

        <el-drawer :title="t('artPage.recommend')" v-model="showArtifactRecommendationDrawer" :size="deviceIsPC ? '30%' : '100%'">
            <el-empty v-if="recommendationList.length === 0"></el-empty>
            <div v-else>
                <artifact-display
                    v-for="item in recommendationList"
                    :key="item[0]"
                    :item="artifactStore.artifacts.value.get(item[0])"
                    style="width: 100%; margin-bottom: 16px"
                    :show-back="true"
                    :back-value="item[1]"
                ></artifact-display>
            </div>
        </el-drawer>

        <el-breadcrumb class="hidden-sm-and-down">
            <el-breadcrumb-item>{{ t("misc.artifact") }}（{{ artifactStore.artifactsCount }}）</el-breadcrumb-item>
        </el-breadcrumb>

        <div class="toolbar-mobile hidden-md-and-up" style="margin-bottom: 12px">
            <el-button-group>
                <el-button
                    @click="handleClickAddArtifact"
                    type="primary"
                    :icon="IconEpPlus"
                    size="small"
                ></el-button>

                <el-dropdown @command="handleDropdownCommand" trigger="click">
                    <el-button size="small" :icon="IconEpMore"></el-button>
                    <template #dropdown>
                        <el-dropdown-menu>
                            <el-dropdown-item command="deleteAll" :icon="IconEpDelete">{{ t("misc.clear") }}</el-dropdown-item>
                            <el-dropdown-item divided command="unlockAll" :icon="IconEpUnlock">{{ t("artPage.unlockAll") }}</el-dropdown-item>
                            <el-dropdown-item divided command="recommend" :icon="IconFa6Lightbulb">{{ t("misc.recommend") }}</el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>

                <el-dropdown trigger="click" @command="handleDropdownCommand">
                    <el-button size="small" :icon="IconFa6PaperPlane"></el-button>

                    <template #dropdown>
                        <el-dropdown-menu>
                            <el-dropdown-item command="importJson" :icon="IconFa6SolidUpload">{{ t("misc.import") }}</el-dropdown-item>

                            <el-dropdown-item divided command="exportJson" :icon="IconFa6SolidDownload">{{ t("artPage.exportMona") }}</el-dropdown-item>
                            <el-dropdown-item command="exportShare" :icon="IconFa6SolidShareNodes">{{ t("artPage.shareLink") }}</el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>
            </el-button-group>

            <div class="m-center">{{ t("misc.total") }}：{{ artifactStore.artifactsCount }}</div>
        </div>

        <div class="tool-bar hidden-sm-and-down">
            <el-button-group>
                <el-button
                    @click="handleClickAddArtifact"
                    type="primary"
                    :icon="IconEpPlus"
                    size="small"
                ></el-button>

                <el-popconfirm
                    :title="t('artPage.confirmClear')"
                    @confirm="deleteAllArtifacts"
                    style="margin-right: 8px"
                >
                    <template #reference>
                        <el-button size="small" :icon="IconEpDelete" type="danger" :title="t('misc.clear')">
                            {{ t("misc.clear") }}
                        </el-button>
                    </template>

                </el-popconfirm>

                <el-button size="small" :icon="IconEpUnlock" :title="t('artPage.unlockAll')" @click="unlockAllArtifacts">
                    {{ t("artPage.unlockAll") }}</el-button>

                <el-button size="small" :icon="IconFa6Lightbulb" @click="handleClickRecommendation">{{ t("misc.recommend") }}</el-button>
            </el-button-group>


            <div class="tool-right">
                <el-button-group>
                    <el-button @click="handleYasUIClicked" size="small" type="primary" v-if="deviceIsPC">{{ t("misc.scan") }}</el-button>
                    <el-button @click="handleImportJsonClicked" size="small" type="primary">{{ t("misc.import") }}</el-button>

                    <el-dropdown split-button size="small" @click="handleOutputJsonClicked" @command="handleOutputCommand">
                        {{ t("misc.export") }}
                        <template #dropdown>
                            <el-dropdown-menu>
                                <el-dropdown-item command="monaJson">{{ t("artPage.monaJSON") }}</el-dropdown-item>
                                <el-dropdown-item command="share">{{ t("artPage.shareLink") }}</el-dropdown-item>
                            </el-dropdown-menu>
                        </template>
                    </el-dropdown>
                </el-button-group>

<!--                <el-button @click="handleOutputJsonClicked" size="small"> 导出 </el-button>-->


            </div>
        </div>

        <!-- </div> -->
        <div class="filter">
            <div class="filter-item">
                <select-artifact-set v-model="filterSet" :multiple="true" :multiple-limit="1000"
                    :placeholder="t('misc.artifactSet')"
                ></select-artifact-set>
            </div>

            <div class="filter-item">
                <select-artifact-main-stat
                    v-model="filterMainStat"
                    :include-any="false"
                    :multiple="true"
                    :placeholder="t('misc.mainStat')"
                ></select-artifact-main-stat>
            </div>

            <el-checkbox v-model="filterGe16" class="show-only-16">{{ t("artPage.show16") }}</el-checkbox>
        </div>

        <!-- artifacts display -->
        <el-tabs v-model="activeName">
            <el-tab-pane v-for="tab in tabs" :key="tab.name" class="panel" :name="tab.name">
                <template #label>
                    <img :src="tab.icon" class="icon" />
                </template>

                <div v-if="filteredArtifacts.length > 0 && activeName === tab.name">
                    <div class="artifacts-div mona-scroll-hidden">
                        <artifact-display
                            class="artifact-item"
                            v-for="item in artifactToBeDisplayed"
                            :key="item.id"
                            :item="item"
                            :buttons="true"
                            :delete-button="true"
                            :edit-button="true"
                            @delete="deleteArtifact(item.id)"
                            @toggle="toggleArtifact(item.id)"
                            @edit="handleClickEditArtifact(item.id)"
                        ></artifact-display>
                    </div>
                </div>
                <div v-else>
                    <el-empty></el-empty>
                </div>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script setup lang="ts">
import {importMonaJson} from '@/utils/artifacts'
import {positions} from '@/constants/artifact'
import {downloadString} from '@/utils/common'
import {deviceIsPC} from "@/utils/device"
import {createRepo} from "@/api/repo"
import {type Ref} from "vue"
import {useArtifactStore} from "@/store/pinia/artifact"
import {usePresetStore} from "@/store/pinia/preset"
import {getArtifactsRecommendation} from "@/utils/artifactRecommendation"

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
import {type ArtifactPosition, ArtifactSetName, ArtifactStatName, IArtifactContentOnly} from "@/types/artifact"
// import {ElLoading, ElMessage, ElMessageBox, ElNotification} from "element-plus"
// import {ElMessage} from "element-plus"

import IconEpPlus from "~icons/ep/plus"
import IconEpUnlock from "~icons/ep/unlock"
import IconFa6Lightbulb from "~icons/fa6-regular/lightbulb"
import IconEpMore from "~icons/ep/more"
import IconEpDelete from "~icons/ep/delete"
import IconFa6PaperPlane from "~icons/fa6-regular/paper-plane"
import IconFa6SolidUpload from "~icons/fa6-solid/upload"
import IconFa6SolidShareNodes from "~icons/fa6-solid/share-nodes"
import IconFa6SolidDownload from "~icons/fa6-solid/download"
import {useI18n} from "@/i18n/i18n"

import "element-plus/es/components/message/style/css"

// i18n
const { t } = useI18n()


const tabs = [
    { icon: flowerIcon, name: 'flower' },
    { icon: featherIcon, name: 'feather' },
    { icon: sandIcon, name: 'sand' },
    { icon: gobletIcon, name: 'cup' },
    { icon: headIcon, name: 'head' },
];
Object.freeze(tabs);
const activeName = ref("flower" as ArtifactPosition)

const pageSize = 20;

const artifactStore = useArtifactStore()
const presetStore = usePresetStore()

///////////////////////////////////////////////////////////////////
// artifact crud
function unlockAllArtifacts() {
    artifactStore.unlockAll()
}

function deleteAllArtifacts() {
    artifactStore.deleteAll()
}

function removeArtifact(id: number) {
    artifactStore.removeArtifact(id)
}

function toggleArtifact(id: number) {
    artifactStore.toggleArtifact(id)
}

function deleteArtifact(id: number) {
    artifactStore.removeArtifact(id)
}


///////////////////////////////////////////////////////////////////
// artifacts filters and display
const filterSet: Ref<ArtifactSetName[]> = ref([])
const filterMainStat: Ref<ArtifactStatName[]> = ref([])
const filterGe16 = ref(true)

const artifactsCurrentSlot = computed(() => {
    return artifactStore.artifactsByPosition.value[activeName.value];
})

const filteredArtifacts = computed(() => {
    let results = [];

    for (let artifact of artifactsCurrentSlot.value) {
        const setName = artifact.setName;
        const mainStatName = artifact.mainTag.name;
        const level = artifact.level;

        if (filterSet.value.length > 0 && filterSet.value.indexOf(setName) === -1) {
            continue;
        }
        if (filterMainStat.value.length > 0 && filterMainStat.value.indexOf(mainStatName) === -1) {
            continue;
        }
        if (filterGe16.value && level < 16) {
            continue;
        }

        results.push(artifact);
    }

    return results;
})

const artifactToBeDisplayed = computed(() => {
    return filteredArtifacts.value
})


///////////////////////////////////////////////////////////////////
// new artifact
const newDialogVisible = ref(false)

function handleClickAddArtifact() {
    newDialogVisible.value = true
}

function handleConfirmAddArtifact(a: IArtifactContentOnly) {
    const position = a.position
    newDialogVisible.value = false
    artifactStore.addArtifact(a)
    activeName.value = position
}


///////////////////////////////////////////////////////////////////
// YAS-UI
const showYasUIDialog = ref(false)

function handleYasUIClicked() {
    showYasUIDialog.value = true
}


///////////////////////////////////////////////////////////////////
// share
const showOutputShareDialog = ref(false)
const shareLink = ref("")

function handleCopyShareLink() {
    if (window.navigator) {
        navigator.clipboard.writeText(shareLink.value)
        ElMessage({
            message: t("artPage.copied"),
            type: "success"
        })
        showOutputShareDialog.value = false
    }
}

function shareArtifact() {
    ElNotification({
        title: t("artPage.creating"),
        message: t("artPage.createDesc"),
        duration: 2000
    })

    const str = getArtifactString()
    createRepo(str).then(response => {
        // console.log(response)
        if (response.status === 200) {
            // console.log("success")
            const code = response.data.code
            // shareLink.value = `https://mona-uranai.com/artifacts?code=${code}`
            shareLink.value = `https://mona-uranai.com/import?type=artifact&code=${code}`
            showOutputShareDialog.value = true
        }
    })
}


///////////////////////////////////////////////////////////////////
// import/export artifacts
const showImportDialog = ref(false)
const fileUploader: Ref<InstanceType<typeof ImportBlock> | null> = ref(null)
const importDeleteUnseen = ref(false)
const importBackupKumiDir = ref(false)

function handleImportJsonClicked() {
    showImportDialog.value = true;
}

async function importJson(text: string, deleteUnseen: boolean, backupKumiDir: boolean) {
    try {
        const rawObj = JSON.parse(text)
        await importMonaJson(rawObj, deleteUnseen, backupKumiDir)
    } catch (e) {
        ElMessage({
            message: t("artPage.wrongFormat"),
            type: "error"
        })
    }
}

function handleImportJson() {
    const component = fileUploader.value;
    if (!component) {
        return;
    }

    const loading = ElLoading.service({
        lock: true,
        text: t("artPage.importing")
    })

    if (fileUploader.value) {
        fileUploader.value
            .getReadPromise()
            .then((text: string) => {
                importJson(text, importDeleteUnseen.value, importBackupKumiDir.value)
            })
            .catch((e: any) => {
                ElMessage({
                    message: e,
                    type: "error"
                })
            })
            .finally(() => {
                loading.close()
                showImportDialog.value = false
            });
    }
}

function getArtifactString(): string {
    let temp = {
        version: '1',
        flower: [],
        sand: [],
        feather: [],
        cup: [],
        head: []
    } as any

    for (let position of positions) {
        temp[position] = artifactStore.artifactsByPosition.value[position]
    }

    return JSON.stringify(temp)
}

function handleOutputJsonClicked() {
    const str = getArtifactString()
    downloadString(str, 'application/json', 'artifacts_mona');
}


///////////////////////////////////////////////////////////////////
// edit artifacts
const showEditArtifactDrawer = ref(false)
const editArtifactComponent: Ref<InstanceType<typeof EditArtifact> | null> = ref(null)

function handleClickEditArtifact(id: number) {
    showEditArtifactDrawer.value = true;

    nextTick(() => {
        if (editArtifactComponent.value) {
            editArtifactComponent.value.setId(id)
        }
    });
}

function handleConfirmEdit(id: number) {
    if (editArtifactComponent.value) {
        const newArtifact: IArtifactContentOnly = editArtifactComponent.value.getNewArtifact() as IArtifactContentOnly
        artifactStore.updateArtifact(id, newArtifact)
    }

    showEditArtifactDrawer.value = false
}


///////////////////////////////////////////////////////////////////
// artifacts recommendation
const showArtifactRecommendationDrawer = ref(false)
const recommendationList: Ref<[number, number][]> = ref([])
const recommendationInCalculation = ref(false)


function handleClickRecommendation() {
    if (presetStore.allFlat.value.length === 0) {
        ElMessage.error({
            message: t("artPage.msg1")
        })
        return
    }

    showArtifactRecommendationDrawer.value = true;

    getArtifactsRecommendation().then((result) => {
        let temp = result.slice(0, 50);
        const maxValue = temp.map((item) => item[1]).reduce((p, c) => Math.max(p, c), 0);

        for (let i = 0; i < temp.length; i++) {
            temp[i][1] /= maxValue;
        }

        recommendationList.value = temp as [number, number][];
    });
}


// commands
function handleDropdownCommand(command: string) {
    switch (command) {
        case "add":
            handleClickAddArtifact()
            break
        case "deleteAll":
            ElMessageBox.confirm(
                t("artPage.confirmClear"),
                t("misc.hint"),
                {
                    confirmButtonText: t("misc.confirm"),
                    cancelButtonText: t("misc.cancel"),
                    type: "warning"
                }
            ).then(() => {
                deleteAllArtifacts()
            }).catch(() => {})
            break
        case "unlockAll":
            unlockAllArtifacts()
            break
        case "recommend":
            handleClickRecommendation()
            break
        case "importJson":
            handleImportJsonClicked()
            break
        case "exportJson":
            handleOutputJsonClicked()
            break
        case "exportShare":
            shareArtifact()
            break
    }
}

function handleOutputCommand(command: string) {
    switch (command) {
        case "monaJson": {
            handleOutputJsonClicked()
            break
        }
        case "share": {
            shareArtifact()
            break
        }
    }
}
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
