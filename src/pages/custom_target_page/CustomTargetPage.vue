<template>
    <div>
        <custom-target-dialog
            :show="showCustomTargetDialog"
            @close="showCustomTargetDialog = false"
            @confirm="onCustomTargetDialogConfirm"
            :usedNames="usedTargetNames"
        >
        </custom-target-dialog>

        <el-dialog :visible.sync="exportJsonDialogVisible" title="导出JSON">
            <p style="max-height: 300px; overflow: auto">{{ json }}</p>
            <span slot="footer">
                <el-button class="clip" @click="clipJson">复制</el-button>
            </span>
        </el-dialog>

        <el-breadcrumb>
            <el-breadcrumb-item>自定义目标函数</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-alert :closable="false" style="margin-bottom: 16px" show-icon>
            <template #title>
                由于目标函数预设并不包含所有角色，虽然有近似的通用目标函数，但是如果要追求准确，可以在此自定义<br>
                各项参数的具体意义请参考“算法”页面
            </template>
        </el-alert>

        <div class="bar">
            <el-button @click="showCustomTargetDialog = true" type="primary" icon="el-icon-plus"
            >添加</el-button>

            <div style="vertical-align: top">
                <el-button @click="exportJson">导出Json</el-button>
                <el-button @click="importJson">导入Json</el-button>
            </div>
        </div>
        

        <el-tabs
            type="border-card"
            tab-position="left"
            
            v-if="usedTargetNames.length !== 0"
        >
            
            <el-tab-pane
                v-for="(value, name) in customedTargets"
                :key="name"
                :label="name"
                :name="name"
            >
                
                <preview-target :config="value.config"></preview-target>

                <el-button style="margin-top: 16px" @click="deleteCustomTarget(name)">删除</el-button>
            </el-tab-pane>
        </el-tabs>
        <p v-else>还没有添加自定义目标函数</p>
    </div>
</template>

<script>
import CustomTargetDialog from "./CustomTargetDialog";
import PreviewTarget from "./PreviewTarget";

import { mapState } from "vuex";
import { all } from "@/common/target/base";

import Clipboard from "clipboard";

export default {
    name: "CustomTargetPage",
    components: {
        CustomTargetDialog,
        PreviewTarget,
    },
    data: function() {
        return {
            showCustomTargetDialog: false,

            exportJsonDialogVisible: false,
            json: "",
        }
    },
    methods: {
        onCustomTargetDialogConfirm(item) {
            this.showCustomTargetDialog = false;
            let func = all(item.config);
            this.$store.commit("addCustomedTarget", {
                name: item.name,
                calc: func,
                config: item.config,
            });
            // window.console.log(item);
        },
        deleteCustomTarget(name) {
            this.$store.commit("deleteCustomedTarget", name);
        },
        exportJson() {
            this.json = JSON.stringify(this.customedTargets);
            this.exportJsonDialogVisible = true;
        },
        importJson() {
            this.$prompt("输入JSON", "导入JSON", {
                confirmButtonText: "确定",
                cancelButtonText: "取消"
            }).then(({ value }) => {
                if (value === "") {
                    return;
                }
                return new Promise((resolve, reject) => {
                    let obj = JSON.parse(value);
                    if (typeof obj !== "object") {
                        reject("wrong_format");
                    }

                    for (let key in obj) {
                        this.$store.commit("addCustomedTarget", {
                            name: key,
                            calc: all(obj[key].config),
                            config: obj[key].config,
                        });
                    }
                })
            }).catch((e) => {
                if (e !== "cancel") {
                    this.$message.error("请输入正确的json字符串");
                }
            })
        },
        clipJson() {
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
        }
    },
    computed: {
        usedTargetNames() {
            return Object.keys(this.customedTargets);
        },

        ...mapState([
            "customedTargets"
        ])
    }
}
</script>

<style scoped>
.bar {
    display: flex;
    align-items: center;
    justify-content: space-between;

    margin-bottom: 16px;
}
</style>