<template>
    <div class="config-item">
        <h3 class="config-title">代码</h3>
        <el-input type="textarea" :row="10" v-model="code"></el-input>
    </div>
</template>

<script>
export default {
    name: "CustomConfig",
    data: function () {
        return {
            code: `return attribute.attack();`
        }
    }
}
</script>