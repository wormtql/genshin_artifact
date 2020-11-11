<template>
    <div>
        <new-dialog :visible="newDialogVisible"
            @close="newDialogVisible = false"
            @confirm="doAdd"
        ></new-dialog>
        <el-dialog :visible.sync="exportJsonDialogVisible" title="导出JSON">
            <p style="max-height: 300px; overflow: auto">{{ json }}</p>
            <span slot="footer">
                <el-button class="clip" @click="clipJson">复制</el-button>
            </span>
        </el-dialog>

        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div style="display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; margin-bottom: 16px">
            <el-button @click="add"
                type="primary"
                icon="el-icon-plus"
            >
                <!-- <i class="el-icon-plus" style="padding: 0; margin: 0;"></i> -->
                添加圣遗物
            </el-button>
            <div>
                <el-button @click="exportJson">导出json</el-button>
                <el-button @click="importJson">导入json</el-button>
            </div>
        </div>

        <el-tabs v-model="activeName" type="border-card">
            <el-tab-pane label="生之花" name="flower" class="panel">
                <artifact
                    class="artifactPanel"
                    v-for="(item, index) in $store.state.flower"
                    :key="index"
                    :item="item"
                    :omit="item.omit"
                    @delete="removeArtifact"
                    @toggle="toggleArtifact"
                ></artifact>
                <!-- <add-button @click="add" class="artifactPanel"></add-button> -->
            </el-tab-pane>
            <el-tab-pane label="死之羽" name="feather" class="panel">
                <artifact
                    class="artifactPanel"
                    v-for="(item, index) in $store.state.feather"
                    :key="index"
                    :item="item"
                    :omit="item.omit"
                    @delete="removeArtifact"
                    @toggle="toggleArtifact"
                ></artifact>
                <!-- <add-button @click="add" class="artifactPanel"></add-button> -->
            </el-tab-pane>
            <el-tab-pane label="时之沙" name="sand" class="panel">
                <artifact
                    class="artifactPanel"
                    v-for="(item, index) in $store.state.sand"
                    :key="index"
                    :item="item"
                    :omit="item.omit"
                    @delete="removeArtifact"
                    @toggle="toggleArtifact"
                ></artifact>
                <!-- <add-button @click="add" class="artifactPanel"></add-button> -->
            </el-tab-pane>
            <el-tab-pane label="空之杯" name="cup" class="panel">
                <artifact
                    class="artifactPanel"
                    v-for="(item, index) in $store.state.cup"
                    :key="index"
                    :item="item"
                    :omit="item.omit"
                    @delete="removeArtifact"
                    @toggle="toggleArtifact"
                ></artifact>
                <!-- <add-button @click="add" class="artifactPanel"></add-button> -->
            </el-tab-pane>
            <el-tab-pane label="理之冠" name="head" class="panel">
                <artifact
                    class="artifactPanel"
                    v-for="(item, index) in $store.state.head"
                    :key="index"
                    :item="item"
                    :omit="item.omit"
                    @delete="removeArtifact"
                    @toggle="toggleArtifact"
                ></artifact>
                <!-- <add-button @click="add" class="artifactPanel"></add-button> -->
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
// import AddButton from "./AddButton";
import NewDialog from "./NewDialog";
import Artifact from "@/components/Artifact";

import Clipboard from "clipboard";

export default {
    name: "ArtifactsPage",
    components: {
        // AddButton,
        NewDialog,
        Artifact,
    },
    data: function() {
        return {
            activeName: "flower",

            newDialogVisible: false,
            exportJsonDialogVisible: false,

            json: "",
        }
    },
    methods: {
        removeArtifact: function(item) {
            if (item.position === "flower") {
                this.$store.commit("deleteFlower", item);
            } else if (item.position === "feather") {
                this.$store.commit("deleteFeather", item);
            } else if (item.position === "sand") {
                this.$store.commit("deleteSand", item);
            } else if (item.position === "cup") {
                this.$store.commit("deleteCup", item);
            } else if (item.position === "head") {
                this.$store.commit("deleteHead");
            }
        },
        add: function() {
            this.newDialogVisible = true;
        },
        doAdd: function(value) {
            // window.console.log(value);
            this.newDialogVisible = false;

            if (value.position === "flower") {
                this.$store.commit("addFlower", value);
            } else if (value.position === "feather") {
                this.$store.commit("addFeather", value);
            } else if (value.position === "sand") {
                this.$store.commit("addSand", value);
            } else if (value.position === "cup") {
                this.$store.commit("addCup", value);
            } else if (value.position === "head") {
                this.$store.commit("addHead", value);
            }
        },
        exportJson: function() {
            let obj = {
                "flower": this.$store.state.flower,
                "feather": this.$store.state.feather,
                "sand": this.$store.state.sand,
                "cup": this.$store.state.cup,
                "head": this.$store.state.head,
            };

            let s = JSON.stringify(obj);
            this.json = s;

            this.exportJsonDialogVisible = true;
            
        },
        importJson: function() {
            this.$prompt("输入JSON", "导入JSON", {
                confirmButtonText: "确定",
                cancelButtonText: "取消"
            }).then(({ value }) => {
                let obj = JSON.parse(value);
                if (obj.flower) {
                    this.$store.commit("setFlower", obj.flower);
                }
                if (obj.feather) {
                    this.$store.commit("setFeather", obj.feather);
                }
                if (obj.sand) {
                    this.$store.commit("setSand", obj.sand);
                }
                if (obj.cup) {
                    this.$store.commit("setCup", obj.cup);
                }
                if (obj.head) {
                    this.$store.commit("setHead", obj.head);
                }
            })
        },
        clipJson: function() {
            // console.log("aaa");
            let s = this.json;
            let clipboard = new Clipboard(".clip", {
                text: function() {
                    return s;
                }
            });
            clipboard.on("success", () => {
                this.$message({ message: "复制成功", type: "success" });
                clipboard.destroy();
            });
            clipboard.on("error", () => {
                this.$message({ message: "复制失败", type: "error" });
                clipboard.destroy();
            });
            // console.log("bbb");
        },
        toggleArtifact: function(art) {
            // window.console.log(art);
            if (typeof art.omit !== "undefined") {
                art.omit = !art.omit;
            } else {
                art.omit = false;
            }
        }
    }
}
</script>

<style scoped>
.panel {
    display: flex;
    flex-wrap: wrap;

}

.artifactPanel {
    margin: 8px;
}
</style>