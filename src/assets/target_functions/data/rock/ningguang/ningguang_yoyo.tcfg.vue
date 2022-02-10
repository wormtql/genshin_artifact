<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">是否速切</h3>
            <el-switch
                active-text="是"
                inactive-text="否"
                v-model="isQte"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "ningguang_yoyo.tcfg",
    data() {
        return {
            isQte: false,
        }
    },
    methods: {
        setData(d) {
            this.isQte = d.isQte ?? false;
        },
    }
    
}
</script>