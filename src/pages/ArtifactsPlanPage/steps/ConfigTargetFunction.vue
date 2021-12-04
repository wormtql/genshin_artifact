<template>
    <div>
        <component
            v-if="!!tf.config"
            :is="tf.config"
            ref="config"
        ></component>
        <div v-else>
            <el-alert
                title="该函数无要配置的参数"
                :closable="false"
                style="margin-bottom: 20px"
            ></el-alert>
        </div>
    </div>
</template>

<script>
import targetFunctionsData from "@asset/target_functions/data";

// import deepCopy from "@util/deepcopy";

export default {
    name: "ConfigTargetFunction",
    data() {
        return {
            targetFuncName: "single",
        }
    },
    methods: {
        getConfig() {
            if (!this.tf.config) {
                return {};
            }

            let vm = this.$refs.config;
            if (vm.compact) {
                return vm.compact();
            } else {
                let temp = Object.assign({}, vm.$data);
                return temp;
            }
        },

        getTargetFuncConfig() {
            return this.getConfig();
        },

        setTargetFuncConfig(targetFunc) {
            if (!targetFunctionsData[targetFunc.name]) {
                return;
            }
            this.targetFuncName = targetFunc.name;

            this.$nextTick(() => {
                if (this.tf.config) {
                    this.$refs.config.setData(targetFunc.args ?? {});
                }
            });
        },

        setTargetFuncName(name) {
            if (name !== this.targetFuncName && targetFunctionsData[name]) {
                this.targetFuncName = name;
            }
        }
    },
    computed: {
        tf() {
            return targetFunctionsData[this.targetFuncName];
        },
    }
}
</script>