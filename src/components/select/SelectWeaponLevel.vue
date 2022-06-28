<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-option-group label=">=60">
            <el-option label="60-" value="60-"></el-option>
            <el-option label="60+" value="60+"></el-option>
            <el-option label="65" value="65"></el-option>
        </el-option-group>
        <el-option-group label=">=70">
            <el-option label="70-" value="70-"></el-option>
            <el-option label="70+" value="70+"></el-option>
            <el-option label="75" value="75"></el-option>
        </el-option-group>
        <el-option-group label=">=80">
            <el-option label="80-" value="80-"></el-option>
            <el-option label="80+" value="80+"></el-option>
            <el-option label="85" value="85"></el-option>
            <el-option label="90" value="90-"></el-option>
        </el-option-group>
    </el-select>
</template>

<script>
export default {
    name: "SelectWeaponLevel",
    props: ["modelValue"],
    emits: ["update:modelValue"]
}
</script>