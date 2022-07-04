<template>
    <el-radio-group
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-radio-button label="NormalAttack">{{ t("skillType.a") }}</el-radio-button>
        <el-radio-button label="ChargedAttack">{{ t("skillType.b") }}</el-radio-button>
        <el-radio-button label="ElementalSkill">{{ t("skillType.e") }}</el-radio-button>
        <el-radio-button label="ElementalBurst">{{ t("skillType.q") }}</el-radio-button>
    </el-radio-group>
</template>

<script>
import {useI18n} from "../../i18n/i18n"

export default {
    name: "SelectSkillType",
    props: {
        modelValue: {
            type: String,
            required: true,
        }
    },
    emits: ["update:modelValue"],
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>