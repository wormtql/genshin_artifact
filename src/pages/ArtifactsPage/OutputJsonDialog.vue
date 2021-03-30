<template>
    <el-dialog
        title="导出json"
        :visible.sync="visible"
        width="80%"
        :before-close="handleClose"
    >
        <p class="text">
            {{ outputJson }}
        </p>

        <div class="buttons">
            <el-button
                type="primary"
                class="confirm-button"
                @click="handleDownload"
                >下载</el-button
            >
            <el-button
                type="primary"
                class="confirm-button"
                @click="handleCopy"
                v-if="canCopy"
                >复制</el-button
            >
            <el-button class="cancel-button" @click="handleClose"
                >关闭</el-button
            >
        </div>
    </el-dialog>
</template>

<script>
export default {
    name: 'OutputJsonDialog',
    props: {
        visible: {
            type: Boolean,
        },
    },
    created: function() {
        this.canCopy = !!navigator.clipboard;
    },
    data: function() {
        return {
            json: '',
        };
    },
    methods: {
        handleClose() {
            this.$emit('close');
        },

        handleCopy() {
            navigator.clipboard
                .writeText(this.outputJson)
                .then(() => {
                    this.$message('复制成功');
                })
                .catch(() => {
                    this.$message.error('复制失败');
                });
        },

        handleDownload() {
            const $a = document.createElement('a');
            $a.href = `data:application/json;charset=utf-8,${this.outputJson}`;
            $a.download =
                'genshin_artifact_' + new Date().toLocaleString() + '.json';
            document.body.appendChild($a);
            $a.click();
            $a.remove();
        },
    },
    computed: {
        outputJson() {
            let obj = {};
            ['flower', 'feather', 'sand', 'cup', 'head'].forEach(item => {
                obj[item] = this.$store.state.artifacts[item];
            });

            return JSON.stringify(obj);
        },
    },
};
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
    max-height: 116px;
    overflow: auto;
}
</style>