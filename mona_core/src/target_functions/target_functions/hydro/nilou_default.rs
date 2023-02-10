use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Nilou;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::reaction_type::TransformativeType;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::transformative_damage::{get_em_bonus, get_transformative_base, transformative_damage};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct NilouDefaultTargetFunction {
    pub e_ratio: f64,
    pub q_ratio: f64,
    pub bloom_ratio: f64,
    pub other_em: f64,
    pub other_bloom_ratio: f64,
}

impl TargetFunction for NilouDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            enemy, attribute
        };

        type S = <Nilou as CharacterTrait>::DamageEnumType;
        let dmg_e1 = Nilou::damage::<SimpleDamageBuilder>(
            &context, S::E1, &CharacterSkillConfig::NoConfig, None
        ).normal.expectation;

        let dmg_q1 = Nilou::damage::<SimpleDamageBuilder>(
            &context, S::Q1, &CharacterSkillConfig::NoConfig, None
        ).normal.expectation;

        let bloom = transformative_damage(character.common_data.level, attribute, enemy).bloom;

        let other_bloom = {
            let bloom_base = get_transformative_base(character.common_data.level, TransformativeType::Bloom);
            let res_ratio = enemy.get_resistance_ratio(Element::Dendro, 0.0);
            let em = self.other_em + 100.0;
            let bonus_em = get_em_bonus(em);
            let hp = attribute.get_value(AttributeName::HP);
            let bonus_hp = (((hp - 30000.0) / 1000.0).floor() * 0.009).clamp(0.0, 4.0);
            let bonus = bonus_em + bonus_hp;

            bloom_base * res_ratio * (1.0 + bonus)
        };

        dmg_e1 * self.e_ratio + dmg_q1 * self.q_ratio + bloom * self.bloom_ratio + other_bloom * self.other_bloom_ratio
    }
}

impl TargetFunctionMetaTrait for NilouDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::NilouDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "妮露-莲光落舞筵",
            en: "Nilou-Dance of Lotuslight"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出妮露",
            en: "Nilou DPS"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Nilou),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_ratio",
            title: crate::common::i18n::locale!(
                zh_cn: "元素战技倍数",
                en: "Elemental Skill Times",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 5.0 },
        },
        ItemConfig {
            name: "q_ratio",
            title: crate::common::i18n::locale!(
                zh_cn: "元素爆发倍数",
                en: "Elemental Burst Times",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 1.0 },
        },
        ItemConfig {
            name: "bloom_ratio",
            title: crate::common::i18n::locale!(
                zh_cn: "绽放倍数",
                en: "Bloom Times",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 3.0 },
        },
        ItemConfig {
            name: "other_em",
            title: crate::common::i18n::locale!(
                zh_cn: "队友的等效精通",
                en: "Teammates' equivalent EM",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3000.0, default: 1000.0 }
        },
        ItemConfig {
            name: "other_bloom_ratio",
            title: crate::common::i18n::locale!(
                zh_cn: "队友的绽放倍数",
                en: "Teammates' Bloom Times",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 7.0 },
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (e_ratio, q_ratio, bloom_ratio, other_em, other_bloom_ratio) = match *config {
            TargetFunctionConfig::NilouDefault { e_ratio, q_ratio, bloom_ratio, other_em, other_bloom_ratio } =>
                (e_ratio, q_ratio, bloom_ratio, other_em, other_bloom_ratio),
            _ => (0.0, 0.0, 0.0, 0.0, 0.0)
        };
        Box::new(NilouDefaultTargetFunction {
            e_ratio, q_ratio, bloom_ratio, other_em, other_bloom_ratio
        })
    }
}