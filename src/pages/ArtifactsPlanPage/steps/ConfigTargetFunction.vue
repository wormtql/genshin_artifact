<template>
    <div>
        <component
            v-if="needConfig"
            :is="targetFunc.config"
            ref="config"
            v-model="value.args"
        ></component>
        <div v-else>
            <el-alert
                title="该函数无要配置的参数，请点击下一步"
                :closable="false"
                style="margin-bottom: 20px"
            ></el-alert>
        </div>

        <!-- <el-button @click="handleNextStep">下一步</el-button> -->
    </div>
</template>

<script>
import targetFunctionsData from "@asset/target_functions/data";

import deepCopy from "@util/deepcopy";

export default {
    name: "ConfigTargetFunction",
    props: {
        targetFuncName: {
            type: String,
        },

        value: {
            type: Object,
            required: true,
        }
    },
    watch: {
        targetFuncName(name) {
            if (this.$parent.lock) {
                // console.log("lock");
                return;
            }
            let data = targetFunctionsData[name];
            if (data && data.config) {
                let temp = deepCopy(this.value);
                temp.args = data.config.first();

                this.$emit("input", temp);
            } else {
                let temp = deepCopy(this.value);
                temp.args = {};

                this.$emit("input", temp);
            }
        }
    },
    computed: {
        targetFunc() {
            return targetFunctionsData[this.targetFuncName];
        },

        needConfig() {
            return !!this.targetFunc.config;
        }
    }
}
</script>