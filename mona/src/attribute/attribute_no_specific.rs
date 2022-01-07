// use std::collections::HashMap;
// use std::cell::RefCell;
//
// use super::attribute::{AttributeName};
// use super::entry::{NodeHandle};
//
// pub struct AttributeNoSpecific {
//     pub attributes: HashMap<AttributeName, RefCell<NodeHandle>>
// }
//
// impl AttributeNoSpecific {
//     pub fn new() -> AttributeNoSpecific {
//         let mut ret = AttributeNoSpecific {
//             attributes: HashMap::new(),
//         };
//
//         ret.add_entry(AttributeName::HealingBonus, 0.0);
//         ret.add_entry(AttributeName::ElementalMastery, 0.0);
//         ret.add_entry(AttributeName::Recharge, 1.0);
//         ret.add_entry(AttributeName::ShieldStrength, 0.0);
//
//         ret.add_entry(AttributeName::HPBase, 0.0);
//         ret.add_entry(AttributeName::HPFixed, 0.0);
//         ret.add_entry(AttributeName::HPPercentage, 0.0);
//
//         ret.add_entry(AttributeName::ATKBase, 0.0);
//         ret.add_entry(AttributeName::ATKFixed, 0.0);
//         ret.add_entry(AttributeName::ATKPercentage, 0.0);
//
//         ret.add_entry(AttributeName::DEFBase, 0.0);
//         ret.add_entry(AttributeName::DEFFixed, 0.0);
//         ret.add_entry(AttributeName::DEFPercentage, 0.0);
//
//         ret.add_entry(AttributeName::CriticalBase, 0.05);
//         ret.add_entry(AttributeName::CriticalNormalAttack, 0.0);
//         ret.add_entry(AttributeName::CriticalChargedAttack, 0.0);
//         ret.add_entry(AttributeName::CriticalPlungingAttack, 0.0);
//         ret.add_entry(AttributeName::CriticalElementalSkill, 0.0);
//         ret.add_entry(AttributeName::CriticalElementalBurst, 0.0);
//         ret.add_entry(AttributeName::CriticalElectro, 0.0);
//         ret.add_entry(AttributeName::CriticalPyro, 0.0);
//         ret.add_entry(AttributeName::CriticalHydro, 0.0);
//         ret.add_entry(AttributeName::CriticalCryo, 0.0);
//         ret.add_entry(AttributeName::CriticalAnemo, 0.0);
//         ret.add_entry(AttributeName::CriticalGeo, 0.0);
//         ret.add_entry(AttributeName::CriticalDendro, 0.0);
//         ret.add_entry(AttributeName::CriticalPhysical, 0.0);
//
//         ret.add_entry(AttributeName::CriticalDamageBase, 0.5);
//         ret.add_entry(AttributeName::CriticalDamageNormalAttack, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageChargedAttack, 0.0);
//         ret.add_entry(AttributeName::CriticalDamagePlungingAttack, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageElementalSkill, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageElementalBurst, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageElectro, 0.0);
//         ret.add_entry(AttributeName::CriticalDamagePyro, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageHydro, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageCryo, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageAnemo, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageGeo, 0.0);
//         ret.add_entry(AttributeName::CriticalDamageDendro, 0.0);
//         ret.add_entry(AttributeName::CriticalDamagePhysical, 0.0);
//
//         ret.add_entry(AttributeName::BonusBase, 0.0);
//         ret.add_entry(AttributeName::BonusNormalAttack, 0.0);
//         ret.add_entry(AttributeName::BonusChargedAttack, 0.0);
//         ret.add_entry(AttributeName::BonusPlungingAttack, 0.0);
//         ret.add_entry(AttributeName::BonusElementalSkill, 0.0);
//         ret.add_entry(AttributeName::BonusElementalBurst, 0.0);
//         ret.add_entry(AttributeName::BonusElectro, 0.0);
//         ret.add_entry(AttributeName::BonusPyro, 0.0);
//         ret.add_entry(AttributeName::BonusHydro, 0.0);
//         ret.add_entry(AttributeName::BonusCryo, 0.0);
//         ret.add_entry(AttributeName::BonusAnemo, 0.0);
//         ret.add_entry(AttributeName::BonusGeo, 0.0);
//         ret.add_entry(AttributeName::BonusDendro, 0.0);
//         ret.add_entry(AttributeName::BonusPhysical, 0.0);
//
//         ret.add_entry(AttributeName::EnhanceOverload, 0.0);
//         ret.add_entry(AttributeName::EnhanceBurning, 0.0);
//         ret.add_entry(AttributeName::EnhanceElectroCharged, 0.0);
//         ret.add_entry(AttributeName::EnhanceSuperconduct, 0.0);
//         ret.add_entry(AttributeName::EnhanceSwirlElectro, 0.0);
//         ret.add_entry(AttributeName::EnhanceSwirlPyro, 0.0);
//         ret.add_entry(AttributeName::EnhanceSwirlHydro, 0.0);
//         ret.add_entry(AttributeName::EnhanceSwirlCryo, 0.0);
//         ret.add_entry(AttributeName::EnhanceSwirlBase, 0.0);
//         ret.add_entry(AttributeName::EnhanceVaporize, 0.0);
//         ret.add_entry(AttributeName::EnhanceMelt, 0.0);
//
//         ret.add_entry(AttributeName::HPRatioBase, 0.0);
//         ret.add_entry(AttributeName::HPRatioNormalAttack, 0.0);
//         ret.add_entry(AttributeName::HPRatioChargedAttack, 0.0);
//         ret.add_entry(AttributeName::HPRatioPlungingAttack, 0.0);
//         ret.add_entry(AttributeName::HPRatioElementalSkill, 0.0);
//         ret.add_entry(AttributeName::HPRatioElementalBurst, 0.0);
//         ret.add_entry(AttributeName::HPRatioElectro, 0.0);
//         ret.add_entry(AttributeName::HPRatioPyro, 0.0);
//         ret.add_entry(AttributeName::HPRatioHydro, 0.0);
//         ret.add_entry(AttributeName::HPRatioCryo, 0.0);
//         ret.add_entry(AttributeName::HPRatioAnemo, 0.0);
//         ret.add_entry(AttributeName::HPRatioGeo, 0.0);
//         ret.add_entry(AttributeName::HPRatioDendro, 0.0);
//         ret.add_entry(AttributeName::HPRatioPhysical, 0.0);
//
//         ret.add_entry(AttributeName::DEFRatioBase, 0.0);
//         ret.add_entry(AttributeName::DEFRatioNormalAttack, 0.0);
//         ret.add_entry(AttributeName::DEFRatioChargedAttack, 0.0);
//         ret.add_entry(AttributeName::DEFRatioPlungingAttack, 0.0);
//         ret.add_entry(AttributeName::DEFRatioElementalSkill, 0.0);
//         ret.add_entry(AttributeName::DEFRatioElementalBurst, 0.0);
//         ret.add_entry(AttributeName::DEFRatioElectro, 0.0);
//         ret.add_entry(AttributeName::DEFRatioPyro, 0.0);
//         ret.add_entry(AttributeName::DEFRatioHydro, 0.0);
//         ret.add_entry(AttributeName::DEFRatioCryo, 0.0);
//         ret.add_entry(AttributeName::DEFRatioAnemo, 0.0);
//         ret.add_entry(AttributeName::DEFRatioGeo, 0.0);
//         ret.add_entry(AttributeName::DEFRatioDendro, 0.0);
//         ret.add_entry(AttributeName::DEFRatioPhysical, 0.0);
//
//         ret.add_entry(AttributeName::ATKRatioBase, 0.0);
//         ret.add_entry(AttributeName::ATKRatioNormalAttack, 0.0);
//         ret.add_entry(AttributeName::ATKRatioChargedAttack, 0.0);
//         ret.add_entry(AttributeName::ATKRatioPlungingAttack, 0.0);
//         ret.add_entry(AttributeName::ATKRatioElementalSkill, 0.0);
//         ret.add_entry(AttributeName::ATKRatioElementalBurst, 0.0);
//         ret.add_entry(AttributeName::ATKRatioElectro, 0.0);
//         ret.add_entry(AttributeName::ATKRatioPyro, 0.0);
//         ret.add_entry(AttributeName::ATKRatioHydro, 0.0);
//         ret.add_entry(AttributeName::ATKRatioCryo, 0.0);
//         ret.add_entry(AttributeName::ATKRatioAnemo, 0.0);
//         ret.add_entry(AttributeName::ATKRatioGeo, 0.0);
//         ret.add_entry(AttributeName::ATKRatioDendro, 0.0);
//         ret.add_entry(AttributeName::ATKRatioPhysical, 0.0);
//
//         ret.add_entry(AttributeName::ExtraDmgBase, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgNormalAttack, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgChargedAttack, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgPlungingAttack, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgElementalSkill, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgElementalBurst, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgElectro, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgPyro, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgHydro, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgCryo, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgAnemo, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgGeo, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgDendro, 0.0);
//         ret.add_entry(AttributeName::ExtraDmgPhysical, 0.0);
//
//         ret
//     }
// }