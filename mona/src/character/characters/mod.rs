use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;
use crate::attribute::Attribute;
use super::traits::{CharacterTrait, CharacterConstant, CharacterDamage, CharacterEffect};

pub use albedo::{Albedo};
pub use aloy::{ALOY_STATIC_DATA, ALOY_SKILL};
pub use amber::{Amber};
pub use arataki_itto::{AratakiItto};
pub use barbara::{Barbara};
pub use beidou::{Beidou};
pub use bennett::{BENNETT_SKILL, BENNETT_STATIC_DATA};
pub use chongyun::{CHONGYUN_SKILL, CHONGYUN_STATIC_DATA};
pub use diluc::{DILUC_SKILL, DILUC_STATIC_DATA};
pub use diona::{DIONA_SKILL, DIONA_STATIC_DATA};
pub use eula::{EULA_SKILL, EULA_STATIC_DATA};
pub use fischl::{FISCHL_SKILL, FISCHL_STATIC_DATA};
pub use ganyu::{GANYU_SKILL, GANYU_STATIC_DATA, GanyuEffect, GanyuDamageEnum, GanyuDamage};
pub use gorou::{GOROU_SKILL, GOROU_STATIC_DATA};
pub use hu_tao::{HU_TAO_SKILL, HU_TAO_STATIC_DATA, HuTaoEffect, HuTaoDamageEnum, HuTaoDamage};
pub use jean::{JEAN_SKILL, JEAN_STATIC_DATA};
pub use kaedehara_kazuha::{KAEDEHARA_KAZUHA_SKILL, KAEDEHARA_KAZUHA_STATIC_DATA};
pub use kaeya::{KAEYA_SKILL, KAEYA_STATIC_DATA};
pub use kamisato_ayaka::{KAMISATO_AYAKA_SKILL, KAMISATO_AYAKA_STATIC_DATA};
pub use keqing::{KEQING_SKILL, KEQING_STATIC_DATA};
pub use klee::{KLEE_SKILL, KLEE_STATIC_DATA};
pub use kujou_sara::{KUJOU_SARA_SKILL, KUJOU_SARA_STATIC_DATA};
pub use lisa::{LISA_SKILL, LISA_STATIC_DATA};
pub use mona::{MonaEffect, MONA_STATIC_DATA, MONA_SKILL};
pub use ningguang::{NINGGUANG_SKILL, NINGGUANG_STATIC_DATA};
pub use noelle::{NOELLE_SKILL, NOELLE_STATIC_DATA};
pub use qiqi::{QIQI_SKILL, QIQI_STATIC_DATA};
pub use raiden_shogun::{RaidenShogunEffect, RAIDEN_SHOGUN_SKILL, RAIDEN_SHOGUN_STATIC_DATA};
pub use razor::{RAZOR_SKILL, RAZOR_STATIC_DATA};
pub use rosaria::{ROSARIA_SKILL, ROSARIA_STATIC_DATA};
pub use sangonomiya_kokomi::{SANGONOMIYA_KOKOMI_SKILL, SANGONOMIYA_KOKOMI_STATIC_DATA, SangonomiyaKokomiEffect};
pub use sayu::{SAYU_SKILL, SAYU_STATIC_DATA};
pub use sucrose::{SUCROSE_SKILL, SUCROSE_STATIC_DATA};
pub use tartaglia::{TARTAGLIA_SKILL, TARTAGLIA_STATIC_DATA};
pub use thoma::{THOMA_SKILL, THOMA_STATIC_DATA};
pub use venti::{VENTI_SKILL, VENTI_STATIC_DATA};
pub use xiangling::{XIANGLING_SKILL, XIANGLING_STATIC_DATA};
pub use xiao::{XIAO_SKILL, XIAO_STATIC_DATA};
pub use xingqiu::{XINGQIU_SKILL, XINGQIU_STATIC_DATA};
pub use xinyan::{XINYAN_SKILL, XINYAN_STATIC_DATA};
pub use yanfei::{YANFEI_STATIC_DATA};
pub use yoimiya::{YoimiyaEffect, YOIMIYA_SKILL, YOIMIYA_STATIC_DATA};
pub use zhongli::{ZhongliEffect, ZHONGLI_SKILL, ZHONGLI_STATIC_DATA};
use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;

pub mod albedo;
pub mod aloy;
pub mod amber;
pub mod arataki_itto;
pub mod barbara;
pub mod beidou;
pub mod bennett;
pub mod chongyun;
pub mod diluc;
pub mod diona;
pub mod eula;
pub mod fischl;
pub mod ganyu;
pub mod gorou;
pub mod hu_tao;
pub mod jean;
pub mod kaedehara_kazuha;
pub mod kaeya;
pub mod kamisato_ayaka;
pub mod keqing;
pub mod klee;
pub mod kujou_sara;
pub mod lisa;
pub mod mona;
pub mod ningguang;
pub mod noelle;
pub mod qiqi;
pub mod raiden_shogun;
pub mod razor;
pub mod rosaria;
pub mod sangonomiya_kokomi;
pub mod sayu;
pub mod sucrose;
pub mod tartaglia;
pub mod thoma;
pub mod venti;
pub mod xiangling;
pub mod xiao;
pub mod xingqiu;
pub mod xinyan;
pub mod yanfei;
pub mod yoimiya;
pub mod zhongli;

