<template>
    <div>
        <div class="config-item">
            <h3 class="config-title" style="margin-bottom: 24px">阈值（0-1）</h3>
            <el-input v-model="threshold"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "AboveCritConfig",
    data: function () {
        return {
            threshold: "0.5",
        }
    },
    methods: {
        compact() {
            let threshold = parseFloat(this.threshold) || 0.5;
            return {
                threshold,
            }
        }
    }
}
</script>