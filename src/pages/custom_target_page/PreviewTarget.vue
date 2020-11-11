<template>
    <div>
        <div>
            <h4>普通攻击</h4>
            <el-alert :title="'频率：' + config.aFreq" class="item" :closable="false"></el-alert>
            <el-alert :title="'元素占比：' + config.aRatio" class="item" :closable="false"></el-alert>
            <el-alert :title="'倍率：' + config.aTimes" class="item" :closable="false"></el-alert>
        </div>
        <div>
            <h4>重击</h4>
            <el-alert :title="'频率：' + config.bFreq" class="item" :closable="false"></el-alert>
            <el-alert :title="'元素占比：' + config.bRatio" class="item" :closable="false"></el-alert>
            <el-alert :title="'倍率：' + config.bTimes" class="item" :closable="false"></el-alert>
        </div>
        <div>
            <h4>元素战技</h4>
            <el-alert :title="'频率：' + config.eFreq" class="item" :closable="false"></el-alert>
            <el-alert :title="'倍率：' + config.eTimes" class="item" :closable="false"></el-alert>
        </div>
        <div>
            <h4>元素爆发</h4>
            <el-alert :title="'频率：' + config.qFreq" class="item" :closable="false"></el-alert>
            <el-alert :title="'倍率：' + config.qTimes" class="item" :closable="false"></el-alert>
        </div>
    </div>
</template>

<script>
export default {
    name: "PreviewTarget",
    props: {
        config: {
            type: Object
        }
    }
}
</script>

<style scoped>
.item {
    margin-bottom: 8px;
}
</style>