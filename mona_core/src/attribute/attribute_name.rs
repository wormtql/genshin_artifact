use crate::common::{SkillType, Element};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum AttributeName {
    // 自定义数据
    USER1,
    USER2,

    HealingBonus,
    IncomingHealingBonus,
    ElementalMastery,
    // 不参与精通转换的计算，例如草神天赋不被船桨计算
    ElementalMasteryExtra,
    Recharge,
    ShieldStrength,

    DefMinus,
    DefPenetration,
    ResMinusBase,
    ResMinusElectro,
    ResMinusPyro,
    ResMinusHydro,
    ResMinusCryo,
    ResMinusGeo,
    ResMinusAnemo,
    ResMinusDendro,
    ResMinusPhysical,

    SpeedNormalAttack,
    SpeedChargedAttack,

    HPBase,
    HPFixed,
    HPPercentage,
    HP,

    ATKBase,
    ATKFixed,
    ATKPercentage,
    ATK,

    DEFBase,
    DEFFixed,
    DEFPercentage,
    DEF,

    // not character attributes, but needed
    ATKBonusForOther,
    HealBonusForOther,

    CriticalBase,
    CriticalAttacking,          // critical when attack enemy, but not counted in real panel
    CriticalNormalAttack,
    CriticalChargedAttack,
    CriticalPlungingAttack,
    CriticalElementalSkill,
    CriticalElementalBurst,
    CriticalElectro,
    CriticalPyro,
    CriticalHydro,
    CriticalCryo,
    CriticalAnemo,
    CriticalGeo,
    CriticalDendro,
    CriticalPhysical,

    CriticalDamageBase,
    CriticalDamageNormalAttack,
    CriticalDamageChargedAttack,
    CriticalDamagePlungingAttack,
    CriticalDamageElementalSkill,
    CriticalDamageElementalBurst,
    CriticalDamageElectro,
    CriticalDamagePyro,
    CriticalDamageHydro,
    CriticalDamageCryo,
    CriticalDamageAnemo,
    CriticalDamageGeo,
    CriticalDamageDendro,
    CriticalDamagePhysical,

    BonusBase,
    BonusNormalAttack,
    BonusChargedAttack,
    BonusPlungingAttack,
    BonusElementalSkill,
    BonusElementalBurst,
    BonusElectro,
    BonusPyro,
    BonusHydro,
    BonusCryo,
    BonusAnemo,
    BonusGeo,
    BonusDendro,
    BonusPhysical,
    BonusNormalAndElemental, // 普通攻击&元素伤害 todo 以后应该重构掉

    EnhanceBurgeon,
    EnhanceHyperbloom,
    EnhanceBloom,
    EnhanceOverload,
    EnhanceBurning,
    EnhanceShatter,
    EnhanceElectroCharged,
    EnhanceSuperconduct,
    EnhanceSwirlElectro,
    EnhanceSwirlPyro,
    EnhanceSwirlHydro,
    EnhanceSwirlCryo,
    EnhanceSwirlBase,
    EnhanceVaporize,
    EnhanceMelt,
    EnhanceAggravate,
    EnhanceSpread,

    HPRatioBase,
    HPRatioNormalAttack,
    HPRatioChargedAttack,
    HPRatioPlungingAttack,
    HPRatioElementalSkill,
    HPRatioElementalBurst,
    HPRatioElectro,
    HPRatioPyro,
    HPRatioHydro,
    HPRatioCryo,
    HPRatioAnemo,
    HPRatioGeo,
    HPRatioDendro,
    HPRatioPhysical,

    DEFRatioBase,
    DEFRatioNormalAttack,
    DEFRatioChargedAttack,
    DEFRatioPlungingAttack,
    DEFRatioElementalSkill,
    DEFRatioElementalBurst,
    DEFRatioElectro,
    DEFRatioPyro,
    DEFRatioHydro,
    DEFRatioCryo,
    DEFRatioAnemo,
    DEFRatioGeo,
    DEFRatioDendro,
    DEFRatioPhysical,

    ATKRatioBase,
    ATKRatioNormalAttack,
    ATKRatioChargedAttack,
    ATKRatioPlungingAttack,
    ATKRatioElementalSkill,
    ATKRatioElementalBurst,
    ATKRatioElectro,
    ATKRatioPyro,
    ATKRatioHydro,
    ATKRatioCryo,
    ATKRatioAnemo,
    ATKRatioGeo,
    ATKRatioDendro,
    ATKRatioPhysical,

    ExtraDmgBase,
    ExtraDmgNormalAttack,
    ExtraDmgChargedAttack,
    ExtraDmgPlungingAttack,
    ExtraDmgElementalSkill,
    ExtraDmgElementalBurst,
    ExtraDmgElectro,
    ExtraDmgPyro,
    ExtraDmgHydro,
    ExtraDmgCryo,
    ExtraDmgAnemo,
    ExtraDmgGeo,
    ExtraDmgDendro,
    ExtraDmgPhysical,
}

