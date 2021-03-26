<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>帮助</el-breadcrumb-item>
            <el-breadcrumb-item>目标函数参数说明</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div class="body">
            <h1>刻晴-斩尽牛杂/刻晴-物晴</h1>
            <target-func-explanation-item name="敌方雷元素覆盖率" description="敌方被雷元素覆盖时间的比例，这会影响到平雷4的效果"></target-func-explanation-item>
            <target-func-explanation-item
                name="平A、重击、E、Q频率"
                description="这四个都视为技能，如果放了10次技能，平A占4次，那么平A频率为0.4"
            ></target-func-explanation-item>
            <target-func-explanation-item
                name="重击、平A附魔比例"
                description="被刻晴E技能影响的附魔比例，例如平A10下，4下是雷元素的，那么平A附魔比例为0.4，这会影响到伤害加成的取舍"
            ></target-func-explanation-item>

            <h1>胡桃-蝶引来生</h1>
            <target-func-explanation-item
                name="敌方火元素覆盖率"
                description="敌方被火元素覆盖时间的比例，这会影响到渡火4的效果"
            ></target-func-explanation-item>
            <target-func-explanation-item
                name="蒸发占比/融化占比"
                description="所有攻击中，蒸发/融化的比例，例如进行10次攻击（平A与重击都可），2下蒸发，那么蒸发占比为0.2"
            ></target-func-explanation-item>
            <target-func-explanation-item
                name="重击频率"
                description="胡桃开E后，重击的频率，例如进行10次攻击，其中4次重击，那么重击频率为0.4"
            ></target-func-explanation-item>
        </div>
    </div>
</template>

<script>
import TargetFuncExplanationItem from "./TargetFuncExplanationItem";

export default {
    components: {
        TargetFuncExplanationItem,
    },
    name: "TargetFuncExplanationPage"
}
</script>

<style scoped>
.body {
    padding: 0 150px;
}
</style>