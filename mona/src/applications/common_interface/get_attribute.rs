use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::artifacts::{Artifact, ArtifactList};
use crate::attribute::{AttributeNoReactive, AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use crate::buffs::{Buff, BuffType};
use crate::character::Character;
use crate::weapon::Weapon;

#[derive(Serialize, Deserialize)]
pub struct GetAttributeInterface {
    character: CharacterInterface,
    weapon: WeaponInterface,
    target_function: TargetFunctionInterface,
    buffs: Vec<BuffType>,
    artifacts: Vec<Artifact>
}

pub fn get_attribute(val: &JsValue) -> JsValue {
    let input: GetAttributeInterface = val.into_serde().unwrap();

    let character: Character<ComplicatedAttributeGraph> = input.character.to_character();
    let weapon: Weapon<ComplicatedAttributeGraph> = input.weapon.to_weapon(&character);

    let character_simple: Character<SimpleAttributeGraph2> = input.character.to_character();
    let weapon_simple: Weapon<SimpleAttributeGraph2> = input.weapon.to_weapon(&character_simple);
    let target_function = input.target_function.to_target_function(&character_simple, &weapon_simple);

    let default_artifact_config = target_function.get_default_artifact_config(
        &Default::default()
    );

    let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();
    let artifact_list = ArtifactList {
        artifacts
    };

    let buffs: Vec<Box<dyn Buff<ComplicatedAttributeGraph>>> = input.buffs.iter().map(|x| x.into()).collect();

    let attribute: ComplicatedAttributeGraph = AttributeUtils::create_attribute_from_big_config(
        &artifact_list,
        &default_artifact_config,
        &character,
        &weapon,
        &buffs
    );

    let result = AttributeNoReactive::from(&attribute);
    JsValue::from_serde(&result).unwrap()
}
