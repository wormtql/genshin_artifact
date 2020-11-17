<template>
    <div>
        <custom-target-dialog
            :show="showCustomTargetDialog"
            @close="showCustomTargetDialog = false"
            @confirm="onCustomTargetDialogConfirm"
            :usedNames="usedTargetNames"
        >
        </custom-target-dialog>

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

        <el-button @click="showCustomTargetDialog = true" type="primary" icon="el-icon-plus"
            style="margin-bottom: 16px"
        >添加</el-button>

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

export default {
    name: "CustomTargetPage",
    components: {
        CustomTargetDialog,
        PreviewTarget,
    },
    data: function() {
        return {
            showCustomTargetDialog: false,
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