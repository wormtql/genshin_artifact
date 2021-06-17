<template>
    <div>
        <el-dialog
            :visible.sync="selectArtifactDialogVisible"
            width="80%"
            title="选择圣遗物"
        >
            <select-artifacts
                :pos="selectArtifactSlot"
                @input="handleConfirmChangeSlot"
                :value="selectArtifactId"
            ></select-artifacts>
        </el-dialog>

        <click-edit-label
            :value="node.label"
            @input="handleChangeTitle"
            fontsize="24px"
            :editable="canDelete"
        ></click-edit-label>

        <div class="toolbar">
            <div class="left">
                <el-button
                    size="small"
                    @click="handleNewKumi"
                >新建套装<i class="el-icon-plus"></i></el-button>
            </div>
            
            <div class="right">
                <el-popconfirm
                    title="确定删除？"
                    @confirm="handleDeleteDir"
                    v-if="canDelete"
                >
                    <el-button
                        size="small"
                        type="danger"
                        slot="reference"
                    >删除收藏夹</el-button>
                </el-popconfirm>
                
            </div>
        </div>

        <kumi-item
            v-for="kumi in kumiByDir[dirId]"
            :key="kumi.id"
            @click="handleChangeSlot(kumi.id, $event)"
            @delete="handleDeleteKumi(kumi.id)"
            @deleteArtifact="handleDeleteKumiArtifact(kumi.id, $event)"
            :data="kumi"
            class="kumi-item"
        ></kumi-item>
    </div>
</template>

<script>
import getTreeNode from "@/store/utils/getTreeNode";
import positions from "@const/positions";

import ClickEditLabel from "@c/misc/ClickEditLabel";
import KumiItem from "./KumiItem.vue";
import SelectArtifacts from "@c/select/SelectArtifacts";

export default {
    name: "KumiDirectory",
    components: {
        ClickEditLabel,
        KumiItem,
        SelectArtifacts,
    },
    props: ["dirId"],
    data() {
        return {
            selectArtifactDialogVisible: false,
            selectArtifactSlot: "flower",
            selectArtifactId: -1,
            changeSlotKumiId: -1,
            changeSlotPosId: 0,
        }
    },
    methods: {
        handleChangeTitle(text) {
            if (text !== this.node.title && text !== "") {
                this.$store.commit("kumi/updateDirName", { id: this.dirId, newName: text });
            }
        },

        handleChangeSlot(kumiId, posId) {
            let pos = positions[posId];
            this.selectArtifactSlot = pos;
            this.selectArtifactDialogVisible = true;
            this.changeSlotPosId = posId;
            this.changeSlotKumiId = kumiId;
        },

        handleConfirmChangeSlot(artId) {
            this.$store.commit("kumi/updateKumiArtifact", {
                id: this.changeSlotKumiId,
                posId: this.changeSlotPosId,
                newId: artId
            });
            // this.selectArtifactId = artId;
            this.selectArtifactDialogVisible = false;
        },

        handleNewKumi() {
            this.$store.commit("kumi/newKumi", {
                ids: [-1, -1, -1, -1, -1],
                label: "新建套装",
                under: this.dirId,
            });
        },

        handleDeleteKumi(id) {
            this.$store.commit("kumi/deleteKumi", { id });
        },

        handleDeleteDir() {
            this.$emit("delete", this.dirId);
        },

        handleDeleteKumiArtifact(kumiId, posId) {
            this.$store.commit("kumi/updateKumiArtifact", {
                id: kumiId,
                posId,
                newId: -1
            });
        },
    },
    computed: {
        node() {
            return getTreeNode(this.$store.state.kumi["tree"], node => node.id === this.dirId && node.type === "dir");
        },

        kumiByDir() {
            return this.$store.getters["kumi/kumiByDir"];
        },

        canDelete() {
            return this.node.id > 1;
        }
    }
}
</script>

<style lang="scss" scoped>
.toolbar {
    margin: 16px 0;
    display: flex;
    justify-content: space-between;
}

.kumi-item {
    margin-bottom: 16px;
    // box-shadow: 0px 0px 10px 1px #00000011;
}

.child-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #dcdfe6;
    padding: 8px;

    .child-label {
        font-size: 14px;

        input {
            border: none;
            outline: none;
            padding: 8px;
            border-radius: 3px;

        }
    }
}
</style>