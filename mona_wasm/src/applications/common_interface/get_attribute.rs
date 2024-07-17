use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::applications::common::{BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{AttributeNoReactive, AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::buffs::{Buff, BuffConfig};
use mona::character::Character;
use mona::weapon::Weapon;

#[derive(Serialize, Deserialize)]
pub struct GetAttributeInterface {
    character: CharacterInterface,
    weapon: WeaponInterface,
    buffs: Vec<BuffInterface>,
    artifacts: Vec<Artifact>,
    artifact_config: Option<ArtifactEffectConfig>
}

pub fn get_attribute(val: JsValue) -> JsValue {
    let input: GetAttributeInterface = serde_wasm_bindgen::from_value(val).unwrap();

    let character: Character<ComplicatedAttributeGraph> = input.character.to_character();
    let weapon: Weapon<ComplicatedAttributeGraph> = input.weapon.to_weapon(&character);

    let artifact_config = match input.artifact_config {
        Some(x) => x,
        None => Default::default()
    };

    let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();
    let artifact_list = ArtifactList {
        artifacts: &artifacts
    };

    let buffs: Vec<Box<dyn Buff<ComplicatedAttributeGraph>>> = input.buffs.iter().map(|x| x.to_buff()).collect();

    let attribute: ComplicatedAttributeGraph = AttributeUtils::create_attribute_from_big_config(
        &artifact_list,
        &artifact_config,
        &character,
        &weapon,
        &buffs
    );

    let result = AttributeNoReactive::from(&attribute);
    let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
    result.serialize(&s).unwrap()
}
