use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct Frostbearer;

impl WeaponTrait for Frostbearer {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Frostbearer,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: WeaponSubStatFamily::ATK90,
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        effect: Some("霜葬：普通攻击与重击命中敌人时，有60%/70%/80%/90/100%概率在敌人上方生成恒冰晶核并坠落，造成80%/95%/110%/125%/140%攻击力的范围伤害。若敌人处于冰元素影响下，则造成200%/240%/280%/320%/360%攻击力的伤害。该效果每10秒至多触发一次。"),
        chs: "忍冬之果"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
