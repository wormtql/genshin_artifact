<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">是否启用被动</h3>
            <el-switch
                v-model="value.passive"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "Fenghuazhisong.wcfg",
    props: ["value"],
    first() {
        return {
            passive: false,
        }
    }
}
</script>