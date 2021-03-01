<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">生命值低于50%</h3>
            <el-switch v-model="hpBelow50" active-text="是" inactive-text="否"></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "HutaoConfig",
    data: function () {
        return {
            hpBelow50: false,
        }
    }
}
</script>