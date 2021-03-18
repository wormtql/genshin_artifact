<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">应用效果（未被击中）</h3>
            <el-switch v-model="value.undamaged" active-text="是" inactive-text="否"></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "Anxiangshanguang.wcfg",
    props: ["value"],

    first() {
        return {
            undamaged: true,
        }
    }
}
</script>