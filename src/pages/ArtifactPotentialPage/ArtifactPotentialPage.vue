<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>圣遗物潜力系统</el-breadcrumb-item>
            <!-- <el-button>123</el-button> -->
        </el-breadcrumb>
        <el-divider></el-divider>

        <my-step
            :steps="['潜力函数', '参数', '结果']"
            :pointer="step"
            @navigate="step = $event"
        ></my-step>

        <div style="margin-top: 24px">
            <choose-potential-func
                v-show="step === 0"
                @select="handleSelectPotentialFunc"
            ></choose-potential-func>

            <potential-func-args
                v-show="step === 1"
                :func-name="selected.funcName"
                @single="handleComputeSingle"
                @all="handleComputeAll"
            ></potential-func-args>

            <!-- <potential-compute-panel
                v-show="step === 2"
            ></potential-compute-panel> -->

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
            selected: {
                funcName: "",
                pArgs: {},
            }
        }
    },
    methods: {
        handleSelectPotentialFunc(name) {
            this.selected.funcName = name;

            this.step++;
        },

        handleComputeSingle(config) {
            this.selected.pArgs = config;

            this.step++;
        },

        handleComputeAll(config) {
            this.selected.pArgs = config;

            this.step++;
            this.$refs.resultOfAll.compute();
        }
    }
}
</script>