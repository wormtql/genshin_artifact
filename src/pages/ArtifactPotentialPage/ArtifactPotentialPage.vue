<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物潜力计算</el-breadcrumb-item>
            <!-- <el-button>123</el-button> -->
        </el-breadcrumb>
        <el-divider></el-divider>

        <div class="toolbar">
            <div class="toolbar-right">
                <el-button
                    size="small"
                    type="primary"
                    @click="handleComputeAll"
                >
                    <i class="el-icon-cpu"></i>
                    开始计算
                </el-button>
            </div>
        </div>

        <my-step
            :steps="['潜力函数', '参数', '结果']"
            :pointer="step"
            :lock="false"
            @navigate="step = $event"
        ></my-step>

        <div style="margin-top: 24px">
            <choose-potential-func
                v-model="funcName"
                v-show="step === 0"
            ></choose-potential-func>

            <potential-func-args
                v-show="step === 1"
                :func-name="funcName"
                ref="configPotentialFunc"
            ></potential-func-args>

            <result-of-all
                ref="resultOfAll"
                v-show="step === 2"
            ></result-of-all>
        </div>
    </div>
</template>

<script>
import ChoosePotentialFunc from "./steps/ChoosePotentialFunc";
import PotentialFuncArgs from "./steps/PotentialFuncArgs";
import MyStep from "@c/MyStep";
// import PotentialComputePanel from "./PotentialComputePanel";
import ResultOfAll from "./ResultOfAll";

export default {
    name: "ArtifactPotentialPage",
    components: {
        MyStep,
        ChoosePotentialFunc,
        PotentialFuncArgs,
        // PotentialComputePanel,
        ResultOfAll,
    },
    data: function () {
        return {
            step: 0,
            
            funcName: "ratio",
        }
    },
    methods: {
        handleComputeAll() {
            let args = this.$refs.configPotentialFunc.getPArgs();

            let configObject = {
                potentialFunction: {
                    name: this.funcName,
                    args,
                }
            };

            this.step = 2;
            this.$refs.resultOfAll.compute(configObject);
        }
    }
}
</script>

<style lang="scss" scoped>
.toolbar {
    margin-bottom: 24px;
}
</style>