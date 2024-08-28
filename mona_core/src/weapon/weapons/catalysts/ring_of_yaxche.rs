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

struct RingOfYaxcheEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for RingOfYaxcheEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let rate = self.rate;
        let step = 0.005 + 0.001 * refine;
        let max = 0.12 + 0.04 * refine;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::BonusNormalAttack,
            Box::new(move |hp, _| ((hp / 1000.0).floor() * step).min(max)),
            Box::new(|_x, _y, _grad| (0.0, 0.0)),
            "木棉之环被动"
        );
    }
}

pub struct RingOfYaxche;

impl WeaponTrait for RingOfYaxche {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RingOfYaxche,
        internal_name: "Catalyst_Isikhulu",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技时，获得「玉锻之冕」效果：每1000点生命值上限都会使普通攻击造成的伤害提升<span style=\"color: #409EFF;\">0.6%-0.7%-0.8%-0.9%-1%</span>，持续10秒。通过这种方式至多使普通攻击造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。",
            en: "Using an Elemental Skill grants the Jade-Forged Crown effect: Every 1,000 Max HP will increase the Normal Attack DMG dealt by the equipping character by <span style=\"color: #409EFF;\">0.6%-0.7%-0.8%-0.9%-1%</span> for 10s. Normal Attack DMG can be increased this way by a maximum of <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "木棉之环",
            en: "Ring of Yaxche"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::RingOfYaxche { rate } => Some(Box::new(RingOfYaxcheEffect {
                rate
            })),
            _ => None
        }
    }
}
