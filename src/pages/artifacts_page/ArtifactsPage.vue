<template>
    <div>
        <!-- new artifact dialog -->
        <add-artifact-dialog
            :visible="newDialogVisible"
            @close="newDialogVisible = false"
            @confirm="handleAddArtifact"
        ></add-artifact-dialog>

        <!-- bread crumb -->
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-alert
            type="error"
            v-show="!$store.getters.valid"
            title="圣遗物数量过多，请禁用或者删除明显更次的圣遗物"
            :closable="false"
        >
        </el-alert>

        <!-- tool bar -->
        <div class="tool-bar">
            <el-button @click="add"
                type="primary"
                icon="el-icon-plus"
            >
                添加圣遗物
            </el-button>
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
                    v-for="(item, index) in $store.state[artType.name]"
                    :key="item.id"
                    :item="item"
                    @delete="removeArtifact(artType.name, index)"
                    @toggle="toggleArtifact(artType.name, index)"
                ></artifact>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
import AddArtifactDialog from "./AddArtifactDialog";
import Artifact from "./Artifact";

import { artifactsIcon } from "../../assets/artifacts";

export default {
    name: "ArtifactsPage",
    components: {
        AddArtifactDialog,
        Artifact,
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
        }
    },
    methods: {
        /**
         * remove artifacts of type: position and index: index
         */
        removeArtifact: function(position, index) {
            this.$store.commit("removeArtifact", {
                position, index
            });
        },

        /**
         * toggle enabled
         */
        toggleArtifact: function(position, index) {
            this.$store.commit("toggleArtifact", {
                position, index
            });
        },

        add: function() {
            this.newDialogVisible = true;
        },

        handleAddArtifact: function(item) {
            this.newDialogVisible = false;

            this.$store.commit("addArtifact", item);
        },
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
</style>