<template>
    <div>
        <h3 class="config-title">{{ params.title }}</h3>
        <template v-if="type === 'float'">
            <el-slider
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="0.1"
                :show-input="true"
                :show-input-controls="false"
            ></el-slider>
        </template>
        <template v-if="type === 'int'">
            <el-slider
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="1"
            ></el-slider>
        </template>
        <template v-if="type === 'intInput'">
            <el-input-number
                size="mini"
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
            ></el-input-number>
        </template>
        <template v-if="type === 'bool'">
            <el-switch
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                active-text="是"
            ></el-switch>
        </template>
        <template v-if="type === 'floatPercentageInput'">
            <el-input
                :modelValue="modelValue"
                @update:modelValue="handleInputValue"
            >
                <template slot="append">%</template>
            </el-input>
        </template>
        <template v-if="type === 'floatInput'">
            <el-input
                :modelValue="modelValue"
                @update:modelValue="handleInputValue"
            >
            </el-input>
        </template>
        <template v-if="type === 'element4'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro']"
            ></select-element-type>
        </template>
        <template v-if="type === 'element8'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro', 'Anemo', 'Geo', 'Dendro', 'Physical']"
            ></select-element-type>
        </template>
        <template v-if="type === 'skill4'">
            <select-skill-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            ></select-skill-type>
        </template>
        <template v-if="type === 'option'">
            <el-radio-group
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
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
import SelectSkillType from "@c/select/SelectSkillType"

export default {
    name: "ConfigItem",
    components: { SelectSkillType, SelectElementType },
    props: {
        modelValue: {},
        type: {},
        params: {},
    },
    emits: ["update:modelValue"],
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
            this.$emit("update:modelValue", value)
        },
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