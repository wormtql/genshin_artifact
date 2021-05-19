<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">「千年的大乐章·揭旗之歌」</h3>
            <el-switch
                v-model="passive"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "QianyangujianConfig.wcfg",
    data() {
        return {
            passive: false,
        }
    },
    methods: {
        setData(d) {
            this.passive = d.passive ?? false;
        }
    }
}
</script>