pub fn get_static_data(name: CharacterName) -> CharacterStaticData {
    match name {
        CharacterName::Albedo => Albedo::STATIC_DATA,
        CharacterName::Aloy => ALOY_STATIC_DATA,
        CharacterName::Amber => Amber::STATIC_DATA,
        CharacterName::AratakiItto => AratakiItto::STATIC_DATA,
        CharacterName::Barbara => Barbara::STATIC_DATA,
        CharacterName::Beidou => Beidou::STATIC_DATA,
        CharacterName::Bennett => BENNETT_STATIC_DATA,
        CharacterName::Chongyun => CHONGYUN_STATIC_DATA,
        CharacterName::Diluc => DILUC_STATIC_DATA,
        CharacterName::Diona => DIONA_STATIC_DATA,
        CharacterName::Eula => EULA_STATIC_DATA,
        CharacterName::Fischl => FISCHL_STATIC_DATA,
        CharacterName::Ganyu => GANYU_STATIC_DATA,
        CharacterName::Gorou => GOROU_STATIC_DATA,
        CharacterName::HuTao => HU_TAO_STATIC_DATA,
        CharacterName::Jean => JEAN_STATIC_DATA,
        CharacterName::KaedeharaKazuha => KAEDEHARA_KAZUHA_STATIC_DATA,
        CharacterName::Kaeya => KAEYA_STATIC_DATA,
        CharacterName::KamisatoAyaka => KAMISATO_AYAKA_STATIC_DATA,
        CharacterName::Keqing => KEQING_STATIC_DATA,
        CharacterName::Klee => KLEE_STATIC_DATA,
        CharacterName::KujouSara => KUJOU_SARA_STATIC_DATA,
        CharacterName::Lisa => LISA_STATIC_DATA,
        CharacterName::Mona => MONA_STATIC_DATA,
        CharacterName::Ningguang => NINGGUANG_STATIC_DATA,
        CharacterName::Noelle => NOELLE_STATIC_DATA,
        CharacterName::Qiqi => QIQI_STATIC_DATA,
        CharacterName::RaidenShogun => RAIDEN_SHOGUN_STATIC_DATA,
        CharacterName::Razor => RAZOR_STATIC_DATA,
        CharacterName::Rosaria => ROSARIA_STATIC_DATA,
        CharacterName::SangonomiyaKokomi => SANGONOMIYA_KOKOMI_STATIC_DATA,
        CharacterName::Sayu => SAYU_STATIC_DATA,
        CharacterName::Sucrose => SUCROSE_STATIC_DATA,
        CharacterName::Tartaglia => TARTAGLIA_STATIC_DATA,
        CharacterName::Thoma => THOMA_STATIC_DATA,
        CharacterName::Venti => VENTI_STATIC_DATA,
        CharacterName::Xiangling => XIANGLING_STATIC_DATA,
        CharacterName::Xiao => XIAO_STATIC_DATA,
        CharacterName::Xingqiu => XINGQIU_STATIC_DATA,
        CharacterName::Xinyan => XINYAN_STATIC_DATA,
        CharacterName::Yanfei => YANFEI_STATIC_DATA,
        CharacterName::Yoimiya => YOIMIYA_STATIC_DATA,
        CharacterName::Zhongli => ZHONGLI_STATIC_DATA,
        _ => panic!("unimplemented character"),
    }
}

pub fn get_effect<T: Attribute>(
    name: CharacterName,
    common_data: &CharacterCommonData,
    config: &CharacterConfig
) -> Option<Box<dyn ChangeAttribute<T>>> {
    match name {
        CharacterName::Amber => Some(Box::new(<Amber as CharacterEffect<T>>::new_effect(common_data, config))),
        CharacterName::Ganyu => Some(Box::new(GanyuEffect::new(config))),
        CharacterName::HuTao => Some(Box::new(HuTaoEffect::new(common_data, config))),
        CharacterName::Mona => Some(Box::new(MonaEffect::new(common_data))),
        CharacterName::RaidenShogun => Some(Box::new(RaidenShogunEffect::new(common_data))),
        CharacterName::SangonomiyaKokomi => Some(Box::new(SangonomiyaKokomiEffect::new())),
        CharacterName::Yoimiya => Some(Box::new(YoimiyaEffect::new(common_data, config))),
        CharacterName::Zhongli => Some(Box::new(ZhongliEffect::new(common_data))),
        _ => None,
    }
}

pub fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, skill_index: usize, skill_config: &CharacterSkillConfig) -> D::Result {
    match context.character_common_data.name {
        CharacterName::Amber => <Amber as CharacterDamage<D>>::damage_internal(context, skill_index, skill_config),
        CharacterName::AratakiItto => <AratakiItto as CharacterDamage<D>>::damage_internal(context, skill_index, skill_config),
        CharacterName::Barbara => <Barbara as CharacterDamage<D>>::damage_internal(context, skill_index, skill_config),
        CharacterName::Beidou => <Beidou as CharacterDamage<D>>::damage_internal(context, skill_index, skill_config),
        _ => <Amber as CharacterDamage<D>>::damage_internal(context, skill_index, skill_config)
    }
}
