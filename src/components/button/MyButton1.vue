<template>
    <span class="button-root" @click="$emit('click')">
        <i :class="[icon]"></i>
    </span>
</template>

<script>
export default {
    name: "MyButton1",
    props: {
        icon: {
            default: "el-icon-delete"
        }
    }
}
</script>

<style scoped lang="scss">
.button-root {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    cursor: pointer;

    &:hover {
        background-color: #00000011;
        
    }

    i {
        // font-size: 12px;
        color: #409eff;
        
    }
}
</style>