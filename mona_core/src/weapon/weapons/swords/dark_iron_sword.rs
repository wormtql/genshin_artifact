use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct DarkIronSwordEffect {
    pub rate: f64
}

impl DarkIronSwordEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for DarkIronSwordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("暗铁剑被动等效", value);
    }
}

pub struct DarkIronSword;

impl WeaponTrait for DarkIronSword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DarkIronSword,
        internal_name: "Sword_Darker",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM31),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "触发超载、超导、感电或雷元素扩散反应后的12秒内，攻击力提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。",
            en: "Upon causing an Overloaded, Superconduct, Electro-Charged, or an Electro-infused Swirl reaction, ATK is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> for 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "暗铁剑",
            en: "Dark Iron Sword"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::DarkIronSword { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(DarkIronSwordEffect::new(rate)))
    }
}
