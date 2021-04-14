<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">在后台的时间（秒）</h3>
            <el-input-number
                v-model="value.backendSeconds"
                :min="0"
                :max="10"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "Anxianglieshou.wcfg",
    props: ["value"],
    first() {
        return {
            backendSeconds: 0,
        }
    }
}
</script>