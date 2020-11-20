<template>
    <el-card>
        <el-checkbox class="item" v-model="value['countBerserker']">计入战狂4效果（暴击率+24%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countInstructor']">计入教官4效果（元素精通+120）</el-checkbox>
        <el-checkbox class="item" v-model="value['countBraveHeart']">计入勇士之心4效果（伤害加成+30%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countMartialArtist']">计入武人4效果（普攻和重击伤害+30%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countWandererTroupe']">计入流浪大地的乐团4效果（重击伤害加成+35%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countThunderSmoother']">计入平息鸣雷的尊者4效果（伤害加成+35%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countGladiatorFinale']">计入角斗士的终幕礼4效果（普攻伤害加成+35%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countRetracingBolide']">计入逆飞的流星4效果（普攻、重击伤害+40%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countLavaWalker']">计入渡过烈火的贤人4效果（伤害加成+35%）</el-checkbox>
        <el-checkbox class="item" v-model="value['countCrimsonWitch1']">计入炽烈的炎之魔女4效果（火元素伤害加成+100%（按叠加两次计算））</el-checkbox>
    </el-card>
</template>

<script>
// 选择圣遗物参数

export default {
    name: "SelectArtifactParam",
    props: {
        value: {
            type: Object,
        }
    }
}
</script>

<style scoped>
.item {
    margin-bottom: 12px;
    display: block;
}

.item:nth-last-child(1) {
    margin: 0;
}
</style>