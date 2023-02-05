use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct PrimordialJadeCutterEffect;

impl PrimordialJadeCutterEffect {
    pub fn new() -> PrimordialJadeCutterEffect {
        PrimordialJadeCutterEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for PrimordialJadeCutterEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        let hp_bonus = refine as f64 * 0.05 + 0.15;
        attribute.add_hp_percentage("磐岩结绿被动", hp_bonus);

        let atk_bonus = refine as f64 * 0.003 + 0.009;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * atk_bonus),
            Box::new(move |grad, _x1, _x2| (grad * atk_bonus, 0.0)),
            "磐岩结绿被动"
        );
    }
}

pub struct PrimordialJadeCutter;

impl WeaponTrait for PrimordialJadeCutter {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrimordialJadeCutter,
        internal_name: "Sword_Morax",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate96),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "生命值提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。此外，基于装备该武器的角色生命值上限的<span style=\"color: #409EFF;\">1.2%-1.5%-1.8%-2.1%-2.4%</span>，获得攻击力加成。",
            en: "HP increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Additionally, provides an ATK Bonus based on <span style=\"color: #409EFF;\">1.2%-1.5%-1.8%-2.1%-2.4%</span> of the wielder's Max HP."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "磐岩结绿",
            en: "Primordial Jade Cutter"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PrimordialJadeCutterEffect::new()))
    }
}