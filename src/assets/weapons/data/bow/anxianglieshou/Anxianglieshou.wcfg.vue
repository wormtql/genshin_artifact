<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">在后台的时间（秒）</h3>
            <el-input-number
                v-model="backendSeconds"
                :min="0"
                :max="10"
                size="small"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
export default {
    name: "Anxianglieshou.wcfg",
    data() {
        return {
            backendSeconds: 0,
        }
    },
    methods: {
        setData(d) {
            this.backendSeconds = d.backendSeconds;
        }
    }
}
</script>