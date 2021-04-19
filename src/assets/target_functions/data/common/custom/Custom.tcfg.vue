<template>
    <div class="config-item">
        <h3 class="config-title">代码</h3>
        <el-input
            type="textarea"
            :row="10"
            v-model="code"
            size="small"
        ></el-input>
    </div>
</template>

<script>
export default {
    name: "Custom.tcfg",
    data() {
        return {
            code: `return attribute.attack();`,
        }
    },
    methods: {
        setData(d) {
            this.code = d.code;
        }
    }
}
</script>