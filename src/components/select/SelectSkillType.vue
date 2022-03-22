<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-radio-button label="NormalAttack">普通攻击</el-radio-button>
        <el-radio-button label="ChargedAttack">重击</el-radio-button>
        <el-radio-button label="ElementalSkill">元素战技</el-radio-button>
        <el-radio-button label="ElementalBurst">元素爆发</el-radio-button>
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