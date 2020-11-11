<template>
    <el-button class="addButton" @click="$emit('click')">
        +
    </el-button>
</template>

<script>
export default {
    name: "AddButton",
    
}
</script>

<style scoped>
.addButton {
    height: 165px;
    width: 120px;
    background: #eee;
    font-size: 4rem;
}
</style>