<template>
    <div>
        <component v-if="targetFunc.config" :is="targetFunc.config" ref="config">
        </component>
        <div v-else>
            <el-alert
                title="该函数无要配置的参数，请点击下一步"
                :closable="false"
                style="margin-bottom: 20px"
            ></el-alert>
        </div>

        <el-button @click="handleNextStep">下一步</el-button>
    </div>
</template>

<script>
import { targetFunctionsData } from "@asset/target_functions";

export default {
    name: "ConfigTargetFunction",
    props: {
        targetFuncName: {
            type: String,
        }
    },
    methods: {
        handleNextStep() {
            if (!this.targetFunc.config) {
                this.$emit("select", {});
                return;
            }

            let configData = {};
            if (this.$refs.config.compact) {
                configData = this.$refs.config.compact();
            } else {
                configData = Object.assign({}, this.$refs.config.$data);
            }
            this.$emit("select", configData);
        }
    },
    computed: {
        targetFunc() {
            return targetFunctionsData[this.targetFuncName];
        }
    }
}
</script>