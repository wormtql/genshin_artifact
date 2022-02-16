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
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: WeaponSubStatFamily::ATK72,
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("浮游四方的灵云：元素伤害加成提升12%/15%/18%/21%/24%；普通攻击命中时，有50%的概率获得高天流云的青睐，在15秒内主动攻击附近的敌人，造成等同于160%/200%/240%/280%/320%攻击力的伤害。该效果每30秒只能触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "天空之卷"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardAtlasEffect::new()))
    }
}
