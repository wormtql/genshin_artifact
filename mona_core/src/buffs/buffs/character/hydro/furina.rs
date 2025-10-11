use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffFurinaQ {
    pub skill_level: usize,
    pub fanfare_value: f64,
}

impl<A: Attribute> Buff<A> for BuffFurinaQ {
    fn change_attribute(&self, attribute: &mut A) {
        // 基于气氛值的伤害提升
        // 气氛值转化提升伤害比例：0.07% ~ 0.35% (Lv1 ~ Lv15)
        let damage_conversion_rates = [
            0.0007, 0.0009, 0.0011, 0.0013, 0.0015, 0.0017, 0.0019, 0.0021, 
            0.0023, 0.0025, 0.0027, 0.0029, 0.0031, 0.0033, 0.0035
        ];
        
        // 气氛值转化受治疗加成比例：0.01% ~ 0.15% (Lv1 ~ Lv15)
        let healing_conversion_rates = [
            0.0001, 0.0002, 0.0003, 0.0004, 0.0005, 0.0006, 0.0007, 0.0008,
            0.0009, 0.0010, 0.0011, 0.0012, 0.0013, 0.0014, 0.0015
        ];

        let skill_index = (self.skill_level - 1).min(14);
        
        // 伤害提升 = 气氛值 × 转化比例
        let damage_bonus = self.fanfare_value * damage_conversion_rates[skill_index];
        let healing_bonus = self.fanfare_value * healing_conversion_rates[skill_index];

        attribute.set_value_by(AttributeName::BonusBase, "BUFF: 芙宁娜-「万众狂欢」", damage_bonus);
        attribute.set_value_by(AttributeName::HealingBonus, "BUFF: 芙宁娜-「万众狂欢」", healing_bonus);
    }
}

impl BuffMeta for BuffFurinaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FurinaQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "芙宁娜-「万众狂欢」",
            en: "Furina-「Universal Revelry」",
        ),
        image: BuffImage::Avatar(CharacterName::Furina),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "芙宁娜元素爆发：基于芙宁娜持有的「气氛值」，附近的队伍中所有角色造成的伤害提升，受治疗加成提升。气氛值上限为300。",
            en: "Furina Elemental Burst: Based on the \"Fanfare\" stacks Furina possesses, all nearby party members' DMG and Incoming Healing Bonus is increased. Max Fanfare: 300.",
        )),
        from: BuffFrom::Character(CharacterName::Furina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill_level",
            title: crate::common::i18n::locale!(
                zh_cn: "技能等级",
                en: "Skill Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
        ItemConfig {
            name: "fanfare_value",
            title: crate::common::i18n::locale!(
                zh_cn: "气氛值",
                en: "Fanfare Stacks",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 300.0, default: 200.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (skill_level, fanfare_value) = match *b {
            BuffConfig::FurinaQ { skill_level, fanfare_value } => (skill_level, fanfare_value),
            _ => (1, 0.0)
        };
        
        // 将氛围值转换为整数（范围 0-300）
        let fanfare_value = (fanfare_value.round() as usize).min(300) as f64;

        Box::new(BuffFurinaQ {
            skill_level, fanfare_value
        })
    }
}
