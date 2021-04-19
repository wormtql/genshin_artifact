<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">扩散占比（0-1）</h3>
            <el-input
                v-model="swirlRatio"
                size="small"
            ></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "VentiSwirl.tcfg",
    data() {
        return {
            swirlRatio: "0.5",
        }
    },
    methods: {
        setData(d) {
            this.swirlRatio = d.swirlRatio.toString();
        },
        compact() {
            return {
                swirlRatio: parseFloat(this.swirlRatio) ?? 0.5,
            }
        }
    }
    
}
</script>