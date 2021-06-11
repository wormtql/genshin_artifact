<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均效果1比例</h3>
            <el-alert
                title="普通攻击命中敌人后的6秒内，重击造成的伤害提升16%/20%/24%/28%/32%"
                :closable="false"
                style="margin-bottom: 16px"
            ></el-alert>
            <common-config-slider v-model="rate1"></common-config-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">平均效果2比例</h3>
            <el-alert
                title="重击命中敌人后的6秒内，攻击力提升8%/10%/12%/14%/16%"
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
    name: "嘟嘟可故事集.wcfg",
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