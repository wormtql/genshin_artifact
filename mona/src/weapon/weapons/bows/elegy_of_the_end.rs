use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct ElegyOfTheEndEffect {
    rate: f64
}

impl ElegyOfTheEndEffect {
    pub fn new(config: &WeaponConfig) -> ElegyOfTheEndEffect {
        match *config {
            WeaponConfig::ElegyOfTheEnd { rate } => ElegyOfTheEndEffect {
                rate
            },
            _ => ElegyOfTheEndEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ElegyOfTheEndEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let em_bonus = refine * 15.0 + 45.0 + (refine * 25.0 + 75.0) * self.rate;
        attribute.set_value_by(AttributeName::ElementalMastery, "终末嗟叹之诗被动等效", em_bonus);
        let atk_bonus = (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("终末嗟叹之诗被动等效", atk_bonus);
    }
}

pub struct ElegyOfTheEnd;

impl WeaponTrait for ElegyOfTheEnd {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ElegyOfTheEnd,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: WeaponSubStatFamily::Recharge120,
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        effect: Some("离别的思念之歌：飘游风中的「千年的大乐章」的一部分。元素精通提高60/75/90/105/120点；元素战技或元素爆发命中敌人时，角色获得一枚追思之符，每0.2秒内至多触发一次，角色处于队伍后台也能触发。拥有4枚追思之符时，将消耗所有追思之符，使附近的队伍中所有角色获得持续12秒的「千年的大乐章·别离之歌」效果：元素精通提高100/125/150/175/200点，攻击力提升20%/25%/30%/35%/40%。触发后20秒内，无法再次获得追思之符。「千年的大乐章」触发的多种数值效果中，同类数值效果不可叠加。"),
        chs: "终末嗟叹之诗"
    };

    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(ElegyOfTheEndEffect::new(config)))
    }
}
