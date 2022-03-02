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
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("元素爆发造成的伤害提升12/15/18/21/24%。元素爆发命中敌人时，有100%概率召唤大鲔冲击，造成100/125/150/175/200%攻击力的范围伤害。该效果每15秒至多触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "衔珠海皇"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LuxuriousSeaLordEffect::new()))
    }
}
