<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">平均被动触发比例</h3>
            <common-config-slider v-model="rate"></common-config-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">主题曲</h3>
            <el-radio-group v-model="type" size="small">
                <el-radio-button label="recitative">宣叙调</el-radio-button>
                <el-radio-button label="aria">咏叹调</el-radio-button>
                <el-radio-button label="interlude">间奏曲</el-radio-button>
            </el-radio-group>
        </div>
    </div>
</template>

<script>
import CommonConfigSlider from "../../../common/CommonConfigSlider.vue"


export default {
    components: { CommonConfigSlider },
    name: "流浪乐章.wcfg",
    data() {
        return {
            type: "recitative",
            rate: 0.1,
        }
    },
    methods: {
        setData(d) {
            this.type = d.type;
            this.rate = d.rate;
        }
    }
}
</script>