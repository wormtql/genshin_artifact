use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::buffs::macros::buff_meta_c;

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
    buff_meta_c!(DoriC4 Dori);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_below50",
            title: "b39",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "energy_below50",
            title: "b40",
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