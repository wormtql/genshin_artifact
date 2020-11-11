<template>
    <el-dialog
        :visible.sync="show"
        title="计算结果"
        width="80%"
        :before-close="close"
    >
        <h3>最佳搭配</h3>
        <div class="result">
            <artifact
                v-for="(item, index) in combo"
                :key="'result' + index"
                :item="item"
                style="margin-right: 16px; margin-bottom: 16px"
                :canDelete="false"
            >
                
            </artifact>
        </div>

        <h3>
            数值
        </h3>
        {{ resultValue }}

        <div slot="footer">
            <!-- <el-button>
                导出JSON
            </el-button> -->
            <el-button @click="close">
                关闭
            </el-button>
        </div>
    </el-dialog>
</template>

<script>
import Artifact from "@/components/Artifact";

export default {
    name: "ResultDialog",
    components: {
        Artifact
    },
    props: {
        combo: {
            type: Array
        },
        resultValue: {
            type: Number
        },
        show: {
            type: Boolean,
        },
    },
    methods: {
        close() {
            this.$emit("close");
        }
    }
}
</script>

<style scoped>
.result {
    display: flex;
    flex-wrap: wrap;
}
</style>