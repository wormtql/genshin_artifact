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

struct SurfsUpEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for SurfsUpEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {

        let refine = data.refine as f64;

        attribute.add_hp_percentage("冲浪时光被动", 0.15 + 0.05 * refine);
        attribute.set_value_by(AttributeName::BonusNormalAttack, "冲浪时光被动", (0.09 + 0.03 * refine) * self.stack);
    }
}

pub struct SurfsUp;

impl WeaponTrait for SurfsUp {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SurfsUp,
        internal_name: "Catalyst_MechaPufferfish",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "生命值上限提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。每15秒一次，施放元素战技后的14秒内，产生如下效果：获得4层「炽夏」，每层使普通攻击造成的伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。持续期间内，每1.5秒一次：普通攻击命中敌人后，移除1层；每1.5秒一次：对敌人触发蒸发反应后，增加1层。「炽夏」效果至多叠加4层。",
            en: "Max HP increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Once every 15s, for the 14s after using an Elemental Skill: Gain 4 Scorching Summer stacks. Each stack increases Normal Attack DMG by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. For the duration of the effect, once every 1.5s, lose 1 stack after a Normal Attack hits an opponent; once every 1.5s, gain 1 stack after triggering a Vaporize reaction on an opponent. Max 4 Scorching Summer stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "冲浪时光",
            en: "Surf’s Up"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK04
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::SurfsUp { stack } => Some(Box::new(SurfsUpEffect {
                stack
            })),
            _ => None
        }
    }
}
