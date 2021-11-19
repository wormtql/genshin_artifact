<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均效果比例</h3>
            <common-config-slider v-model="rate" :max="1"></common-config-slider>
        </div>
    </div>
</template>

<script>
import CommonConfigSlider from "../../../common/CommonConfigSlider.vue"


export default {
    components: { CommonConfigSlider },
    name: "Chenshazhifangchui.wcfg",
    data() {
        return {
            rate: 1,
        }
    },
    methods: {
        setData(d) {
            this.rate = d.rate ?? 1;
        }
    }
}
</script>