<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">阈值</h3>
            <el-input v-model="value.threshold"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "ZhongliQHP.tcfg",
    props: ["value"],
    first() {
        return {
            threshold: "30000",
        }
    },
    compact(obj) {
        return {
            threshold: parseFloat(obj.threshold) ?? 30000,
        }
    }
}
</script>