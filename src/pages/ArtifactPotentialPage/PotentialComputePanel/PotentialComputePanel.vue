<template>
    <div>
        <el-row :gutter="16">
            <el-col :span="16">
                <el-alert
                    title="在此处选择圣遗物进行计算"
                    :closable="false"
                    style="margin-bottom: 16px"
                ></el-alert>
                <select-artifact
                    @select="handleCompute"
                ></select-artifact>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import SelectArtifact from "./SelectArtifact";
import { computeSingle } from "@alg/potential/compute_artifact_potential_promise";

export default {
    name: "PotentialComputePanel",
    components: {
        SelectArtifact,
    },
    methods: {
        handleCompute(art) {
            let name = this.$parent.$data.selected.funcName;
            let pArgs = this.$parent.$data.selected.pArgs;
            console.log("compute start", name, pArgs);
            computeSingle(art, name, pArgs).then(result => {
                console.log(result);
            })
        }
    }
}
</script>