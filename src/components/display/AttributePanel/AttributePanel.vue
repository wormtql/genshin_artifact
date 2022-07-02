<template>
    <div>
        <h3 class="class">{{ t("stat.base") }}</h3>
        <attribute-item :title="t('stat.lifeStatic')" :composition="attribute.hp"></attribute-item>
        <attribute-item :title="t('stat.attackStatic')" :composition="attribute.atk"></attribute-item>
        <attribute-item :title="t('stat.defendStatic')" :composition="attribute.def"></attribute-item>
        
        <attribute-item :title="t('stat.elementalMastery')" :composition="attribute.elemental_mastery"></attribute-item>

        <h3 class="class">{{ t("stat.advanced") }}</h3>
        <attribute-item :title="t('stat.critical')" :composition="attribute.critical" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.criticalDamage')" :composition="attribute.critical_damage" :percentage="true"></attribute-item>

        <attribute-item :title="t('stat.cureEffect')" :composition="attribute.healing_bonus" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.recharge')" :composition="attribute.recharge" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.shield')" :composition="attribute.shield_strength" :percentage="true"></attribute-item>

        <h3 class="class">{{ t("stat.ele") }}</h3>
        <attribute-item :title="t('stat.fireBonus')" :composition="attribute.bonus_pyro" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.waterBonus')" :composition="attribute.bonus_hydro" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.dendroBonus')" :composition="attribute.bonus_dendro" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.thunderBonus')" :composition="attribute.bonus_electro" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.windBonus')" :composition="attribute.bonus_anemo" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.iceBonus')" :composition="attribute.bonus_cryo" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.rockBonus')" :composition="attribute.bonus_geo" :percentage="true"></attribute-item>
        <attribute-item :title="t('stat.physicalBonus')" :composition="attribute.bonus_physical" :percentage="true"></attribute-item>
    </div>
</template>

<script>
import AttributeItem from "./AttributeItem"
import {useI18n} from "../../../i18n/i18n";

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
    },
    setup() {
        const { t } = useI18n()

        return {
            t
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