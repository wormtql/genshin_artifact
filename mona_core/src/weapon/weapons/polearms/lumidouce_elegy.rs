use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct LumidouceElegyEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for LumidouceElegyEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("柔灯挽歌被动", 0.11 + 0.04 * refine);
        attribute.set_value_by(AttributeName::BonusBase, "柔灯挽歌被动", self.stack * (0.13 + 0.05 * refine));
    }
}

pub struct LumidouceElegy;

impl WeaponTrait for LumidouceElegy {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LumidouceElegy,
        internal_name: "Pole_Muguet",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击力提升<span style=\"color: #409EFF;\">15%-19%-23%-27%-31%</span>；装备者对敌人触发燃烧反应或对处于燃烧状态下的敌人造成草元素伤害后，造成的伤害提高<span style=\"color: #409EFF;\">18%-23%-28%-33%-38%</span>。该效果持续8秒，至多叠加2层；叠加至2层或2层的持续时间刷新时，恢复<span style=\"color: #409EFF;\">12-13-14-15-16</span>点元素能量，每12秒至多通过这种方式恢复一次元素能量。上述2种效果角色处于队伍后台时也能触发。",
            en: "ATK increased by <span style=\"color: #409EFF;\">15%-19%-23%-27%-31%</span>. When the equipping character triggers Burning on an opponent or deals Dendro DMG to Burning opponents, the DMG dealt is increased by <span style=\"color: #409EFF;\">18%-23%-28%-33%-38%</span>. This effect lasts for 8s, max 2 stacks. When 2 stacks are reached or when the duration of the 2nd stack is refreshed, restore <span style=\"color: #409EFF;\">12-13-14-15-16</span> Energy. Energy can be restored this way once every 12s. The 2 aforementioned effects can be triggered even when the character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "柔灯挽歌",
            en: "Lumidouce Elegy"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK02
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::LumidouceElegy { stack } => {
                Some(Box::new(LumidouceElegyEffect {
                    stack
                }))
            }
            _ => None
        }
    }
}