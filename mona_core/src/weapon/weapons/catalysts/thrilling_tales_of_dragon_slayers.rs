use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct ThrillingTalesOfDragonSlayers;

impl WeaponTrait for ThrillingTalesOfDragonSlayers {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ThrillingTalesOfDragonSlayers,
        internal_name: "Catalyst_Pulpfic",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "主动切换角色时，新登场的角色攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>，持续10秒。该效果每20秒只能触发一次。",
            en: "When switching characters, the new character taking the field has their ATK increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 10s. This effect can only occur once every 20s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "讨龙英杰谭",
            en: "Thrilling Tales of Dragon Slayers"
        ),
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
