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

pub struct CashflowSupervisionEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for CashflowSupervisionEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_atk_percentage("金流监督被动", 0.12 + 0.04 * refine);
        attribute.set_value_by(AttributeName::BonusNormalAttack, "金流监督被动", (0.012 + 0.04 * refine) * self.stack);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "金流监督被动", (0.105 + 0.035 * refine) * self.stack);
    }
}

pub struct CashflowSupervision;

impl WeaponTrait for CashflowSupervision {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CashflowSupervision,
        internal_name: "Catalyst_Wheatley",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击力提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。当前生命值提升或降低时，普通攻击造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，重击造成的伤害提升<span style=\"color: #409EFF;\">14%-17.5%-21%-24.5%-28%</span>。该效果持续4秒，至多叠加3层，每0.3秒至多触发一次；处于叠加3层的状态下时，攻击速度提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>。",
            en: "ATK is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. When current HP increases or decreases, Normal Attack DMG will be increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> and Charged Attack DMG will be increased by <span style=\"color: #409EFF;\">14%-17.5%-21%-24.5%-28%</span> for 4s. Max 3 stacks. This effect can be triggered once every 0.3s. When the wielder has 3 stacks, ATK SPD will be increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "金流监督",
            en: "Cashflow Supervision"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::CashflowSupervision { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(CashflowSupervisionEffect {
            stack
        }))
    }
}
