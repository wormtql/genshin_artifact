use crate::attribute::{Attribute, AttributeCommon};
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

struct WaveridingWhirlEffect {
    rate: f64,
    hydro_count: usize,
}

impl<A: Attribute> WeaponEffect<A> for WaveridingWhirlEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus = 0.15 + 0.05 * refine
            + (0.09 + 0.03 * refine) * (self.hydro_count.min(2) as f64);
        attribute.add_hp_percentage("乘浪的回旋被动", bonus * self.rate);
    }
}

pub struct WaveridingWhirl;

impl WeaponTrait for WaveridingWhirl {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WaveridingWhirl,
        internal_name: "Catalyst_Umpakati",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge133),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "游泳消耗的体力降低15%。此外，施放元素战技后的10秒内，生命值上限提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>，队伍中每存在一位水元素类型的角色，生命值上限将进一步提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，至多通过这种方式提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>，该效果每15秒至多触发一次。",
            en: "Decreases Swimming Stamina consumption by 15%. In addition, for 10s after using an Elemental Skill, Max HP is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. For every Hydro Elemental Type character in the party, Max HP is increased by another <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>, and the maximum increase that can be achieved in this way is <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>. Can be triggered once every 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "乘浪的回旋",
            en: "Waveriding Whirl"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
        ItemConfig {
            name: "hydro_count",
            title: locale!(
                zh_cn: "队伍水元素数量",
                en: "# Hydro Member"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::WaveridingWhirl { rate, hydro_count } => Some(Box::new(WaveridingWhirlEffect {
                rate, hydro_count
            })),
            _ => None
        }
    }
}
