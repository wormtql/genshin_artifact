<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
    >
        <el-radio-button label="fire">火</el-radio-button>
        <el-radio-button label="thunder">雷</el-radio-button>
        <el-radio-button label="water">水</el-radio-button>
        <el-radio-button label="wind">风</el-radio-button>
        <el-radio-button label="rock">岩</el-radio-button>
        <el-radio-button label="ice">冰</el-radio-button>
        <el-radio-button label="physical">物理</el-radio-button>
    </el-radio-group>
</template>

<script>
export default {
    name: "SelectElementType",
    props: {
        value: {
            type: String,
            required: true,
        }
    },
}
</script>