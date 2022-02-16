<template>
    <div>
        <h3 class="config-title">{{ params.title }}</h3>
        <template v-if="type === 'float'">
            <el-slider
                :value="value"
                @input="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="0.1"
                :show-input="true"
                :show-input-controls="false"
                input-size="mini"
            ></el-slider>
        </template>
        <template v-if="type === 'int'">
            <el-slider
                :value="value"
                @input="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="1"
            ></el-slider>
        </template>
        <template v-if="type === 'intInput'">
            <el-input-number
                size="mini"
                :value="value"
                @input="handleChangeValue"
                :min="params.min"
                :max="params.max"
            ></el-input-number>
        </template>
        <template v-if="type === 'bool'">
            <el-switch
                :value="value"
                @input="handleChangeValue"
                active-text="是"
            ></el-switch>
        </template>
        <template v-if="type === 'floatPercentageInput'">
            <el-input
                :value="value"
                @input="handleInputValue"
                size="mini"
            >
                <template slot="append">%</template>
            </el-input>
        </template>
        <template v-if="type === 'floatInput'">
            <el-input
                :value="value"
                @input="handleInputValue"
                size="mini"
            >
            </el-input>
        </template>
        <template v-if="type === 'element4'">
            <select-element-type
                :value="value"
                @input="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro']"
            ></select-element-type>
        </template>
        <template v-if="type === 'option'">
            <el-radio-group
                :value="value"
                @input="handleChangeValue"
                size="small"
            >
                <el-radio-button
                    v-for="(option, index) in params.options"
                    :key="index"
                    :label="index"
                >{{ option }}</el-radio-button>
            </el-radio-group>
        </template>
    </div>
</template>

<script>
import SelectElementType from "@c/select/SelectElementType"

export default {
    name: "ConfigItem",
    components: { SelectElementType },
    props: {
        value: {},
        type: {},
        params: {},
    },
    methods: {
        handleInputValue(value) {
            let v = 0
            if (value === "") {
                v = 0
            } else {
                v = parseFloat(value)
                if (isNaN(v)) {
                    v = 0
                }
            }
            
            this.handleChangeValue(v)
        },

        handleChangeValue(value) {
            this.$emit("input", value)
        }
    }
}
</script>

<style lang="scss" scoped>
.config-title {
    font-size: 12px;
    color: #666666;
    margin: 0 0 12px 0;
}

// .config-item {
//     margin-bottom: 8px;

//     &:last-of-type {
//         margin-bottom: 0;
//     }
// }
</style>