<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">施放元素爆发的被动覆盖率</h3>
            <el-slider
                :min="0"
                :max="1"
                :step="0.01"
                v-model="rate"
                show-input
            ></el-slider>
        </div>
    </div>
</template>

<script>


export default {
    name: "薙草之稻光.wcfg",
    data() {
        return {
            rate: 0,
        }
    },
    methods: {
        setData(d) {
            this.rate = d.rate ?? 0;
        }
    }
}
</script>