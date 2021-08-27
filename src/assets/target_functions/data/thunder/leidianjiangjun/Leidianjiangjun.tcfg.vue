<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">是否处于雷罚恶曜之眼</h3>
            <el-switch
                active-text="是"
                inactive-text="否"
                v-model="underE"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "KeqingNormal.tcfg",
    data() {
        return {
            underE: false,
        }
    },
    methods: {
        setData(d) {
            this.underE = d.underE ?? false;
        },
    }
    
}
</script>