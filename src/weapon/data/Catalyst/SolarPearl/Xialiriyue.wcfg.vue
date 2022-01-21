<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均效果1比例</h3>
            <common-config-slider v-model="rate1"></common-config-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">平均效果2比例</h3>
            <common-config-slider v-model="rate2"></common-config-slider>
        </div>
    </div>
</template>

<script>
import CommonConfigSlider from "../../../common/CommonConfigSlider.vue"


export default {
    components: { CommonConfigSlider },
    name: "匣里日月.wcfg",
    data() {
        return {
            rate1: 0.3,
            rate2: 0.1,
        }
    },
    methods: {
        setData(d) {
            this.rate1 = d.rate1;
            this.rate2 = d.rate2;
        }
    }
}
</script>