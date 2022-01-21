<template>
    <div>
        <div class="config-item">
            <h3 class="config-title">身边有2个及以上敌人</h3>
            <el-switch
                v-model="enemyAbove2"
                active-text="是"
                inactive-text="否"
            ></el-switch>
        </div>
    </div>
</template>

<script>
export default {
    name: "JuedouzhiqiangConfig.wcfg",
    data() {
        return {
            enemyAbove2: true,
        }
    },
    methods: {
        setData(d) {
            this.enemyAbove2 = d.enemyAbove2;
        }
    }
}
</script>