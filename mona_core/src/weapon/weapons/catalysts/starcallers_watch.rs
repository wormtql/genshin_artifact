use crate::attribute::{Attribute, AttributeName};
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

pub struct StarcallersWatchEffect {
    pub use_buff: bool,
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for StarcallersWatchEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let em_bonus = 25.0 * refine + 75.0;
        attribute.set_value_by(AttributeName::ElementalMastery, "「祭星者之望」被动", em_bonus);

        if self.use_buff {
            let bonus = 0.07 * refine + 0.21;
            attribute.set_value_by(AttributeName::BonusBase, "「祭星者之望」被动", bonus * self.rate);
        }
    }
}

pub struct StarcallersWatch;

impl WeaponTrait for StarcallersWatch {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StarcallersWatch,
        internal_name: "Catalyst_Figurines",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM58),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素精通提升<span style=\"color: #409EFF;\">100-125-150-175-200</span>点。装备者创造护盾后的15秒内，获得「照夜之镜」效果：队伍中自己的当前场上角色对附近的敌人造成的伤害提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，每14秒至多获得一次「照夜之镜」效果。",
            en: "Increases Elemental Mastery by span style=\"color: #409EFF;\">100-125-150-175-200</span>. Gain the \"Mirror of Night\" effect within 15s after the equipping character creates a shield: The current active party member deals <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> increased DMG to nearby opponents. You can gain the \"Mirror of Night\" effect once every 14s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "祭星者之望",
            en: "Starcaller’s Watch"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "use_buff",
            title: locale!(
                zh_cn: "是否开启被动",
                en: "Enable Effect"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "被动比例",
                en: "Effect Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::StarcallersWatch { use_buff, rate } => Some(Box::new(StarcallersWatchEffect {
                use_buff, rate
            })),
            _ => None
        }
    }
}
