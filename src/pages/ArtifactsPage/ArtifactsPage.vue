<template>
    <div>
        <!-- new artifact dialog -->
        <add-artifact-dialog
            :visible="newDialogVisible"
            @close="newDialogVisible = false"
            @confirm="handleAddArtifact"
        ></add-artifact-dialog>

        <import-json-dialog
            :visible="importJsonDialogVisible"
            @close="importJsonDialogVisible = false"
        >
        </import-json-dialog>

        <output-json-dialog
            :visible="outputJsonDialogVisible"
            @close="outputJsonDialogVisible = false"
        >
        </output-json-dialog>

        <edit-artifact-drawer
            :visible="editArtifactDrawerVisible"
            @close="editArtifactDrawerVisible = false"
            :args="editArtifactArgs"
            ref="editDrawer"
        >
        </edit-artifact-drawer>

        <!-- bread crumb -->
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-alert title="请注意保存圣遗物数据至本地，以防意外导致数据丢失" type="warning" :closable="false"></el-alert>
        <el-alert
            :title="`预计配装计算时间：${estimatedTime}，迭代次数：${$store.getters['artifacts/iterCount']}`"
            type="warning" :closable="false"
        ></el-alert>
        <el-alert title="在同一个浏览器下，正常情况下，数据会自动保存，只需录入一次圣遗物即可。推荐只录入20级圣遗物"></el-alert>
        <el-alert title="小贴士：圣遗物面板上三个按钮作用分别是：删除、禁用/启用、编辑"></el-alert>

        <!-- <el-alert
            type="error"
            v-show="!$store.getters.valid"
            title="圣遗物数量过多，请禁用或者删除明显更次的圣遗物"
            :closable="false"
        >
        </el-alert> -->

        <!-- tool bar -->
        <div class="tool-bar">
            <el-button @click="add"
                type="primary"
                icon="el-icon-plus"
            >
                添加圣遗物
            </el-button>

            <div class="tool-right">
                <el-button @click="handleImportJsonClicked">
                    导入json
                </el-button>
                <el-button @click="handleOutputJsonClicked">
                    导出json
                </el-button>
            </div>
        </div>

        <div class="small-toolbar" style="margin-bottom: 16px">
            <el-button
                size="mini"
                icon="el-icon-unlock"
                circle
                title="启用全部"
                @click="$store.commit('artifacts/unlockAll')"
            ></el-button>
        </div>

        <!-- artifacts display -->
        <el-tabs v-model="activeName" type="card">
            <el-tab-pane
                v-for="artType in artifactsType"
                :key="artType.name"
                class="panel"
                :name="artType.name"
            >
                <div slot="label" class="flex-row">
                    <img :src="artType.iconURL">
                    <span>{{ artType.chs }}</span>
                </div>
                <artifact
                    class="artifact-panel"
                    v-for="(item, index) in allArtifacts[artType.name]"
                    :key="artType.name + item.id"
                    :item="item"
                    @delete="removeArtifact(artType.name, index)"
                    @toggle="toggleArtifact(artType.name, index)"
                    @edit="editArtifact(artType.name, index)"
                ></artifact>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import AddArtifactDialog from "./AddArtifactDialog";
import ImportJsonDialog from "./ImportJsonDialog";
import OutputJsonDialog from "./OutputJsonDialog";
import Artifact from "./Artifact";
import EditArtifactDrawer from "./EditArtifactDrawer";

import { artifactsIcon } from "@asset/artifacts";
import { toChs as estimateToChs } from "@util/time_estimate";

export default {
    name: "ArtifactsPage",
    components: {
        AddArtifactDialog,
        ImportJsonDialog,
        OutputJsonDialog,
        Artifact,
        EditArtifactDrawer,
    },
    created: function () {
        this.artifactsIcon = artifactsIcon;

        this.artifactsType = [
            {
                name: "flower",
                chs: "生之花",
                iconURL: artifactsIcon["flower"],
            },
            {
                name: "feather",
                chs: "死之羽",
                iconURL: artifactsIcon["feather"],
            },
            {
                name: "sand",
                chs: "时之沙",
                iconURL: artifactsIcon["sand"],
            },
            {
                name: "cup",
                chs: "空之杯",
                iconURL: artifactsIcon["cup"],
            },
            {
                name: "head",
                chs: "理之冠",
                iconURL: artifactsIcon["head"],
            }
        ];
    },
    data: function() {
        return {
            activeName: "flower",

            newDialogVisible: false,
            importJsonDialogVisible: false,
            outputJsonDialogVisible: false,
            editArtifactDrawerVisible: false,

            editArtifactArgs: {
                position: "flower",
                index: -1,
            }
        }
    },
    methods: {
        /**
         * remove artifacts of type: position and index: index
         */
        removeArtifact: function(position, index) {
            this.$store.commit("artifacts/removeArtifact", {
                position, index
            });
        },

        /**
         * toggle enabled
         */
        toggleArtifact: function(position, index) {
            this.$store.commit("artifacts/toggleArtifact", {
                position, index
            });
        },

        /**
         * edit an artifact
         */
        editArtifact: function(position, index) {
            this.editArtifactDrawerVisible = true;
            this.editArtifactArgs.position = position;
            this.editArtifactArgs.index = index;

            let art = this.$store.getters["artifacts/allArtifacts"][position][index];

            this.$refs.editDrawer.setInit(art);
        },

        add: function() {
            this.newDialogVisible = true;
        },

        handleAddArtifact: function(item) {
            this.newDialogVisible = false;

            this.$store.commit("artifacts/addArtifact", item);
        },

        handleImportJsonClicked() {
            this.importJsonDialogVisible = true;
        },

        handleOutputJsonClicked() {
            this.outputJsonDialogVisible = true;
        },
    },
    computed: {
        allArtifacts() {
            return this.$store.getters["artifacts/allArtifacts"];
        },

        estimatedTime() {
            let iterCount = this.$store.getters["artifacts/iterCount"];
            return estimateToChs(iterCount);
        }
    }
}
</script>

<style scoped>
.panel {
    display: flex;
    flex-wrap: wrap;

}

.artifact-panel {
    margin: 8px;
}

.tool-bar {
    margin-bottom: 16px;
    margin-top: 16px;
}

.tool-bar .tool-right {
    float: right;
}
</style>