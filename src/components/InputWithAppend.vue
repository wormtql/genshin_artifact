<template>
    <el-input
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <template slot="append">
            %
        </template>
    </el-input>
</template>

<script>
export default {
    name: "InputWithAppend",
    props: {
        value: {
            type: String,
            required: true,
        }
    }
}
</script>