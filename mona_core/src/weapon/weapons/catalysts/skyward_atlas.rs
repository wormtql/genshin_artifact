use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct SkywardAtlasEffect;

impl SkywardAtlasEffect {
    pub fn new() -> SkywardAtlasEffect {
        SkywardAtlasEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardAtlasEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.add_elemental_bonus("天空之卷", value);
    }
}

pub struct SkywardAtlas;

impl WeaponTrait for SkywardAtlas {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardAtlas,
        internal_name: "Catalyst_Dvalin",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK72),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素伤害加成提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>；普通攻击命中时，有50%的概率获得高天流云的青睐，在15秒内主动攻击附近的敌人，造成等同于<span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span>攻击力的伤害。该效果每30秒只能触发一次。",
            en: "Increases Elemental DMG Bonus by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. Normal Attack hits have a 50% chance to earn the favor of the clouds, which actively seek out nearby opponents to attack for 15s, dealing <span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span> ATK DMG. Can only occur once every 30s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "天空之卷",
            en: "Skyward Atlas"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardAtlasEffect::new()))
    }
}
