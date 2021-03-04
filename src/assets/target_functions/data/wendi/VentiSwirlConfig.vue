<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">扩散占比（0-1）</h3>
            <el-input v-model="swirlRatio"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "VentiSwirlConfig",
    data: function () {
        return {
            swirlRatio: "0.5",
        }
    },
    methods: {
        compact() {
            return {
                swirlRatio: parseFloat(this.swirlRatio),
            }
        }
    }
}
</script>