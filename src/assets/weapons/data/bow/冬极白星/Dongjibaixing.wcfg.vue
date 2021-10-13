<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">「白夜极星」层数</h3>
            <el-input-number
                :min="0"
                :max="4"
                v-model="level"
                size="small"
            ></el-input-number>
        </div>
    </div>
</template>

<script>


export default {
    name: "Dongjibaixing.wcfg",
    data() {
        return {
            level: 2,
        }
    },
    methods: {
        setData(d) {
            this.level = d.level ?? 2;
        }
    }
}
</script>