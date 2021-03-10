<template>
    <div>
        <el-alert
            title="事实上，这些参数没那么重要，基本上跳过就可以"
            :closable="false"
            type="warning"
            class="hint"
        ></el-alert>
        <div class="config-item">
            <h3 class="config-title">基础攻击力</h3>
            <el-alert
                title="指角色+武器的基础攻击力"
                :closable="false"
                class="hint"
            ></el-alert>
            <el-input v-model="baseAtk"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">当前攻击力</h3>
            <el-alert
                title="可以认为是除去副词条的攻击力"
                :closable="false"
                class="hint"
            ></el-alert>
            <el-input v-model="atk"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">基础生命值</h3>
            <el-input v-model="baseHP"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">当前生命值</h3>
            <el-input v-model="hp"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">基础暴击率</h3>
            <!-- <el-alert
                title="如果想要更加精确的结果，这里填不穿任何圣遗物时的暴击率"
                :closable="false"
                class="hint"
            ></el-alert> -->
            <el-input v-model="baseCrit"></el-input>
        </div>

        <div class="config-item">
            <h3 class="config-title">基础暴击伤害</h3>
            <!-- <el-alert
                title="如果想要更加精确的结果，这里填不穿任何圣遗物时的暴击伤害"
                :closable="false"
                class="hint"
            ></el-alert> -->
            <el-input v-model="baseCriticalDamage"></el-input>
        </div>
    </div>
</template>

<script>
export default {
    name: "HutaoEPotentialConfig",
    data: function () {
        return {
            baseAtk: "600",
            atk: "1400",
            baseCrit: "0.3",
            baseCriticalDamage: "1",
            baseHP: "15000",
            hp: "20000",
        }
    },
    methods: {
        compact() {
            return {
                baseAtk: parseFloat(this.baseAtk),
                baseCrit: parseFloat(this.baseCrit),
                baseCriticalDamage: parseFloat(this.baseCriticalDamage),
                atk: parseFloat(this.atk),
                baseHP: parseFloat(this.baseHP),
                hp: parseFloat(this.hp),
            }
        }
    }
}
</script>

<style scoped>
.hint {
    margin-bottom: 16px;
}
</style>