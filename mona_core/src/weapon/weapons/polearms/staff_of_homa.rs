use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct StaffOfHomaEffect {
    be50_rate: f64
}

impl StaffOfHomaEffect {
    pub fn new(config: &WeaponConfig) -> StaffOfHomaEffect {
        match *config {
            WeaponConfig::StaffOfHoma { be50_rate } => StaffOfHomaEffect {
                be50_rate
            },
            _ => StaffOfHomaEffect {
                be50_rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for StaffOfHomaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let hp_bonus = refine * 0.05 + 0.15;
        attribute.add_hp_percentage("护摩之杖被动", hp_bonus);
        let atk_bonus_ratio = refine * 0.002 + 0.006 + (refine * 0.002 + 0.008) * self.be50_rate;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * atk_bonus_ratio),
            Box::new(move |grad, _x1, _x2| (grad * atk_bonus_ratio, 0.0)),
            "护摩之杖被动等效"
        );
    }
}

pub struct StaffOfHoma;

impl WeaponTrait for StaffOfHoma {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StaffOfHoma,
        internal_name: "Pole_Homa",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "生命值提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。此外，基于装备该武器的角色生命值上限的<span style=\"color: #409EFF;\">0.8%-1%-1.2%-1.4%-1.6%</span>，获得攻击力加成。当装备该武器的角色生命值低于50%时，进一步获得<span style=\"color: #409EFF;\">1%-1.2%-1.4%-1.6%-1.8%</span>基于生命值上限的攻击力提升。",
            en: "HP increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Additionally, provides an ATK Bonus based on <span style=\"color: #409EFF;\">0.8%-1%-1.2%-1.4%-1.6%</span> of the wielder's Max HP. When the wielder's HP is less than 50%, this ATK Bonus is increased by an additional <span style=\"color: #409EFF;\">1%-1.2%-1.4%-1.6%-1.8%</span> of Max HP."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "护摩之杖",
            en: "Staff of Homa"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "be50_rate",
            title: locale!(
                zh_cn: "生命值低于50%时间比例",
                en: "HP Below 50% Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(StaffOfHomaEffect::new(config)))
    }
}
