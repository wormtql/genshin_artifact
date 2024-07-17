use crate::common::{SkillType, Element};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum AttributeName {
    // 自定义数据，应当只用在角色的特定的Effect中，否则容易使用不当，产生冲突
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
    ExtraDmgPlungingAttackLowHigh, // 坠地冲击额外伤害，由于闲云而首次引进
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

    pub fn bonus_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::BonusNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::BonusChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::BonusPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::BonusElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::BonusElementalBurst),
            SkillType::NoneType => None,
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

    pub fn critical_rate_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::CriticalNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::CriticalChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::CriticalPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::CriticalElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::CriticalElementalBurst),
            SkillType::NoneType => None,
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

    pub fn critical_damage_name_by_skill_name(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::CriticalDamageNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::CriticalDamageChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::CriticalDamagePlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::CriticalDamageElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::CriticalDamageElementalBurst),
            SkillType::NoneType => None,
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

    pub fn hp_ratio_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::HPRatioNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::HPRatioChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::HPRatioPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::HPRatioElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::HPRatioElementalBurst),
            SkillType::NoneType => None,
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

    pub fn def_ratio_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::DEFRatioNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::DEFRatioChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::DEFRatioPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::DEFRatioElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::DEFRatioElementalBurst),
            SkillType::NoneType => None,
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

    pub fn atk_ratio_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::ATKRatioNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::ATKRatioChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::ATKRatioPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::ATKRatioElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::ATKRatioElementalBurst),
            SkillType::NoneType => None,
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

    pub fn extra_dmg_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::ExtraDmgNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::ExtraDmgChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::ExtraDmgPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::ExtraDmgElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::ExtraDmgElementalBurst),
            SkillType::NoneType => None,
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
