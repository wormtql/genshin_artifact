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