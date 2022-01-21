<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-option label="60突破" value="60+"></el-option>
        <el-option label="70突破" value="70+"></el-option>
        <el-option label="80突破" value="80+"></el-option>
        <el-option label="90" value="90"></el-option>
    </el-select>
</template>

<script>
export default {
    name: "SelectWeaponLevel",
    props: ["value"],
}
</script>