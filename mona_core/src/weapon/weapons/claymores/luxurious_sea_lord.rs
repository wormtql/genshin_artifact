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

pub struct LuxuriousSeaLordEffect;

impl LuxuriousSeaLordEffect {
    pub fn new() -> LuxuriousSeaLordEffect {
        LuxuriousSeaLordEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for LuxuriousSeaLordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusElementalBurst, "衔珠海皇被动", value);
    }
}

pub struct LuxuriousSeaLord;

impl WeaponTrait for LuxuriousSeaLord {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LuxuriousSeaLord,
        internal_name: "Claymore_MillenniaTuna",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素爆发造成的伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。元素爆发命中敌人时，有100%概率召唤大鲔冲击，造成<span style=\"color: #409EFF;\">100%-125%-150%-175%-200%</span>攻击力的范围伤害。该效果每15秒至多触发一次。",
            en: "Increases Elemental Burst DMG by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. When Elemental Burst hits opponents, there is a 100% chance of summoning a huge onrush of tuna that deals <span style=\"color: #409EFF;\">100%-125%-150%-175%-200%</span> ATK as AoE DMG. This effect can occur once every 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "衔珠海皇",
            en: "Luxurious Sea-Lord"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LuxuriousSeaLordEffect::new()))
    }
}
