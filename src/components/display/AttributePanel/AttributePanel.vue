<template>
    <div>
        <h3 class="class">基础属性</h3>
        <attribute-item title="生命值" :composition="attribute.hp"></attribute-item>
        <attribute-item title="攻击力" :composition="attribute.atk"></attribute-item>
        <attribute-item title="防御力" :composition="attribute.def"></attribute-item>
        
        <attribute-item title="元素精通" :composition="attribute.elemental_mastery"></attribute-item>

        <h3 class="class">进阶属性</h3>
        <attribute-item title="暴击率" :composition="attribute.critical" :percentage="true"></attribute-item>
        <attribute-item title="暴击伤害" :composition="attribute.critical_damage" :percentage="true"></attribute-item>

        <attribute-item title="治疗加成" :composition="attribute.healing_bonus" :percentage="true"></attribute-item>
        <attribute-item title="元素充能效率" :composition="attribute.recharge" :percentage="true"></attribute-item>
        <!-- <attribute-item title="护盾强效" :composition="attribute.healing_bonus" :percentage="true"></attribute-item> -->

        <h3 class="class">元素属性</h3>
        <attribute-item title="火元素伤害加成" :composition="attribute.bonus_pyro" :percentage="true"></attribute-item>
        <attribute-item title="水元素伤害加成" :composition="attribute.bonus_hydro" :percentage="true"></attribute-item>
        <attribute-item title="草元素伤害加成" :composition="attribute.bonus_dendro" :percentage="true"></attribute-item>
        <attribute-item title="雷元素伤害加成" :composition="attribute.bonus_electro" :percentage="true"></attribute-item>
        <attribute-item title="风元素伤害加成" :composition="attribute.bonus_anemo" :percentage="true"></attribute-item>
        <attribute-item title="冰元素伤害加成" :composition="attribute.bonus_cryo" :percentage="true"></attribute-item>
        <attribute-item title="岩元素伤害加成" :composition="attribute.bonus_geo" :percentage="true"></attribute-item>
        <attribute-item title="物理伤害加成" :composition="attribute.bonus_physical" :percentage="true"></attribute-item>

        <h3 class="class">元素</h3>
        <!-- <div class="item">
            <span class="title">元素精通</span>
            <span class="value">{{ attribute.elementalMastery }}</span>
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
            <span class="title">扩散增强</span>
            <span class="value">{{ mul100("swirlEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">蒸发增强</span>
            <span class="value">{{ mul100("vaporizeEnhance") }}</span>
        </div>
        <div class="item">
            <span class="title">融化增强</span>
            <span class="value">{{ mul100("meltEnhance") }}</span>
        </div> -->
    </div>
</template>

<script>
import AttributeItem from "./AttributeItem"

export default {
    name: "AttributePanel",
    components: {
        AttributeItem
    },
    props: {
        attribute: {
            type: Object,
        },
    },
    methods: {
        mul100(field) {
            return (this.attribute[field] * 100).toFixed(1) + "%";
        }
    },
    computed: {
        attack() {
            let p = this.attribute;
            return p.attackStatic + p.attackBasic + p.attackPercentage;
        },

        defend() {
            let p = this.attribute;
            return p.defendStatic + p.defendBasic + p.defendPercentage;
        },

        life() {
            let p = this.attribute;
            return p.lifeStatic + p.lifeBasic + p.lifePercentage;
        }
    }
}
</script>

<style lang="scss" scoped>
.attribute {
    margin-bottom: 8px;
}

.class {
    /* background:rgb(74, 99, 211); */
    // padding: 0px 16px;
    // display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    // border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}
</style>