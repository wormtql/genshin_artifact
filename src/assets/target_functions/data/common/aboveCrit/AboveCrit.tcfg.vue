<template>
    <div>
        <div class="config-item">
            <h3 class="config-title" style="margin-bottom: 24px">阈值（0-1）</h3>
            <el-input v-model="value.threshold"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "AboveCrit.tcfg",
    props: {
        value: {
            type: Object,
        }
    },
    
    first() {
        return {
            threshold: "0.5",
        }
    },

    compact(obj) {
        return {
            threshold: parseFloat(obj.threshold) ?? 0.5,
        }
    }
}
</script>