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

pub struct SkywardBladeEffect;

impl SkywardBladeEffect {
    pub fn new() -> SkywardBladeEffect {
        SkywardBladeEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let crit = data.refine as f64 * 0.01 + 0.03;
        attribute.set_value_by(AttributeName::CriticalBase, "天空之刃被动", crit);
    }
}

pub struct SkywardBlade;

impl WeaponTrait for SkywardBlade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardBlade,
        internal_name: "Sword_Dvalin",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge120),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "暴击率提升<span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span>；施放元素爆发时，获得破空之势：移动速度提升<span style=\"color: #409EFF;\">10%-10%-10%-10%-10%</span>，攻击速度提升<span style=\"color: #409EFF;\">10%-10%-10%-10%-10%</span>，普通攻击与重击命中时，额外造成<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>攻击力的伤害，持续12秒。",
            en: "CRIT Rate increased by <span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span>. Gains Skypiercing Might upon using an Elemental Burst: Increases Movement SPD by <span style=\"color: #409EFF;\">10%-10%-10%-10%-10%</span>, increases ATK SPD by <span style=\"color: #409EFF;\">10%-10%-10%-10%-10%</span>, and Normal and Charged hits deal additional DMG equal to <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> of ATK. Skypiercing Might lasts for 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "天空之刃",
            en: "Skyward Blade"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardBladeEffect::new()))
    }
}
