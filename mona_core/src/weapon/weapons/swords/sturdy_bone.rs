use crate::attribute::{Attribute, AttributeName};
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

pub struct SturdyBoneEffect {
    rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for SturdyBoneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus = 0.12 + 0.04 * refine;

        attribute.set_value_by(AttributeName::ATKBase, "弥坚骨被动", bonus * self.rate);
    }
}

pub struct SturdyBone;

impl WeaponTrait for SturdyBone {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SturdyBone,
        internal_name: "Sword_Umpakati",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "冲刺或替代冲刺的能力消耗的体力降低15%；此外，使用冲刺或替代冲刺的能力后，普通攻击造成的伤害提高，提高数值相当于攻击力的<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，该效果在生效18次或7秒后消失。",
            en: "Sprint or Alternate Sprint Stamina Consumption decreased by 15%. Additionally, after using Sprint or Alternate Sprint, Normal Attack DMG is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> of ATK. This effect expires after triggering 18 times or 7s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "弥坚骨",
            en: "Sturdy Bone"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::SturdyBone { rate } => Some(Box::new(SturdyBoneEffect {
                rate
            })),
            _ => None
        }
    }
}
