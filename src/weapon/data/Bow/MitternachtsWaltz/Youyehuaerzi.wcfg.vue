<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均效果1比例</h3>
            <el-alert
                title="普通攻击命中敌人后的5秒内，元素战技造成的伤害提升20%/25%/30%/35%/40%"
                :closable="false"
                style="margin-bottom: 16px"
            ></el-alert>
            <common-config-slider v-model="rate1"></common-config-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">平均效果2比例</h3>
            <el-alert
                title="元素战技命中敌人后的5秒内，普通攻击造成的伤害提升20%/25%/30%/35%/40%"
                :closable="false"
                style="margin-bottom: 16px"
            ></el-alert>
            <common-config-slider v-model="rate2"></common-config-slider>
        </div>
    </div>
</template>

<script>
import CommonConfigSlider from "../../../common/CommonConfigSlider.vue"


export default {
    components: { CommonConfigSlider },
    name: "幽夜华尔兹.wcfg",
    data() {
        return {
            rate1: 0.3,
            rate2: 0.3,
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