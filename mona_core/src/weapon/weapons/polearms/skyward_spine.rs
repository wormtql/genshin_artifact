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

pub struct SkywardSpineEffect;

impl SkywardSpineEffect {
    pub fn new() -> SkywardSpineEffect {
        SkywardSpineEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardSpineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "天空之脊被动", data.refine as f64 * 0.02 + 0.06);
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "天空之脊被动", 0.12);
    }
}

pub struct SkywardSpine;

impl WeaponTrait for SkywardSpine {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardSpine,
        internal_name: "Pole_Dvalin",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge80),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "暴击率提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，普通攻击速度提升<span style=\"color: #409EFF;\">12%-12%-12%-12%-12%</span>。此外，普通攻击与重击命中敌人时，有<span style=\"color: #409EFF;\">50%-50%-50%-50%-50%</span>概率触发真空刃，在小范围内造成额外<span style=\"color: #409EFF;\">40%-55%-70%-85%-100%</span>攻击力的伤害。该效果每2秒至多触发一次。",
            en: "Increases CRIT Rate by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> and increases Normal ATK SPD by <span style=\"color: #409EFF;\">12%-12%-12%-12%-12%</span>. Additionally, Normal and Charged Attacks hits on opponents have a <span style=\"color: #409EFF;\">50%-50%-50%-50%-50%</span> chance to trigger a vacuum blade that deals <span style=\"color: #409EFF;\">40%-55%-70%-85%-100%</span> of ATK as DMG in a small AoE. This effect can occur no more than once every 2s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "天空之脊",
            en: "Skyward Spine"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardSpineEffect::new()))
    }
}
