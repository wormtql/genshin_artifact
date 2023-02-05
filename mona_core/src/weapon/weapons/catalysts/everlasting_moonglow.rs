use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct EverlastingMoonglowEffect;

impl EverlastingMoonglowEffect {
    pub fn new() -> EverlastingMoonglowEffect {
        EverlastingMoonglowEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for EverlastingMoonglowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::HealingBonus, "不灭月华被动", refine * 0.025 + 0.075);
        attribute.set_value_by(AttributeName::HPRatioNormalAttack, "不灭月华被动", refine * 0.005 + 0.005);
    }
}

pub struct EverlastingMoonglow;

impl WeaponTrait for EverlastingMoonglow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EverlastingMoonglow,
        internal_name: "Catalyst_Kaleido",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "治疗加成提升<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>；普通攻击造成的伤害增加，增加值为装备该武器的角色生命值上限的<span style=\"color: #409EFF;\">1%-1.5%-2%-2.5%-3%</span>。在施放元素爆发后的12秒内，普通攻击命中敌人时恢复0.6点元素能量，每0.1秒至多通过这种方式恢复一次元素能量。",
            en: "Healing Bonus increased by <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>, Normal Attack DMG is increased by <span style=\"color: #409EFF;\">1%-1.5%-2%-2.5%-3%</span> of the Max HP of the character equipping this weapon. For 12s after using an Elemental Burst, Normal Attacks that hit opponents will restore 0.6 Energy. Energy can be restored this way once every 0.1s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "不灭月华",
            en: "Everlasting Moonglow"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(EverlastingMoonglowEffect::new()))
    }
}
