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

pub struct TomeOfTheEternalFlowEffect {
    pub stack: f64
}

impl<A: Attribute> WeaponEffect<A> for TomeOfTheEternalFlowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_hp_percentage("万世流涌大典被动", 0.12 + refine * 0.04);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "万世流涌大典被动", (0.1 + 0.04 * refine) * self.stack);
    }
}

pub struct TomeOfTheEternalFlow;

impl WeaponTrait for TomeOfTheEternalFlow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TomeOfTheEternalFlow,
        internal_name: "Catalyst_Iudex",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "生命值提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。当前生命值提升或降低时，重击造成的伤害提升<span style=\"color: #409EFF;\">14%-18%-22%-26%-30%</span>。该效果持续4秒，至多叠加3次，每0.3秒至多触发一次；叠加至3层或3层的持续时间刷新时，恢复<span style=\"color: #409EFF;\">8-9-10-11-12</span>点元素能量，每12秒至多通过这种方式恢复一次元素能量。",
            en: "HP is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. When current HP increases or decreases, Charged Attack DMG will be increased by <span style=\"color: #409EFF;\">14%-18%-22%-26%-30%</span> for 4s. Max 3 stacks. This effect can be triggered once every 0.3s. When the character has 3 stacks or a third stack's duration refreshes, <span style=\"color: #409EFF;\">8-9-10-11-12</span> Energy will be restored. This Energy restoration effect can be triggered once every 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "万世流涌大典",
            en: "Tome of the Eternal Flow"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::TomeOfTheEternalFlow { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(TomeOfTheEternalFlowEffect {
            stack
        }))
    }
}
