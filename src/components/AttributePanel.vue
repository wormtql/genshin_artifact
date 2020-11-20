<template>
    <div>
        <el-divider>基础属性</el-divider>
        <el-input v-model="panel['life1']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>基础生命值</template>
        </el-input>
        <el-input v-model="panel['life2']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>固定增加生命值</template>
        </el-input>
        <el-input v-model="panel['life3']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>%增加生命值</template>
        </el-input>
        <el-tag class="attribute" effect="dark">总生命值：{{ panel["life1"] + panel["life2"] + panel["life3"] }}</el-tag>

        <el-input v-model="panel['attack1']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>基础攻击力</template>
        </el-input>
        <el-input v-model="panel['attack2']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>固定增加攻击力</template>
        </el-input>
        <el-input v-model="panel['attack3']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>%增加攻击力</template>
        </el-input>
        <el-tag class="attribute" effect="dark">总攻击力：{{ panel["attack1"] + panel["attack2"] + panel["attack3"] }}</el-tag>

        <el-input v-model="panel['defend1']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>基础防御力</template>
        </el-input>
        <el-input v-model="panel['defend2']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>固定增加防御力</template>
        </el-input>
        <el-input v-model="panel['defend3']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>%增加防御力</template>
        </el-input>
        <el-tag class="attribute" effect="dark">总防御力：{{ panel["defend1"] + panel["defend2"] + panel["defend3"] }}</el-tag>
        

        <el-divider>暴击</el-divider>
        <el-input v-model="panel['critical']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>普攻暴击率</template>
        </el-input>
        <el-input v-model="panel['bCritical']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>重击暴击率</template>
        </el-input>
        <el-input v-model="panel['eCritical']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>元素战技暴击率</template>
        </el-input>
        <el-input v-model="panel['qCritical']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>元素爆发暴击率</template>
        </el-input>
        <el-input v-model="panel['criticalDamage']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>暴击伤害</template>
        </el-input>


        <el-divider>元素抗性</el-divider>
        <el-input v-model="panel['thunderRes']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>雷元素抗性</template>
        </el-input>
        <el-input v-model="panel['fireRes']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>火元素抗性</template>
        </el-input>
        <el-input v-model="panel['waterRes']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>水元素抗性</template>
        </el-input>
        <el-input v-model="panel['iceRes']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>冰元素抗性</template>
        </el-input>
        <el-input v-model="panel['windRes']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>风元素抗性</template>
        </el-input>
        <el-input v-model="panel['rockRes']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>岩元素抗性</template>
        </el-input>


        <el-divider>元素</el-divider>
        <el-input v-model="panel['elementalMastery']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>元素精通</template>
        </el-input>
        <el-input v-model="panel['recharge']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>元素充能效率</template>
        </el-input>


        <el-divider>伤害加成</el-divider>
        <el-input v-model="panel['thunderBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>雷元素伤害加成</template>
        </el-input>
        <el-input v-model="panel['fireBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>火元素伤害加成</template>
        </el-input>
        <el-input v-model="panel['waterBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>水元素伤害加成</template>
        </el-input>
        <el-input v-model="panel['iceBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>冰元素伤害加成</template>
        </el-input>
        <el-input v-model="panel['windBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>风元素伤害加成</template>
        </el-input>
        <el-input v-model="panel['rockBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>岩元素伤害加成</template>
        </el-input>
        <el-input v-model="panel['physicalBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>物理伤害加成</template>
        </el-input>
        <el-input v-model="panel['aBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>普攻伤害加成</template>
        </el-input>
        <el-input v-model="panel['bBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>重击伤害加成</template>
        </el-input>
        <el-input v-model="panel['eBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>元素战技伤害加成</template>
        </el-input>
        <el-input v-model="panel['qBonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>元素爆发伤害加成</template>
        </el-input>
        <el-input v-model="panel['bonus']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>伤害加成</template>
        </el-input>


        <el-divider>治疗</el-divider>
        <el-input v-model="panel['cureEffect']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>治疗加成</template>
        </el-input>
        <el-input v-model="panel['curedEffect']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>被治疗加成</template>
        </el-input>


        <el-divider>护盾</el-divider>
        <el-input v-model="panel['shield']" size="small" class="attribute" :disabled="disabled">
            <template #prepend>护盾强效</template>
        </el-input>
    </div>
</template>

<script>
export default {
    name: "AttributePanel",
    props: {
        panel: {
            type: Object,
        },
        disabled: {
            type: Boolean,
            default: true,
        }
    },
}
</script>

<style scoped>
.attribute {
    margin-bottom: 8px;
}
</style>