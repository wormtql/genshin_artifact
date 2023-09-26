use crate::attribute::{Attribute, AttributeName};
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

pub struct BalladOfTheBoundlessBlueEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for BalladOfTheBoundlessBlueEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::BonusNormalAttack, "无垠蔚蓝之歌被动", (0.06 + 0.02 * refine) * self.stack);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "无垠蔚蓝之歌被动", (0.045 + 0.015 * refine) * self.stack);
    }
}

pub struct BalladOfTheBoundlessBlue;

impl WeaponTrait for BalladOfTheBoundlessBlue {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BalladOfTheBoundlessBlue,
        internal_name: "Catalyst_DandelionPoem",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "普通攻击或重击命中敌人后的6秒内，普通攻击造成的伤害提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，重击造成的伤害提升<span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>。该效果至多叠加3次，每0.3秒至多触发一次。",
            en: "Within 6s after Normal or Charged Attacks hit an opponent, Normal Attack DMG will be increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> and Charged Attack DMG will be increased by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>. Max 3 stacks. This effect can be triggered once every 0.3s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "无垠蔚蓝之歌",
            en: "Ballad of the Boundless Blue"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::BalladOfTheBoundlessBlue { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(BalladOfTheBoundlessBlueEffect {
            stack
        }))
    }
}
