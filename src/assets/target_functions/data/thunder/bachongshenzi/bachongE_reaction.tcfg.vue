<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">反应类型</h3>
            <el-radio-group
                v-model="mode"
                size="small"
            >
                <el-radio-button label="Overloaded">超载</el-radio-button>
                <el-radio-button label="ElectroCharged">感电</el-radio-button>
            </el-radio-group>
        </div>

        <div
            class="config-item"
            v-if="mode === 'Overloaded'"
        >
            <h3 class="config-title">一轮3E中触发次数</h3>
            <el-input
                v-model="Overloadedtimes"
                size="small"
            ></el-input>
        </div>

        <div
            class="config-item"
            v-if="mode === 'ElectroCharged'"
        >
            <h3 class="config-title">一轮3E中触发次数</h3>
            <el-input
                v-model="electroChargedTimes"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "bachongE_reaction.tcfg",
    data() {
        return {
            Overloadedtimes: "5.0",
            electroChargedTimes: "8.0",
            mode: "Overloaded",
        }
    },
    methods: {
        compact() {
            return {
                Overloadedtimes:parseFloat(this.Overloadedtimes) ?? 5.0,
                electroChargedTimes:parseFloat(this.electroChargedTimes) ?? 8.0,
                mode: this.mode,
            };
        },
        setData(d) {
            this.Overloadedtimes = d.Overloadedtimes.toString();
            this.electroChargedTimes = d.electroChargedTimes.toString();
            this.mode = d.mode;
        }
    }
    
}
</script>