impl AttributeName {
    pub fn bonus_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::BonusElectro,
            Element::Hydro => AttributeName::BonusHydro,
            Element::Anemo => AttributeName::BonusAnemo,
            Element::Pyro => AttributeName::BonusPyro,
            Element::Cryo => AttributeName::BonusCryo,
            Element::Dendro => AttributeName::BonusDendro,
            Element::Geo => AttributeName::BonusGeo,
            Element::Physical => AttributeName::BonusPhysical,
        }
    }

    pub fn bonus_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::BonusNormalAttack,
            SkillType::ChargedAttack => AttributeName::BonusChargedAttack,
            SkillType::PlungingAttack => AttributeName::BonusPlungingAttack,
            SkillType::ElementalSkill => AttributeName::BonusElementalSkill,
            SkillType::ElementalBurst => AttributeName::BonusElementalBurst,
        }
    }

    pub fn critical_rate_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::CriticalElectro,
            Element::Hydro => AttributeName::CriticalHydro,
            Element::Anemo => AttributeName::CriticalAnemo,
            Element::Pyro => AttributeName::CriticalPyro,
            Element::Cryo => AttributeName::CriticalCryo,
            Element::Dendro => AttributeName::CriticalDendro,
            Element::Geo => AttributeName::CriticalGeo,
            Element::Physical => AttributeName::CriticalPhysical,
        }
    }

    pub fn critical_rate_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::CriticalNormalAttack,
            SkillType::ChargedAttack => AttributeName::CriticalChargedAttack,
            SkillType::PlungingAttack => AttributeName::CriticalPlungingAttack,
            SkillType::ElementalSkill => AttributeName::CriticalElementalSkill,
            SkillType::ElementalBurst => AttributeName::CriticalElementalBurst,
        }
    }

    pub fn critical_damage_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::CriticalDamageElectro,
            Element::Hydro => AttributeName::CriticalDamageHydro,
            Element::Anemo => AttributeName::CriticalDamageAnemo,
            Element::Pyro => AttributeName::CriticalDamagePyro,
            Element::Cryo => AttributeName::CriticalDamageCryo,
            Element::Dendro => AttributeName::CriticalDamageDendro,
            Element::Geo => AttributeName::CriticalDamageGeo,
            Element::Physical => AttributeName::CriticalDamagePhysical,
        }
    }

    pub fn critical_damage_name_by_skill_name(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::CriticalDamageNormalAttack,
            SkillType::ChargedAttack => AttributeName::CriticalDamageChargedAttack,
            SkillType::PlungingAttack => AttributeName::CriticalDamagePlungingAttack,
            SkillType::ElementalSkill => AttributeName::CriticalDamageElementalSkill,
            SkillType::ElementalBurst => AttributeName::CriticalDamageElementalBurst,
        }
    }

    pub fn hp_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::HPRatioElectro,
            Element::Hydro => AttributeName::HPRatioHydro,
            Element::Anemo => AttributeName::HPRatioAnemo,
            Element::Pyro => AttributeName::HPRatioPyro,
            Element::Cryo => AttributeName::HPRatioCryo,
            Element::Dendro => AttributeName::HPRatioDendro,
            Element::Geo => AttributeName::HPRatioGeo,
            Element::Physical => AttributeName::HPRatioPhysical,
        }
    }

    pub fn hp_ratio_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::HPRatioNormalAttack,
            SkillType::ChargedAttack => AttributeName::HPRatioChargedAttack,
            SkillType::PlungingAttack => AttributeName::HPRatioPlungingAttack,
            SkillType::ElementalSkill => AttributeName::HPRatioElementalSkill,
            SkillType::ElementalBurst => AttributeName::HPRatioElementalBurst,
        }
    }

    pub fn def_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::DEFRatioElectro,
            Element::Hydro => AttributeName::DEFRatioHydro,
            Element::Anemo => AttributeName::DEFRatioAnemo,
            Element::Pyro => AttributeName::DEFRatioPyro,
            Element::Cryo => AttributeName::DEFRatioCryo,
            Element::Dendro => AttributeName::DEFRatioDendro,
            Element::Geo => AttributeName::DEFRatioGeo,
            Element::Physical => AttributeName::DEFRatioPhysical,
        }
    }

    pub fn def_ratio_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::DEFRatioNormalAttack,
            SkillType::ChargedAttack => AttributeName::DEFRatioChargedAttack,
            SkillType::PlungingAttack => AttributeName::DEFRatioPlungingAttack,
            SkillType::ElementalSkill => AttributeName::DEFRatioElementalSkill,
            SkillType::ElementalBurst => AttributeName::DEFRatioElementalBurst,
        }
    }

    pub fn atk_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::ATKRatioElectro,
            Element::Hydro => AttributeName::ATKRatioHydro,
            Element::Anemo => AttributeName::ATKRatioAnemo,
            Element::Pyro => AttributeName::ATKRatioPyro,
            Element::Cryo => AttributeName::ATKRatioCryo,
            Element::Dendro => AttributeName::ATKRatioDendro,
            Element::Geo => AttributeName::ATKRatioGeo,
            Element::Physical => AttributeName::ATKRatioPhysical,
        }
    }

    pub fn atk_ratio_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::ATKRatioNormalAttack,
            SkillType::ChargedAttack => AttributeName::ATKRatioChargedAttack,
            SkillType::PlungingAttack => AttributeName::ATKRatioPlungingAttack,
            SkillType::ElementalSkill => AttributeName::ATKRatioElementalSkill,
            SkillType::ElementalBurst => AttributeName::ATKRatioElementalBurst,
        }
    }

    pub fn extra_dmg_name_by_element(element: Element) -> AttributeName {
        match element {
            Element:: Electro => AttributeName::ExtraDmgElectro,
            Element::Hydro => AttributeName::ExtraDmgHydro,
            Element::Anemo => AttributeName::ExtraDmgAnemo,
            Element::Pyro => AttributeName::ExtraDmgPyro,
            Element::Cryo => AttributeName::ExtraDmgCryo,
            Element::Dendro => AttributeName::ExtraDmgDendro,
            Element::Geo => AttributeName::ExtraDmgGeo,
            Element::Physical => AttributeName::ExtraDmgPhysical,
        }
    }

    pub fn extra_dmg_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::ExtraDmgNormalAttack,
            SkillType::ChargedAttack => AttributeName::ExtraDmgChargedAttack,
            SkillType::PlungingAttack => AttributeName::ExtraDmgPlungingAttack,
            SkillType::ElementalSkill => AttributeName::ExtraDmgElementalSkill,
            SkillType::ElementalBurst => AttributeName::ExtraDmgElementalBurst,
        }
    }

    pub fn res_minus_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Cryo => AttributeName::ResMinusCryo,
            Element::Pyro => AttributeName::ResMinusPyro,
            Element::Geo => AttributeName::ResMinusGeo,
            Element::Electro => AttributeName::ResMinusElectro,
            Element::Hydro => AttributeName::ResMinusHydro,
            Element::Anemo => AttributeName::ResMinusAnemo,
            Element::Dendro => AttributeName::ResMinusDendro,
            Element::Physical => AttributeName::ResMinusPhysical,
        }
    }
}
