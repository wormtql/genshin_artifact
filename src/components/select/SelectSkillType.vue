<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
    >
        <el-radio-button label="a">普通攻击</el-radio-button>
        <el-radio-button label="b">重击</el-radio-button>
        <el-radio-button label="e">元素战技</el-radio-button>
        <el-radio-button label="q">元素爆发</el-radio-button>
    </el-radio-group>
</template>

<script>
export default {
    name: "SelectSkillType",
    props: {
        value: {
            type: String,
            required: true,
        }
    }
}
</script>