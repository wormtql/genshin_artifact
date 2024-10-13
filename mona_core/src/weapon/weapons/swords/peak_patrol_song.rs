use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct PeakPatrolSongEffect {
    pub stack: f64,
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for PeakPatrolSongEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let def_bonus = 0.06 + 0.02 * refine;
        let ele_bonus = 0.075 + 0.025 * refine;
        attribute.add_def_percentage("岩峰巡歌被动", def_bonus * self.stack);
        attribute.add_elemental_bonus("岩峰巡歌被动", ele_bonus * self.stack);

        let ele_bonus2 = 0.06 + 0.02 * refine;
        let max_bonus2 = 0.192 + 0.064 * refine;
        // let bonus2 = (ele_bonus2 * (self.def / 1000.0).floor()).min(max_bonus2);
        // let bonus2 = (ele_bonus2 * (self.def / 1000.0)).min(max_bonus2);
        //
        // attribute.add_elemental_bonus("岩峰巡歌被动", bonus2 * self.rate);

        let keys = [
            AttributeName::BonusPhysical,
            AttributeName::BonusPyro,
            AttributeName::BonusCryo,
            AttributeName::BonusHydro,
            AttributeName::BonusGeo,
            AttributeName::BonusAnemo,
            AttributeName::BonusDendro,
            AttributeName::BonusElectro,
        ];
        for k in keys {
            let rate = self.rate;
            attribute.add_edge1(
                AttributeName::DEF,
                k,
                Box::new(move |def, _| (ele_bonus2 * (def / 1000.0)).min(max_bonus2) * rate),
                Box::new(|_x, _y, _grad| (0.0, 0.0)),
                "岩峰巡歌被动"
            );
        }
    }
}

pub struct PeakPatrolSong;

impl WeaponTrait for PeakPatrolSong {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PeakPatrolSong,
        internal_name: "Sword_XochitlsTube",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF180),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "普通攻击或下落攻击命中敌人后，将获得「荣花之歌」：防御力提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，并获得<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>所有元素伤害加成，该效果持续6秒，至多叠加2层，每0.1秒至多触发一次。该效果叠加至2层或2层的持续时间刷新时，基于装备者的防御力，每1000点使队伍中附近所有角色的所有元素伤害加成提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，至多提高<span style=\"color: #409EFF;\">25.6%-32%-38.4%-44.8%-51.2%</span>，持续15秒。",
            en: "Gain \"Ode to Flowers\" after Normal or Plunging Attacks hit an opponent: DEF increases by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> and gain a <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> All Elemental DMG Bonus for 6s. Max 2 stacks. Can trigger once per 0.1s. When this effect reaches 2 stacks or the 2nd stack's duration is refreshed, increase all nearby party members' All Elemental DMG Bonus by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> for every 1,000 DEF the equipping character has, up to a maximum of <span style=\"color: #409EFF;\">25.6%-32%-38.4%-44.8%-51.2%</span>, for 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "岩峰巡歌",
            en: "Peak Patrol Song"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "效果1层数",
                en: "Effect 1 Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "满层效果比例",
                en: "Full Stack Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::PeakPatrolSong { rate, stack } => Some(Box::new(PeakPatrolSongEffect {
                rate, stack
            })),
            _ => None
        }
    }
}
