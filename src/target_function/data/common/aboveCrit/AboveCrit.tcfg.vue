<template>
    <div>
        <div class="config-item">
            <h3 class="config-title" style="margin-bottom: 24px">阈值（0-1）</h3>
            <el-input
                v-model="threshold"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "AboveCrit.tcfg",
    data() {
        return {
            threshold: "0.5",
        }
    },
    methods: {
        compact() {
            return {
                threshold: parseFloat(this.threshold) ?? 0.5,
            }
        },
        setData(d) {
            this.threshold = d.threshold.toString();
        }
    }
    
}
</script>