use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;

pub struct TulaytullahsRemembranceEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for TulaytullahsRemembranceEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let interval = 0.036 + 0.012 * refine;
        let value = interval * 10.0_f64.min(self.stack);

        attribute.set_value_by(AttributeName::BonusNormalAttack, "图莱杜拉的回忆被动", value);
    }
}

pub struct TulaytullahsRemembrance;

impl WeaponTrait for TulaytullahsRemembrance {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TulaytullahsRemembrance,
        internal_name: "Catalyst_Alaya",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击速度提升<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>；施放元素战技后的12秒内：普通攻击造成的伤害每1秒提升<span style=\"color: #409EFF;\">4.8%-6.0%-7.2%-8.4%-9.6%</span>；普通攻击命中敌人后，普通攻击造成的伤害提升<span style=\"color: #409EFF;\">9.6%-12%-14.4%-16.8%-19.2%</span>，该效果每0.3秒至多触发1次。持续期间内，普通攻击造成的伤害至多通过上述效果提升至<span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span>。角色退场时将移除效果，再次施放元素战技时会先移除原有的效果。",
            en: "Normal Attack SPD is increased by <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>. After the wielder unleashes an Elemental Skill, Normal Attack DMG will increase by <span style=\"color: #409EFF;\">4.8%-6%-7.2%-8.4%-9.6%</span> every second for 14s. After hitting an opponent with a Normal Attack during this duration, Normal Attack DMG will be increased by <span style=\"color: #409EFF;\">9.6%-12%-14.4%-16.8%-19.2%</span>. This increase can be triggered once every 0.3s. The maximum Normal Attack DMG increase per single duration of the overall effect is <span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span>. The effect will be removed when the wielder leaves the field, and using the Elemental Skill again will reset all DMG buffs."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "图莱杜拉的回忆",
            en: "Tulaytullah's Remembrance"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 7.0 },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::TulaytullahsRemembrance { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(TulaytullahsRemembranceEffect {
            stack
        }))
    }
}