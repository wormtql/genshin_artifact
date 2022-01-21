<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均效果层数</h3>
            <common-config-slider v-model="level" :max="5"></common-config-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">平均护盾比例</h3>
            <common-config-slider v-model="rate"></common-config-slider>
        </div>
    </div>
</template>

<script>
import CommonConfigSlider from "../../../common/CommonConfigSlider.vue"


export default {
    components: { CommonConfigSlider },
    name: "无工之剑.wcfg",
    data() {
        return {
            level: 2.5,
            rate: 0,
        }
    },
    methods: {
        setData(d) {
            this.level = d.level;
            this.rate = d.rate;
        }
    }
}
</script>