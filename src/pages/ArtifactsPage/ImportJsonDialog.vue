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
            <el-button type="primary" class="confirm-button" @click="handleConfirm">确定</el-button>
            <el-button class="cancel-button" @click="handleClose">取消</el-button>
        </div>
    </el-dialog>
</template>

<script>
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

        handleConfirm() {
            if (this.json) {
                this.$emit("confirm", this.json)
            }
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

.cancel-button {
    margin-right: 10px;
}

.text {
    margin: 0;
}
</style>