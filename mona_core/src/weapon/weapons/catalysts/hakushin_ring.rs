use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct HakushinRing;

impl WeaponTrait for HakushinRing {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HakushinRing,
        internal_name: "Catalyst_Bakufu",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "装备该武器的角色触发雷元素相关反应后，队伍中附近的与该元素反应相关的元素类型的角色，获得<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>对应元素的元素伤害加成，持续6秒。通过这种方式，角色获得的元素伤害加成无法叠加。",
            en: "After the character equipped with this weapon triggers an Electro elemental reaction, nearby party members of an Elemental Type involved in the elemental reaction receive a <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> Elemental DMG Bonus for their element, lasting 6s. Elemental Bonuses gained in this way cannot be stacked."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "白辰之环",
            en: "Hakushin Ring"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
