<template>
    <el-dialog
        title="导入json"
        :visible.sync="visible"
        width="80%"
        :before-close="handleClose"
    >
        <h3 class="text">欢迎大佬开发第三方导出工具，json接口请参考：<a target="_blank" href="https://wormtql.gitbook.io/mona-uranai/">文档</a></h3>
        <p></p>
        <el-input
            type="textarea"
            placeholder="输入json"
            v-model="json"
            :rows="5"
        >
        </el-input>

        <div class="buttons">
            <el-button
                type="primary"
                class="confirm-button"
                @click="handleOverwrite"
                style="margin-left: 20px"
            >
                覆盖导入<i class="el-icon-document-delete"></i>
            </el-button>
            <el-button
                type="primary"
                class="confirm-button"
                @click="handleAppend"
                style="margin-left: 20px"
            >
                追加导入<i class="el-icon-document-add"></i>
            </el-button>
            <el-button class="cancel-button" @click="handleClose">取消</el-button>
        </div>
    </el-dialog>
</template>

<script>
import checkImportJson from "@util/checkImportJson";

export default {
    name: "ImportJsonDialog",
    props: {
        visible: {
            type: Boolean,
        }
    },
    data: function () {
        return {
            json: "",
        }
    },
    methods: {
        handleClose() {
            this.$emit("close");
        },

        appendArtifacts() {
            let { artifacts, invalidCount } = checkImportJson(this.json);
            if (invalidCount > 0) {
                this.$message({
                    type: "warning",
                    message: `有${invalidCount}个圣遗物无法识别，这些圣遗物已舍弃`
                });
            }

            this.$store.commit("artifacts/appendArtifacts", artifacts);
        },

        handleAppend() {
            if (this.json) {
                this.appendArtifacts();
            }

            this.$emit("close");
        },

        handleOverwrite() {
            if (this.json) {
                this.$store.commit("artifacts/removeAllArtifacts");

                this.appendArtifacts();
            }

            this.$emit("close");
        }
    }
}
</script>

<style scoped>
.buttons {
    margin-top: 32px;
    display: flex;
    flex-direction: row-reverse;
}

.text {
    margin: 0;
}
</style>