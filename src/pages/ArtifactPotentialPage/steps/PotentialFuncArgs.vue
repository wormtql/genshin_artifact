<template>
    <div>
        <keep-alive v-if="needConfig">
            <component
                :is="potentialFunc.config"
                ref="config"
            ></component>
        </keep-alive>
        <div v-else>
            <el-alert
                title="该函数无要配置的参数，请点击下一步"
                :closable="false"
                style="margin-bottom: 20px"
            ></el-alert>
        </div>

        <!-- <el-button @click="computeAll" type="primary" style="width: 100%">计算所有圣遗物</el-button> -->
    </div>
</template>

<script>
import potentialFuncData from "@asset/potential_functions/data";
import deepCopy from "@util/deepcopy";

export default {
    name: "PotentialFuncArgs",
    props: {
        funcName: {
            type: String,
        }
    },
    computed: {
        potentialFunc() {
            return potentialFuncData[this.funcName];
        },

        needConfig() {
            return this.potentialFunc && this.potentialFunc.config;
        }
    },
    methods: {
        getPArgs() {
            if (!this.needConfig) {
                return {};
            }

            let configData = {};
            if (this.$refs.config.compact) {
                configData = this.$refs.config.compact();
            } else {
                configData = deepCopy(this.$refs.config.$data);
            }

            return configData;
        },
    }
}
</script>