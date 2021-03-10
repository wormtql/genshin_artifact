<template>
    <div>
        <component
            v-if="funcName && potentialFunc.needConfig"
            :is="config"
            ref="config"
        ></component>
        <div v-else>
            <el-alert
                title="该函数无要配置的参数，请点击下一步"
                :closable="false"
                style="margin-bottom: 20px"
            ></el-alert>
        </div>

        <!-- <el-row :gutter="16">
            <el-col :span="12">
                <el-button @click="computeSingle">计算单个圣遗物</el-button>
            </el-col>

            <el-col :span="12">
                <el-button @click="computeAll">计算所有圣遗物</el-button>
            </el-col>
        </el-row> -->
        <el-button @click="computeAll" type="primary" style="width: 100%">计算所有圣遗物</el-button>
    </div>
</template>

<script>
import potentialFuncData from "@asset/potential_functions/data";
import pfConfigs from "@asset/potential_functions/config";

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

        config() {
            return pfConfigs[this.funcName].config;
        }
    },
    methods: {
        getPArgs() {
            if (!this.potentialFunc.needConfig) {
                return {};
            }

            let configData = {};
            if (this.$refs.config.compact) {
                configData = this.$refs.config.compact();
            } else {
                configData = Object.assign({}, this.$refs.config.$data);
            }

            return configData;
        },

        computeSingle() {
            let configData = this.getPArgs();

            this.$emit("single", configData);
        },

        computeAll() {
            let configData = this.getPArgs();

            this.$emit("all", configData);
        }
    }
}
</script>