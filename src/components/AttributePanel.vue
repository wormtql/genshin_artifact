<template>
    <div>
        <h3 class="class">基础属性</h3>
        <div class="item">
            <span class="title">生命值</span>
            <span class="value">{{ life.toFixed(0) }}</span>
        </div>
        <div class="item">
            <span class="title">攻击力</span>
            <span class="value">{{ attack.toFixed(0) }}</span>
        </div>
        <div class="item">
            <span class="title">防御力</span>
            <span class="value">{{ defend.toFixed(0) }}</span>
        </div>


        <h3 class="class">暴击</h3>
        <div class="item">
            <span class="title">暴击率</span>
            <span class="value">{{ (panel.critical * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">暴击伤害</span>
            <span class="value">{{ (panel.criticalDamage * 100).toFixed(1) + "%" }}</span>
        </div>


        <h3 class="class">元素抗性</h3>
        <div class="item">
            <span class="title">雷元素抗性</span>
            <span class="value">{{ (panel.thunderRes * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">火元素抗性</span>
            <span class="value">{{ (panel.fireRes * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">冰元素抗性</span>
            <span class="value">{{ (panel.iceRes * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">水元素抗性</span>
            <span class="value">{{ (panel.waterRes * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">风元素抗性</span>
            <span class="value">{{ (panel.windRes * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">岩元素抗性</span>
            <span class="value">{{ (panel.rockRes * 100).toFixed(1) + "%" }}</span>
        </div>


        <h3 class="class">元素</h3>
        <div class="item">
            <span class="title">元素精通</span>
            <span class="value">{{ panel.elementalMastery }}</span>
        </div>
        <div class="item">
            <span class="title">元素充能效率</span>
            <span class="value">{{ (panel.recharge * 100).toFixed(1) + "%" }}</span>
        </div>


        <h3 class="class">伤害加成</h3>
        <div class="item">
            <span class="title">雷元素伤害加成</span>
            <span class="value">{{ (panel.thunderBonus * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">火元素伤害加成</span>
            <span class="value">{{ (panel.fireBonus * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">冰元素伤害加成</span>
            <span class="value">{{ (panel.iceBonus * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">水元素伤害加成</span>
            <span class="value">{{ (panel.waterBonus * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">风元素伤害加成</span>
            <span class="value">{{ (panel.windBonus * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">岩元素伤害加成</span>
            <span class="value">{{ (panel.rockBonus * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">物理伤害加成</span>
            <span class="value">{{ (panel.physicalBonus * 100).toFixed(1) + "%" }}</span>
        </div>


        <h3 class="class">其他</h3>
        <div class="item">
            <span class="title">治疗加成</span>
            <span class="value">{{ (panel.cureEffect * 100).toFixed(1) + "%" }}</span>
        </div>
        <div class="item">
            <span class="title">护盾强效</span>
            <span class="value">{{ (panel.shield * 100).toFixed(1) + "%" }}</span>
        </div>
    </div>
</template>

<script>
export default {
    name: "AttributePanel",
    props: {
        panel: {
            type: Object,
        },
    },
    computed: {
        attack() {
            let p = this.panel;
            return p.attackStatic + p.attackBasic + p.attackPercentage;
        },

        defend() {
            let p = this.panel;
            return p.defendStatic + p.defendBasic + p.defendPercentage;
        },

        life() {
            let p = this.panel;
            return p.lifeStatic + p.lifeBasic + p.lifePercentage;
        }
    }
}
</script>

<style scoped>
.attribute {
    margin-bottom: 8px;
}

.item {
    display: flex;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    /* background: rgba(0, 0, 0, 0.05); */
    padding: 4px;
}

.title {
    font-size: 12px;
    color: #555555;
    display: inline-block;
    width: 100px;
    /* background: rgba(0, 0, 0, 0.5); */
    /* color: white; */
}

.value {
    font-size: 12px;
    display: inline-block;
    color: #111111;
    
    flex: 1;
}

.class {
    /* background:rgb(74, 99, 211); */
    padding: 0px 16px;
    display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}
</style>