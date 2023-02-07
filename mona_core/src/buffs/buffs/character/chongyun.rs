use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffChongyunTalent2;

impl<A: Attribute> Buff<A> for BuffChongyunTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusCryo, "BUFF: 重云天赋「追冰剑诀」", 0.1);
    }
}

impl BuffMeta for BuffChongyunTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ChongyunTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "重云-「追冰剑诀」",
            en: "Chongyun-「Rimechaser Blade」",
        ),
        image: BuffImage::Avatar(CharacterName::Chongyun),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "重云天赋2：灵刃·重华叠霜领域消失时，会唤出一柄灵刃自动攻击附近的敌人，造成相当于灵刃·重华叠霜技能伤害100%的冰元素范围伤害。被击中的敌人冰元素抗性降低10%，持续8秒。",
            en: "Chongyu Talent2: When the field created by Spirit Blade: Chonghua's Layered Frost disappears, another spirit blade will be summoned to strike nearby opponents, dealing 100% of Chonghua's Layered Frost's Skill DMG as AoE Cryo DMG. Opponents hit by this blade will have their Cryo RES decreased by 10% for 8s.",
        )),
        from: BuffFrom::Character(CharacterName::Chongyun),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffChongyunTalent2)
    }
}
