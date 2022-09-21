<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-option-group label=">=80">
            <el-option label="90" value="90-"></el-option>
            <el-option label="85" value="85"></el-option>
            <el-option label="80+" value="80+"></el-option>
            <el-option label="80-" value="80-"></el-option>
        </el-option-group>
        <el-option-group label=">=70">
            <el-option label="75" value="75"></el-option>
            <el-option label="70+" value="70+"></el-option>
            <el-option label="70-" value="70-"></el-option>
        </el-option-group>
        <el-option-group label=">=60">
            <el-option label="65" value="65"></el-option>
            <el-option label="60+" value="60+"></el-option>
            <el-option label="60-" value="60-"></el-option>
        </el-option-group>
        <el-option-group label=">=50">
            <el-option label="55" value="55"></el-option>
            <el-option label="50+" value="50+"></el-option>
            <el-option label="50-" value="50-"></el-option>
        </el-option-group>
        <el-option-group label=">=40">
            <el-option label="45" value="45"></el-option>
            <el-option label="40+" value="40+"></el-option>
            <el-option label="40-" value="40-"></el-option>
        </el-option-group>
        <el-option-group label=">=30">
            <el-option label="35" value="35"></el-option>
            <el-option label="30+" value="30+"></el-option>
            <el-option label="30-" value="30-"></el-option>
        </el-option-group>
        <el-option-group label=">=20">
            <el-option label="25" value="25"></el-option>
            <el-option label="20+" value="20+"></el-option>
            <el-option label="20-" value="20-"></el-option>
        </el-option-group>
        <el-option-group label=">=10">
            <el-option label="15" value="15"></el-option>
            <el-option label="10+" value="10+"></el-option>
            <el-option label="10-" value="10-"></el-option>
        </el-option-group>
        <el-option-group label=">=1">
            <el-option label="5" value="5"></el-option>
            <el-option label="1" value="1"></el-option>
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