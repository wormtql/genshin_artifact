<template>
    <div>
        <div class="main-div" v-if="!resultData.error">
            <div class="left">
                <el-alert
                    title="由于原神战斗体系比较复杂，许多buff为动态运行时效果（例如击中敌人时效果、仅对雷元素附着的效果、叠层数的效果等），而此处运算结果仅为静态buff的最优值，因此计算结果仅供参考"
                    :closable="false"
                >
                </el-alert>

                <h3 class="title">最佳搭配</h3>
                <div class="artifact-div" v-if="!calculating">
                    <artifact-display
                        v-for="(art, index) in filteredArtifacts"
                        :key="index"
                        :item="art"
                        class="artifact"
                    ></artifact-display>
                </div>
                <div>
                    <el-button size="small" @click="disableArtifacts">禁用以上圣遗物</el-button>
                </div>

                <h3 class="title">最大值</h3>
                <el-alert
                    title="不同目标函数的最大值不可相互比较；输出类型的最大值也并不是最终期望伤害，因此仅供参考"
                    type="warning"
                    :closable="false"
                    style="margin-bottom: 12px"
                ></el-alert>
                <p class="max-value" v-if="!calculating">{{ resultData.value.toFixed(3) }}</p>
            </div>
            <div class="right">
                <attribute-panel :panel="resultData.attribute" style="width: 300px" v-if="!calculating"></attribute-panel>
            </div>
        </div>
        <div v-else>
            没有合适的圣遗物
        </div>
    </div>
</template>

<script>
import ArtifactDisplay from "@c/ArtifactDisplay";
import AttributePanel from "@c/AttributePanel";

export default {
    name: "ResultPage",
    components: {
        ArtifactDisplay,
        AttributePanel,
    },
    props: {
        calculating: {
            type: Boolean,
            default: true,
        },
        resultData: {
            type: Object
        },
        // config: {
        //     type: Object,
        // },
    },
    methods: {
        disableArtifacts() {
            for (let art of this.filteredArtifacts) {
                this.$store.commit("disableArtifactById", { id: art.id });
            }
            this.$message("操作成功");
        }
    },
    computed: {
        filteredArtifacts() {
            return this.resultData.artifacts.filter(item => item);
        }
    }
}
</script>

<style scoped>
.title {
    /* background:rgb(74, 99, 211); */
    padding: 0px 16px;
    display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}

.artifact-div {
    display: flex;
    flex-wrap: wrap;
}

.artifact {
    margin: 0 16px 16px 0;
}

.max-value {
    padding: 0;
    margin: 0;
    font-size: 14px;
}

.left {
    flex: 1;
    margin-right: 16px;
}

.main-div {
    display: flex;
}
</style>