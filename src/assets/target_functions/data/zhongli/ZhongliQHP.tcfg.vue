<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">阈值</h3>
            <el-input
                v-model="threshold"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "ZhongliQHP.tcfg",
    data() {
        return {
            threshold: "30000",
        }
    },
    methods: {
        compact() {
            return {
                threshold: parseFloat(this.threshold) ?? 30000,
            }
        },

        setData(d) {
            this.threshold = d.threshold.toString();
        }
    }
    
}
</script>