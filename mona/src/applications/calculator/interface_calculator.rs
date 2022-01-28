use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::applications::common::{CharacterInterface, SkillInterface, TargetFunctionInterface, WeaponInterface};
use crate::artifacts::{Artifact, ArtifactList};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use crate::buffs::{Buff, BuffType};
use crate::character::{Character, CharacterName};
use crate::character::characters::damage;
use crate::character::traits::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::{ComplicatedDamageBuilder, DamageAnalysis, DamageContext, SimpleDamageBuilder};
use crate::damage::damage_result::SimpleDamageResult;
use crate::enemies::Enemy;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::utils;

pub struct CalculatorInterface {}

#[derive(Serialize, Deserialize)]
pub struct CalculatorConfigInterface {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub buffs: Vec<BuffType>,
    pub artifacts: Vec<Artifact>,
    pub artifact_config: Option<ArtifactEffectConfig>,
    pub skill: SkillInterface
}

#[wasm_bindgen]
impl CalculatorInterface {
    pub fn get_damage_analysis(value: &JsValue) -> JsValue {
        utils::set_panic_hook();
        utils::log!("start");

        let input: CalculatorConfigInterface = value.into_serde().unwrap();

        let character: Character<ComplicatedAttributeGraph> = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);

        let buffs: Vec<Box<dyn Buff<ComplicatedAttributeGraph>>> = input.buffs.iter().map(|x| x.into()).collect();
        let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();

        // utils::log!("{:?}", default_artifact_config);
        let artifact_config = match input.artifact_config {
            Some(x) => x,
            None => Default::default()
        };

        let result = CalculatorInterface::get_damage_analysis_internal(
            &character,
            &weapon,
            &buffs,
            artifacts,
            &artifact_config,
            input.skill.index,
            &input.skill.config
        );

        JsValue::from_serde(&result).unwrap()
    }
}

impl CalculatorInterface {
    pub fn get_damage_analysis_internal(
        character: &Character<ComplicatedAttributeGraph>,
        weapon: &Weapon<ComplicatedAttributeGraph>,
        buffs: &Vec<Box<dyn Buff<ComplicatedAttributeGraph>>>,
        artifacts: Vec<&Artifact>,
        artifact_config: &ArtifactEffectConfig,
        skill_index: usize,
        skill_config: &CharacterSkillConfig
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

        let enemy = Enemy::default();
        let context = DamageContext {
            character_common_data: &character.common_data,
            attribute: &attribute,
            enemy: &enemy
        };

        let damage: DamageAnalysis = damage::<ComplicatedDamageBuilder>(&context, skill_index, skill_config);
        damage

        // match character.common_data.name {
        //
        //     CharacterName::HuTao => {
        //         ans.insert(String::from("普攻1段"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::Normal1, false));
        //         ans.insert(String::from("普攻1段-彼岸蝶舞"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::Normal1, true));
        //         ans.insert(String::from("重击"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::Charged, false));
        //         ans.insert(String::from("重击-彼岸蝶舞"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::Charged, true));
        //         ans.insert(String::from("血梅香"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::ElementalSkillBloodBlossom, false));
        //         ans.insert(String::from("血梅香-彼岸蝶舞"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::ElementalSkillBloodBlossom, true));
        //         ans.insert(String::from("大招伤害"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::ElementalBurst1, false));
        //         ans.insert(String::from("大招伤害-彼岸蝶舞"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::ElementalBurst1, true));
        //         ans.insert(String::from("低血量大招伤害"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::ElementalBurstLow1, false));
        //         ans.insert(String::from("低血量大招伤害-彼岸蝶舞"), HuTaoDamage::damage::<ComplicatedDamageBuilder>(&context, HuTaoDamageEnum::ElementalBurstLow1, true));
        //     },
        //     // CharacterName::Albedo => {
        //     //     ans.insert(String::from("普攻1段"), AlbedoDamage::damage::<ComplicatedDamageBuilder>(&context, AlbedoDamageEnum::Normal1, 0.0));
        //     //     ans.insert(String::from("阳华伤害"), AlbedoDamage::damage::<ComplicatedDamageBuilder>(&context, AlbedoDamageEnum::E1, 0.0));
        //     //     ans.insert(String::from("刹那之花"), AlbedoDamage::damage::<ComplicatedDamageBuilder>(&context, AlbedoDamageEnum::ETransientBlossom, 0.0));
        //     //     ans.insert(String::from("元素爆发伤害"), AlbedoDamage::damage::<ComplicatedDamageBuilder>(&context, AlbedoDamageEnum::Q1, 4.0));
        //     //     ans.insert(String::from("生灭之花"), AlbedoDamage::damage::<ComplicatedDamageBuilder>(&context, AlbedoDamageEnum::QFatalBlossom, 4.0));
        //     // },
        //     // CharacterName::Amber => {
        //     //     ans.insert(String::from("普攻1段"), AmberDamage::damage::<ComplicatedDamageBuilder>(&context, AmberDamageE))
        //     // }
        //     _ => ()
        // }
        //
        // ans
    }
}