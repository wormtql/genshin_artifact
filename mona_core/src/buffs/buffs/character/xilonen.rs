use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::xilonen::XILONEN_SKILL;
use crate::common::Element;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ConfigElements8Multi, ItemConfig, ItemConfigType};

pub struct BuffXilonenE {
    pub elements: Vec<Element>,
    pub skill_level: usize,
}

impl<A: Attribute> Buff<A> for BuffXilonenE {
    fn change_attribute(&self, attribute: &mut A) {
        for &element in self.elements.iter() {
            let attribute_name = AttributeName::res_minus_name_by_element(element);
            attribute.set_value_by(attribute_name, "BUFF: 希诺宁E", XILONEN_SKILL.e_res[self.skill_level - 1]);
        }
    }
}

impl BuffMeta for BuffXilonenE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XilonenE,
        name_locale: locale!(
            zh_cn: "希诺宁-「音火锻淬」",
            en: "Xilonen-「Yohual's Scratch」"
        ),
        image: BuffImage::Avatar(CharacterName::Xilonen),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "「源音采样」激活时，附近的敌人的对应元素抗性降低，同元素类型「源音采样」的效果不能叠加；希诺宁处于队伍后台时，同样能产生该效果。",
            en: "When the Source Samples are active, nearby opponents' corresponding Elemental RES will decrease. Source Sample effects of the same Elemental Type cannot stack. Xilonen can trigger these effects even when off-field."
        )),
        from: BuffFrom::Character(CharacterName::Xilonen),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elements",
            title: locale!(zh_cn: "元素", en: "Elements"),
            config: ItemConfigType::Element8Multi { default: ConfigElements8Multi {
                pyro: false,
                electro: false,
                dendro: false,
                cryo: false,
                anemo: false,
                geo: false,
                hydro: false,
                physical: false,
            } }
        },
        ItemConfig {
            name: "skill_level",
            title: locale!(zh_cn: "技能等级", en: "Skill Level"),
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        match *b {
            BuffConfig::XilonenE { skill_level, elements } => {
                Box::new(BuffXilonenE {
                    skill_level,
                    elements: elements.collect_elements()
                })
            },
            _ => Box::new(BuffXilonenE {
                skill_level: 1,
                elements: Vec::new()
            })
        }
    }
}

pub struct BuffXilonenC2 {
    pub elements: ConfigElements8Multi,
}

impl<A: Attribute> Buff<A> for BuffXilonenC2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.elements.geo {
            attribute.set_value_by(AttributeName::BonusBase, "BUFF: 献予灼原的五重奏", 0.5);
        }
        if self.elements.pyro {
            attribute.add_atk_percentage("BUFF: 献予灼原的五重奏", 0.45);
        }
        if self.elements.hydro {
            attribute.add_hp_percentage("BUFF: 献予灼原的五重奏", 0.45);
        }
        if self.elements.cryo {
            attribute.set_value_by(AttributeName::CriticalDamageBase, "BUFF: 献予灼原的五重奏", 0.6);
        }
    }
}

impl BuffMeta for BuffXilonenC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XilonenC2,
        name_locale: locale!(
            zh_cn: "希诺宁-「献予灼原的五重奏」",
            en: "Xilonen-「Chiucue Mix」"
        ),
        image: BuffImage::Avatar(CharacterName::Xilonen),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "希诺宁携带的岩元素「源音采样」将始终保持激活状态。此外，希诺宁的「源音采样」激活时，将根据「源音采样」的元素类型，使队伍中附近所有元素类型相同的角色获得对应效果：<br>·岩元素：造成的伤害提升50%；<br>·火元素：攻击力提升45%；<br>·水元素：生命值上限提升45%；<br>·冰元素：暴击伤害提升60%；<br>·雷元素：恢复25点元素能量，且元素爆发的冷却时间缩短6秒。",
            en: "Xilonen's Geo Source Samples will always remain active. Additionally, when her Source Samples activate, all nearby party members will gain effects corresponding to the active Source Sample that matches their Elemental Type:<br>·Geo: DMG dealt +50%.<br>·Pyro: ATK +45%.<br>·Hydro: Max HP +45%.<br>·Cryo: CRIT DMG +60%.<br>·Electro: Restore 25 Energy, decrease Elemental Burst CD by 6s."
        )),
        from: BuffFrom::Character(CharacterName::Xilonen),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elements",
            title: locale!(zh_cn: "元素", en: "Elements"),
            config: ItemConfigType::Element8Multi { default: ConfigElements8Multi {
                pyro: false,
                electro: false,
                dendro: false,
                cryo: false,
                anemo: false,
                geo: false,
                hydro: false,
                physical: false,
            } }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let elements = match *b {
            BuffConfig::XilonenC2 { elements } => elements,
            _ => Default::default()
        };
        Box::new(BuffXilonenC2 {
            elements
        })
    }
}

pub struct BuffXilonenC4 {
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffXilonenC4 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = self.def * 0.65;

        attribute.set_value_by(AttributeName::ExtraDmgNormalAttack, "BUFF: 「荣花之赐」", value);
        attribute.set_value_by(AttributeName::ExtraDmgChargedAttack, "BUFF: 「荣花之赐」", value);
        attribute.set_value_by(AttributeName::ExtraDmgPlungingAttack, "BUFF: 「荣花之赐」", value);
    }
}

impl BuffMeta for BuffXilonenC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XilonenC4,
        name_locale: locale!(
            zh_cn: "希诺宁-「荣花之赐」",
            en: "Xilonen-「Blooming Blessing」"
        ),
        image: BuffImage::Avatar(CharacterName::Xilonen),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "拥有「荣花之赐」的角色的普通攻击、重击与下落攻击造成的伤害提升，提升值相当于希诺宁的防御力的65%。该效果将在生效6次或持续时间结束时解除。",
            en: "Characters with Blooming Blessing deal 65% of Xilonen's DEF as increased Normal, Charged, and Plunging Attack DMG. This effect will be removed after triggering 6 times or when the duration ends."
        )),
        from: BuffFrom::Character(CharacterName::Xilonen),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: locale!(zh_cn: "防御力", en: "DEF"),
            config: ItemConfigType::Float { min: 0.0, max: 6000.0, default: 2000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let def = match *b {
            BuffConfig::XilonenC4 { def } => def,
            _ => 0.0
        };

        Box::new(BuffXilonenC4 {
            def
        })
    }
}
