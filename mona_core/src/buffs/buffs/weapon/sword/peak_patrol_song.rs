use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::WeaponName;

pub struct BuffPeakPatrolSong {
    pub refine: usize,
    pub rate: f64,
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffPeakPatrolSong {
    fn change_attribute(&self, attribute: &mut A) {
        let ele_bonus2 = 0.06 + 0.02 * self.refine as f64;
        let max_bonus2 = 0.192 + 0.064 * self.refine as f64;
        // let bonus2 = (ele_bonus2 * (self.def / 1000.0).floor()).min(max_bonus2);
        let bonus2 = (ele_bonus2 * (self.def / 1000.0)).min(max_bonus2);

        attribute.add_elemental_bonus("BUFF: 岩峰巡歌被动", bonus2 * self.rate);
    }
}

impl BuffMeta for BuffPeakPatrolSong {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::PeakPatrolSong,
        name_locale: locale!(
            zh_cn: "岩峰巡歌-「不凋的盛年」",
            en: "Peak Patrol Song-「Halcyon Years Unending」"
        ),
        image: BuffImage::Weapon(WeaponName::PeakPatrolSong),
        genre: BuffGenre::Weapon,
        description: Some(locale!(
            zh_cn: "普通攻击或下落攻击命中敌人后，将获得「荣花之歌」：防御力提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，并获得<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>所有元素伤害加成，该效果持续6秒，至多叠加2层，每0.1秒至多触发一次。该效果叠加至2层或2层的持续时间刷新时，基于装备者的防御力，每1000点使队伍中附近所有角色的所有元素伤害加成提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，至多提高<span style=\"color: #409EFF;\">25.6%-32%-38.4%-44.8%-51.2%</span>，持续15秒。",
            en: "Gain \"Ode to Flowers\" after Normal or Plunging Attacks hit an opponent: DEF increases by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> and gain a <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> All Elemental DMG Bonus for 6s. Max 2 stacks. Can trigger once per 0.1s. When this effect reaches 2 stacks or the 2nd stack's duration is refreshed, increase all nearby party members' All Elemental DMG Bonus by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> for every 1,000 DEF the equipping character has, up to a maximum of <span style=\"color: #409EFF;\">25.6%-32%-38.4%-44.8%-51.2%</span>, for 15s."
        )),
        from: BuffFrom::Weapon(WeaponName::PeakPatrolSong),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: locale!(
                zh_cn: "防御力",
                en: "DEF"
            ),
            config: ItemConfigType::Int { min: 1, max: 6000, default: 1000 },
        },
        ItemConfig {
            name: "refine",
            title: locale!(
                zh_cn: "精炼",
                en: "Refine",
            ),
            config: ItemConfigType::Int { min: 1, max: 5, default: 1 },
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

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate, refine, def) = match *b {
            BuffConfig::PeakPatrolSong { rate, refine, def } => (rate, refine, def),
            _ => (0.0, 1, 0.0)
        };

        Box::new(BuffPeakPatrolSong {
            rate, refine, def
        })
    }
}
