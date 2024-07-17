use std::collections::HashMap;

use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::buffs::{Buff, BuffConfig};
use mona::character::{Character, CharacterName};
use mona::character::characters::damage;
use mona::character::skill_config::CharacterSkillConfig;
use mona::character::traits::CharacterTrait;
use mona::common::Element;
use mona::damage::{ComplicatedDamageBuilder, DamageAnalysis, DamageContext, SimpleDamageBuilder};
use mona::damage::damage_builder::DamageBuilder;
use mona::damage::damage_result::SimpleDamageResult;
use mona::damage::transformative_damage::TransformativeDamage;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::team::TeamQuantization;
use mona::utils;
use mona::weapon::Weapon;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::applications::common::{BuffInterface, CharacterInterface, EnemyInterface, SkillInterface, TargetFunctionInterface, WeaponInterface};

pub struct CalculatorInterface;

#[derive(Serialize, Deserialize)]
pub struct CalculatorConfigInterface {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub buffs: Vec<BuffInterface>,
    pub artifacts: Vec<Artifact>,
    pub artifact_config: Option<ArtifactEffectConfig>,
    pub skill: SkillInterface,
    pub enemy: Option<EnemyInterface>,
}

// #[derive(Serialize, Deserialize)]
// pub struct DamageWithoutAttributeInterface {
//     pub atk: f64,
//     pub atk_ratio: f64,
//     pub def: f64,
//     pub def_ratio: f64,
//     pub hp: f64,
//     pub hp_ratio: f64,
//     pub extra_damage: f64,
//     pub bonus: f64,
//     pub critical: f64,
//     pub critical_damage: f64,
//     pub melt_enhance: f64,
//     pub vaporize_enhance: f64,
//
//     pub def_minus: f64,
//     pub def_penetration: f64,
//     pub res_minus: f64,
// }

// #[wasm_bindgen]
// pub struct TransformativeDamageOutput {
//     pub overload: f64,
//     pub electro_charged: f64,
//     pub swirl_pyro: f64,
//     pub swirl_electro: f64,
//     pub swirl_cryo: f64,
//     pub swirl_hydro: f64
// }

#[wasm_bindgen]
impl CalculatorInterface {
    pub fn get_damage_analysis(value: JsValue, fumo: JsValue) -> JsValue {
        utils::set_panic_hook();
        // utils::log!("start");

        let input: CalculatorConfigInterface = serde_wasm_bindgen::from_value(value).unwrap();
        let fumo: Option<Element> = serde_wasm_bindgen::from_value(fumo).unwrap();

        let character: Character<ComplicatedAttributeGraph> = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);

        let buffs: Vec<Box<dyn Buff<ComplicatedAttributeGraph>>> = input.buffs.iter().map(|x| x.to_buff()).collect();
        let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();

        // utils::log!("{:?}", default_artifact_config);
        let artifact_config = match input.artifact_config {
            Some(x) => x,
            None => Default::default()
        };

        let enemy = if let Some(x) = input.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let result = CalculatorInterface::get_damage_analysis_internal(
            &character,
            &weapon,
            &buffs,
            artifacts,
            &artifact_config,
            input.skill.index,
            &input.skill.config,
            &enemy,
            fumo,
        );

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()
    }

    pub fn get_transformative_damage(value: JsValue) -> TransformativeDamage {
        utils::set_panic_hook();

        let input: CalculatorConfigInterface = serde_wasm_bindgen::from_value(value).unwrap();

        let character: Character<SimpleAttributeGraph2> = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);

        let buffs: Vec<_> = input.buffs.iter().map(|x| x.to_buff()).collect();
        let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();

        let artifact_config = match input.artifact_config {
            Some(x) => x,
            None => Default::default()
        };

        let enemy = if let Some(x) = input.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let attribute = AttributeUtils::create_attribute_from_big_config(
            &ArtifactList {
                artifacts: &artifacts
            },
            &artifact_config,
            &character,
            &weapon,
            &buffs
        );

        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            enemy: &enemy,
            attribute: &attribute
        };

        let result = context.transformative();

        result
    }

    // pub fn damage_without_attribute(value: &JsValue) -> SimpleDamageResult {
    //     let input: DamageWithoutAttributeInterface = value.into_serde().unwrap();
    //
    //     let mut builder = SimpleDamageBuilder::new(input.atk_ratio, input.def_ratio, input.hp_ratio);
    //     builder.add_extra_atk("", input.atk);
    //     builder.add_extra_def("", input.def);
    //     builder.add_extra_hp("", input.hp);
    //     builder.add_extra_damage("", input.extra_damage);
    //     builder.add_extra_bonus("", input.bonus);
    //     builder.add_extra_critical("", input.critical);
    //     builder.add_extra_critical_damage("", input.critical_damage);
    //     builder.add_extra_enhance_melt("", input.melt_enhance);
    //     builder.add_extra_enhance_vaporize("", input.vaporize_enhance);
    //     builder.add_extra_def_minus("", input.def_minus);
    //     builder.add_extra_def_penetration("", input.def_penetration);
    //     builder.add_extra_res_minus("", input.res_minus);
    //
    //
    // }
}

impl CalculatorInterface {
    pub fn get_damage_analysis_internal(
        character: &Character<ComplicatedAttributeGraph>,
        weapon: &Weapon<ComplicatedAttributeGraph>,
        buffs: &Vec<Box<dyn Buff<ComplicatedAttributeGraph>>>,
        artifacts: Vec<&Artifact>,
        artifact_config: &ArtifactEffectConfig,
        skill_index: usize,
        skill_config: &CharacterSkillConfig,
        enemy: &Enemy,
        fumo: Option<Element>,
    ) -> DamageAnalysis {
        // let mut ans: HashMap<String, DamageAnalysis> = HashMap::new();

        let artifact_list = ArtifactList {
            artifacts: &artifacts,
        };

        let attribute = AttributeUtils::create_attribute_from_big_config(
            &artifact_list,
            artifact_config,
            character,
            weapon,
            buffs
        );

        let context = DamageContext {
            character_common_data: &character.common_data,
            attribute: &attribute,
            enemy: &enemy
        };

        let damage: DamageAnalysis = damage::<ComplicatedDamageBuilder>(&context, skill_index, skill_config, fumo);
        damage
    }
}