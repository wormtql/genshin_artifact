use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffYumemizukiMizukiC1 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffYumemizukiMizukiC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::SwirlExtraDmg, "梦见月瑞希C1", 11.0 * self.em);
    }
}

impl BuffMeta for BuffYumemizukiMizukiC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YumemizukiMizukiC1,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「宿雾若水遥」",
            en: "Yumemizuki Mizuki-'In Mist-Like Waters'"
        ),
        image: BuffImage::Avatar(CharacterName::YumemizukiMizuki),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "梦见月瑞希处于梦浮状态下时，每3.5秒将对附近的敌人施加持续3秒的「二十三夜待」效果。处于二十三夜待状态下的敌人受到风元素伤害而触发扩散反应时，将移除该效果，使此次扩散反应对该敌人造成的伤害提升，提升值相当于梦见月瑞希元素精通的1100%。",
            en: "When Yumemizuki Mizuki is in the Dreamdrifter state, she will continuously apply the \"Twenty-Three Nights' Awaiting\" effect to nearby opponents for 3s every 3.5s. When an opponent is affected by Anemo DMG-triggered Swirl reactions while the aforementioned effect is active, the effect will be canceled and this Swirl instance has its DMG against this opponent increased by 900% of Mizuki's Elemental Mastery."
        )),
        from: BuffFrom::Character(CharacterName::YumemizukiMizuki),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "梦见月瑞希的元素精通",
                en: "EM of Mizuki"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::YumemizukiMizukiC1 { em } => em,
            _ => 0.0
        };
        Box::new(BuffYumemizukiMizukiC1 {
            em
        })
    }
}

pub struct BuffYumemizukiMizukiC2 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffYumemizukiMizukiC2 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = self.em * 0.0004;

        attribute.set_value_by(AttributeName::BonusPyro, "梦见月瑞希C2", bonus);
        attribute.set_value_by(AttributeName::BonusHydro, "梦见月瑞希C2", bonus);
        attribute.set_value_by(AttributeName::BonusCryo, "梦见月瑞希C2", bonus);
        attribute.set_value_by(AttributeName::BonusElectro, "梦见月瑞希C2", bonus);
    }
}

impl BuffMeta for BuffYumemizukiMizukiC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YumemizukiMizukiC2,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「缠忆君影梦相见」",
            en: "Yumemizuki Mizuki-'Your Echo I Meet in Dreams'"
        ),
        image: BuffImage::Avatar(CharacterName::YumemizukiMizuki),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "进入梦浮状态时，梦见月瑞希的每点元素精通，会为附近的队伍中所有其他角色提供0.04%火元素、水元素、冰元素与雷元素伤害加成，效果持续至梦浮状态结束。",
            en: "When entering the Dreamdrifter state, every Elemental Mastery point Yumemizuki Mizuki has will increase all nearby party members' Pyro, Hydro, Cryo, and Electro DMG Bonuses by 0.04% until the Dreamdrifter state ends."
        )),
        from: BuffFrom::Character(CharacterName::YumemizukiMizuki),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "梦见月瑞希的元素精通",
                en: "EM of Mizuki"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::YumemizukiMizukiC2 { em } => em,
            _ => 0.0
        };
        Box::new(BuffYumemizukiMizukiC2 {
            em
        })
    }
}
