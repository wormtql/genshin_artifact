<template>
    <el-slider
        :value="value"
        @input="$emit('input', $event)"
        :min="0"
        :max="max"
        :step="0.01"
        :show-input="true"
        :show-input-controls="false"
        input-size="small"
    ></el-slider>
</template>

<script>
export default {
    name: "CommonConfigSlider",
    props: {
        title: {
            required: true,
        },
        max: {
            default: 1
        },
        value: {},
    }
}
</script>