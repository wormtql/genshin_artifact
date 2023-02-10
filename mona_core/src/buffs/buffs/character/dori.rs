use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffDoriC4 {
    pub hp_below50: bool,
    pub energy_below50: bool,
}

impl<A: Attribute> Buff<A> for BuffDoriC4 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.hp_below50 {
            attribute.set_value_by(AttributeName::IncomingHealingBonus, "BUFF: 多莉命座4「酌盈剂虚」", 0.5);
        }
        if self.energy_below50 {
            attribute.set_value_by(AttributeName::Recharge, "BUFF: 多莉命座4「酌盈剂虚」", 0.3);
        }
    }
}

impl BuffMeta for BuffDoriC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DoriC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "多莉-「酌盈剂虚」",
            en: "Dori-「Discretionary Supplement」",
        ),
        image: BuffImage::Avatar(CharacterName::Dori),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "多莉命座4：与灯中幽精相连的角色，依据当前的生命值与元素能量，获得以下提升：<br>·生命值低于50%时，提升50%受治疗加成；<br>·元素能量低于50%时，提升30%元素充能效率。",
            en: "Dori C4: The character connected to the Jinni will obtain the following buffs based on their current HP and Energy:<br>·When their HP is lower than 50%, they gain 50% Incoming Healing Bonus.<br>·When their Energy is less than 50%, they gain 30% Energy Recharge.",
        )),
        from: BuffFrom::Character(CharacterName::Dori)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_below50",
            title: crate::common::i18n::locale!(
                zh_cn: "生命值低于50%",
                en: "HP Below 50%",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "energy_below50",
            title: crate::common::i18n::locale!(
                zh_cn: "元素能量低于50%",
                en: "Energy Below 50%",
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (x, y) = match *b {
            BuffConfig::DoriC4 { hp_below50, energy_below50 } => (hp_below50, energy_below50),
            _ => (false, false)
        };

        Box::new(BuffDoriC4 {
            hp_below50: x,
            energy_below50: y,
        })
    }
}