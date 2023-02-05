use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct MakhairaAquamarineEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for MakhairaAquamarineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let value = (0.06 * data.refine as f64 + 0.18) * self.rate;

        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::ATKFixed,
            Box::new(move |em, _| value * em),
            Box::new(move |em, _, grad| (value, 0.0)),
            "玛海菈的水色被动"
        );
    }
}

pub struct MakhairaAquamarine;

impl WeaponTrait for MakhairaAquamarine {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MakhairaAquamarine,
        internal_name: "Claymore_Pleroma",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "每10秒，产生如下效果：基于装备者的元素精通的<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>，提升该角色的攻击力，并基于该提升的30%为队伍中附近的其他角色提升攻击力，持续12秒，多件同名武器产生的此效果可以叠加。角色处于队伍后台时也能触发效果。",
            en: "The following effect will trigger every 10s: The equipping character will gain <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> of their Elemental Mastery as bonus ATK for 12s, with nearby party members gaining 30% of this buff for the same duration. Multiple instances of this weapon can allow this buff to stack. This effect will still trigger even if the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "玛海菈的水色",
            en: "Makhaira Aquamarine"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::MakhairaAquamarine { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(MakhairaAquamarineEffect { rate }))
    }
}
