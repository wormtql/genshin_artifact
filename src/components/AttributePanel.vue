<template>
    <div>
        <h3 class="class">基础属性</h3>
        <div class="item">
            <span class="title">生命值</span>
            <span class="value">
                {{ life.toFixed(0) }}
                =
                {{ panel.lifeBasic.toFixed(0) }}
                +
                <span class="extra">{{ (panel.lifePercentage + panel.lifeStatic).toFixed(0) }}</span>
            </span>
        </div>
        <div class="item">
            <span class="title">攻击力</span>
            <span class="value">
                {{ attack.toFixed(0) }}
                =
                {{ panel.attackBasic.toFixed(0) }}
                +
                <span class="extra">
                    {{ (panel.attackPercentage + panel.attackStatic).toFixed(0) }}
                </span>
            </span>
        </div>
        <div class="item">
            <span class="title">防御力</span>
            <span class="value">
                {{ defend.toFixed(0) }}
                =
                {{ panel.defendBasic.toFixed(0) }}
                +
                <span class="extra">
                    {{ (panel.defendPercentage + panel.defendStatic).toFixed(0) }}
                </span>
            </span>
        </div>


        <h3 class="class">暴击</h3>
        <div class="item">
            <span class="title">普通攻击暴击率</span>
            <span class="value">{{ mul100("critical") }}</span>
        </div>
        <div class="item">
            <span class="title">重击暴击率</span>
            <span class="value">{{ mul100("bCritical") }}</span>
        </div>
        <div class="item">
            <span class="title">元素战技暴击率</span>
            <span class="value">{{ mul100("eCritical") }}</span>
        </div>
        <div class="item">
            <span class="title">元素爆发暴击率</span>
            <span class="value">{{ mul100("qCritical") }}</span>
        </div>
        <div class="item">
            <span class="title">下落攻击暴击率</span>
            <span class="value">{{ mul100("airCritical") }}</span>
        </div>
        <div class="item">
            <span class="title">暴击伤害</span>
            <span class="value">{{ mul100("criticalDamage") }}</span>
        </div>


        <h3 class="class">伤害加成</h3>
        <div class="item">
            <span class="title">雷元素伤害加成</span>
            <span class="value">{{ mul100("thunderBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">火元素伤害加成</span>
            <span class="value">{{ mul100("fireBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">冰元素伤害加成</span>
            <span class="value">{{ mul100("iceBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">水元素伤害加成</span>
            <span class="value">{{ mul100("waterBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">风元素伤害加成</span>
            <span class="value">{{ mul100("windBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">岩元素伤害加成</span>
            <span class="value">{{ mul100("rockBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">物理伤害加成</span>
            <span class="value">{{ mul100("physicalBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">普通攻击伤害加成</span>
            <span class="value">{{ mul100("aBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">重击伤害加成</span>
            <span class="value">{{ mul100("bBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">元素战技伤害加成</span>
            <span class="value">{{ mul100("eBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">元素爆发伤害加成</span>
            <span class="value">{{ mul100("qBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">下落攻击伤害加成</span>
            <span class="value">{{ mul100("airBonus") }}</span>
        </div>
        <div class="item">
            <span class="title">伤害加成</span>
            <span class="value">{{ mul100("bonus") }}</span>
        </div>


        <h3 class="class">元素抗性</h3>
        <div class="item">
            <span class="title">雷元素抗性</span>
            <span class="value">{{ mul100("thunderRes") }}</span>
        </div>
        <div class="item">
            <span class="title">火元素抗性</span>
            <span class="value">{{ mul100("fireRes") }}</span>
        </div>
        <div class="item">
            <span class="title">冰元素抗性</span>
            <span class="value">{{ mul100("iceRes") }}</span>
        </div>
        <div class="item">
            <span class="title">水元素抗性</span>
            <span class="value">{{ mul100("waterRes") }}</span>
        </div>
        <div class="item">
            <span class="title">风元素抗性</span>
            <span class="value">{{ mul100("windRes") }}</span>
        </div>
        <div class="item">
            <span class="title">岩元素抗性</span>
            <span class="value">{{ mul100("rockRes") }}</span>
        </div>


        <h3 class="class">元素</h3>
        <div class="item">
            <span class="title">元素精通</span>
            <span class="value">{{ panel.elementalMastery }}</span>
        </div>
        <div class="item">
            <span class="title">元素充能效率</span>
            <span class="value">{{ mul100("recharge") }}</span>
        </div>
        <div class="item">
            <span class="title">超载增强</span>
            <span class="value">{{ mul100("overloadEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">燃烧增强</span>
            <span class="value">{{ mul100("burningEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">感电增强</span>
            <span class="value">{{ mul100("electroEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">超导增强</span>
            <span class="value">{{ mul100("superconductEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（雷）增强</span>
            <span class="value">{{ mul100("swirlThunderEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（火）增强</span>
            <span class="value">{{ mul100("swirlFireEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（水）增强</span>
            <span class="value">{{ mul100("swirlWaterEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">扩散（冰）增强</span>
            <span class="value">{{ mul100("swirlIceEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">蒸发增强</span>
            <span class="value">{{ mul100("vaporizeEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">融化增强</span>
            <span class="value">{{ mul100("meltEnhance") }}</span>
        </div>


        <h3 class="class">其他</h3>
        <div class="item">
            <span class="title">治疗加成</span>
            <span class="value">{{ mul100("cureEffect") }}</span>
        </div>
        <div class="item">
            <span class="title">护盾强效</span>
            <span class="value">{{ mul100("shield") }}</span>
        </div>
        <div class="item">
            <span class="title">普通攻击攻速</span>
            <span class="value">{{ mul100("aSpeed") }}</span>
        </div>
        <div class="item">
            <span class="title">重击攻速</span>
            <span class="value">{{ mul100("bSpeed") }}</span>
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
    methods: {
        mul100(field) {
            return (this.panel[field] * 100).toFixed(1) + "%";
        }
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

<style lang="scss" scoped>
.attribute {
    margin-bottom: 8px;
}

.item {
    display: flex;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    /* background: rgba(0, 0, 0, 0.05); */
    padding: 4px;

    .title {
        font-size: 12px;
        color: #555555;
        display: inline-block;
        flex: 1;
    }

    .value {
        font-size: 12px;
        display: inline-block;
        color: #111111;
        
        flex: 1;

        .extra {
            color: #67C23A;
        }
    }
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