<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">扩散占比（0-1）</h3>
            <el-alert
                title="打满21次，扩散7次；通常19次，扩散7次"
                :closable="false"
                style="margin-bottom: 16px"
            ></el-alert>
            <el-input
                v-model="swirlRatio"
                size="small"
            ></el-input>
        </div>
        <div class="config-item">
            <h3 class="config-title">染色时机</h3>
            <el-slider
                :min="0"
                :max="1"
                :step="0.01"
                show-input
                input-size="small"
                v-model="swirlTime"
            ></el-slider>
        </div>
        <div class="config-item">
            <h3 class="config-title">翠绿之影4件套：减抗</h3>
            <div class="buff-config-item">
                <h3 class="buff-config-title">染色后吃到的减抗比例</h3>
                <el-slider
                    :min="0"
                    :max="1"
                    :step="0.01"
                    show-input
                    input-size="small"
                    v-model="rate1"
                ></el-slider>
            </div>
            <div class="buff-config-item">
                <h3 class="buff-config-title">扩散吃到的减抗比例</h3>
                <el-slider
                    :min="0"
                    :max="1"
                    :step="0.01"
                    show-input
                    input-size="small"
                    v-model="rate2"
                ></el-slider>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "VentiSwirl.tcfg",
    data() {
        return {
            swirlRatio: "0.389",
            swirlTime: 0.2,
            rate1: 0.9,
            rate2: 1,
        }
    },
    methods: {
        setData(d) {
            this.swirlRatio = d.swirlRatio.toString();
            this.swirlTime = d.swirlTime;
            this.rate1 = d.rate1;
            this.rate2 = d.rate2;
        },
        compact() {
            return {
                swirlRatio: parseFloat(this.swirlRatio) ?? 0.5,
                swirlTime: this.swirlTime,
                rate1: this.rate1,
                rate2: this.rate2,
            }
        }
    }
    
}
</script>