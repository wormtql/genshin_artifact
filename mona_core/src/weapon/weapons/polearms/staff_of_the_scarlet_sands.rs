use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct StaffOfTheScarletSandsEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for StaffOfTheScarletSandsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let value = (0.13 * refine + 0.39) + (0.07 * refine + 0.21) * self.stack;
        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::ATKFixed,
            Box::new(move |em, _| em * value),
            Box::new(move |em, _, grad| (value * grad, 0.0)),
            "赤沙之杖被动等效",
        );
    }
}

pub struct StaffOfTheScarletSands;

impl WeaponTrait for StaffOfTheScarletSands {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StaffOfTheScarletSands,
        internal_name: "Pole_Deshret",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate96),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "基于装备者元素精通的<span style=\"color: #409EFF;\">52%-65%-78%-91%-104%</span>，获得攻击力加成。元素战技命中敌人时，将产生持续10秒的「赤沙之梦」效果：基于装备者元素精通的<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，获得攻击力加成，该效果至多叠加3层。",
            en: "The equipping character gains <span style=\"color: #409EFF;\">52%-65%-78%-91%-104%</span> of their Elemental Mastery as bonus ATK. When an Elemental Skill hits opponents, the Dream of the Scarlet Sands effect will be gained for 10s: The equipping character will gain <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> of their Elemental Mastery as bonus ATK. Max 3 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "赤沙之杖",
            en: "Staff of the Scarlet Sands"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::StaffOfTheScarletSands { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(StaffOfTheScarletSandsEffect { stack }))
    }
}
