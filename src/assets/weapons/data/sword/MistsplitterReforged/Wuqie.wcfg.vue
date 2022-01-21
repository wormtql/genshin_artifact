<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">雾切之巴印层数</h3>
            <el-input-number
                :min="0"
                :max="3"
                v-model="level"
                size="small"
            ></el-input-number>
        </div>
    </div>
</template>

<script>


export default {
    name: "雾切之回光.wcfg",
    data() {
        return {
            level: 1,
        }
    },
    methods: {
        setData(d) {
            this.level = d.level ?? 1;
        }
    }
}
</script>