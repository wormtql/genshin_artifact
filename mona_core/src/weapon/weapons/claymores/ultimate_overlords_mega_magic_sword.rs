use crate::attribute::{Attribute, AttributeCommon};
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

pub struct UltimateOverlordsMegaMagicSwordEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for UltimateOverlordsMegaMagicSwordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = 0.03 * refine + 0.09;
        attribute.add_atk_percentage("「究极霸王超级魔剑」被动", value + self.rate * value);
    }
}

pub struct UltimateOverlordsMegaMagicSword;

impl WeaponTrait for UltimateOverlordsMegaMagicSword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::UltimateOverlordsMegaMagicSword,
        internal_name: "Claymore_Champion",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击力提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。不仅如此！海沫村中曾蒙你帮助的美露莘们的声援心意充满了力量，依照她们的数目，攻击力至多进一步提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。",
            en: "ATK increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. That's not all! The support from all Melusines you've helped in Merusea Village fills you with strength! Based on the number of them you've helped, your ATK is increased by up to an additional <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "「究极霸王超级魔剑」",
            en: "\"Ultimate Overlord’s Mega Magic Sword\""
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::UltimateOverlordsMegaMagicSword { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(UltimateOverlordsMegaMagicSwordEffect {
            rate
        }))
    }
}
