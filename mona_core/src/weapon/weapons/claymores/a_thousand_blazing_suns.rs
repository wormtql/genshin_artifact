use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct AThousandBlazingSunsEffect {
    pub rate1: f64,
    pub rate2: f64
}

impl<A: Attribute> WeaponEffect<A> for AThousandBlazingSunsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let crit_dmg_bonus = refine * 0.05 + 0.15;
        let bonus = crit_dmg_bonus * self.rate1 * (1.0 + 0.75 * self.rate2);
        attribute.set_value_by(AttributeName::CriticalDamageBase, "「焚曜千阳」被动", bonus);

        let atk_bonus = 0.07 * refine + 0.21;
        let bonus = atk_bonus * self.rate1 * (1.0 + 0.75 * self.rate2);
        attribute.add_atk_percentage("「焚曜千阳」被动", bonus);
    }
}

pub struct AThousandBlazingSuns;

impl WeaponTrait for AThousandBlazingSuns {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AThousandBlazingSuns,
        internal_name: "Claymore_RadianceSword",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate24),
        weapon_base: WeaponBaseATKFamily::ATK741,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技或元素爆发时，获得「焚光」效果：暴击伤害提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>，攻击力提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，该效果持续6秒，每10秒至多触发一次。\
            持续期间内，普通攻击或重击造成元素伤害后，将使此次「焚光」效果的持续时间延长2秒，该效果每1秒至多触发一次，至多通过这种方式使持续时间延长6秒。此外，处于夜魂加持状态下时，「焚光」效果提高75%，且「焚光」效果在装备者处于后台时不进行计时。",
            en: "Gain the \"Scorching Brilliance\" effect when using an Elemental Skill or Burst: CRIT DMG increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> and ATK increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> for 6s. This effect can trigger once every 10s. \
            While a \"Scorching Brilliance\" instance is active, its duration is increased by 2s after Normal or Charged attacks deal Elemental DMG. This effect can trigger once every second, and the max duration increase is 6s. \
            Additionally, when the equipping character is in the Nightsoul's Blessing state, \"Scorching Brilliance\" effects are increased by 75%, and its duration will not count down when the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "焚曜千阳",
            en: "A Thousand Blazing Suns"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "被动①比例",
                en: "Effect-1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "被动②比例",
                en: "Effect-2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::AThousandBlazingSuns { rate1, rate2 } => Some(Box::new(AThousandBlazingSunsEffect {
                rate1, rate2
            })),
            _ => None
        }
    }
}
