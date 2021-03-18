<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">扩散占比（0-1）</h3>
            <el-input v-model="value.swirlRatio"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "VentiSwirl.tcfg",
    props: ["value"],
    first() {
        return {
            swirlRatio: "0.5",
        }
    },
    compact(obj) {
        return {
            swirlRatio: parseFloat(obj.swirlRatio) ?? 0.5,
        }
    }
}
</script>