use std::time;

use mona::attribute::entry::{Node, NodeHandle};
use mona::attribute::{AttributeName, AttributeGraph};
use mona::common::{ChangeAttribute, StatName, Element, SkillType};
use mona::weapon::{WeaponName, WeaponConfig, Weapon};
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName, ArtifactList};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::character::{Character, CharacterName};
use mona::character::character_config::CharacterConfig;
use mona::target_functions::{TargetFunctionConfig, TargetFunctionName};
use mona::target_functions::target_functions::{get_target_function};
use mona::applications::{CharacterInterface, OptimizeArtifactInterface};
use mona::WeaponInterface;
use mona::applications::optimize_artifacts::interface_config_object::TargetFunctionInterface;
use mona::target_functions::target_function_config::TargetFunctionConfig::ExpectationConfig;
use mona::attribute::attribute_utils::AttributeUtils;


fn perf() {
    let character = Character::new(
        CharacterName::Albedo,
        90, false, 0, &CharacterConfig::NoConfig,
    );
    let weapon = Weapon::new(
        WeaponName::MistsplitterReforged,
        90, false, 1, &WeaponConfig::MistsplitterReforgedConfig(1),
        &character
    );
    let artifacts = vec![
        Artifact::new_random(ArtifactSlotName::Flower),
        Artifact::new_random(ArtifactSlotName::Feather),
        Artifact::new_random(ArtifactSlotName::Feather),
        Artifact::new_random(ArtifactSlotName::Feather),
        Artifact::new_random(ArtifactSlotName::Feather),
    ];
    let artifact_list = ArtifactList {
        artifacts: artifacts.iter().collect(),
    };

    let now = time::SystemTime::now();
    for _ in 0..1000000 {
        let attribute = AttributeUtils::create_attribute_from_big_config(
            &artifact_list,
            &ArtifactEffectConfig::default(),
            &character,
            &weapon,
            &vec![],
        );
    }
    println!("{}s", now.elapsed().unwrap().as_secs())
}

fn main() {
    perf();
    // let mut graph = AttributeGraph::new();
    //
    // let mona = Character::new(
    //     CharacterName::Mona,
    //     90,
    //     false,
    //     0,
    //     &CharacterConfig::NoConfig
    // );
    // let sword = Weapon::new(
    //     WeaponName::MistsplitterReforged,
    //     90,
    //     false,
    //     1,
    //     &WeaponConfig::MistsplitterReforgedConfig(1),
    //     &mona,
    // );
    //
    // let set_name = ArtifactSetName::EmblemOfSeveredFate;
    //
    // let art1 = Artifact::new(
    //     set_name,
    //     ArtifactSlotName::Flower,
    //     20, 5,
    //     vec![(StatName::Recharge, 0.1)],
    //     (StatName::CriticalRate, 0.1)
    // );
    //
    // let art2 = Artifact::new(
    //     set_name,
    //     ArtifactSlotName::Flower,
    //     20, 5,
    //     vec![(StatName::Recharge, 0.1)],
    //     (StatName::CriticalRate, 0.1)
    // );
    //
    // let art3 = Artifact::new(
    //     set_name,
    //     ArtifactSlotName::Flower,
    //     20, 5,
    //     vec![(StatName::Recharge, 0.1)],
    //     (StatName::CriticalRate, 0.1)
    // );
    //
    // let art4 = Artifact::new(
    //     set_name,
    //     ArtifactSlotName::Flower,
    //     20, 5,
    //     vec![(StatName::Recharge, 0.1)],
    //     (StatName::CriticalRate, 0.1)
    // );
    //
    // let artifacts = ArtifactList {
    //     artifacts: vec![&art1, &art2, &art3, &art4]
    // };
    //
    // mona.change_attribute(&mut graph);
    // sword.change_attribute(&mut graph);
    // artifacts.apply(&mut graph, &mona, &ArtifactEffectConfig::default());
    // // println!("{}", graph.get_value(AttributeName::ATK));
    // println!("{:?}", graph.get_attribute_composition(AttributeName::Recharge));
    // println!("{:?}", graph.get_attribute_composition(AttributeName::BonusElementalBurst));
    //
    // let target_function = get_target_function(
    //     TargetFunctionName::Expectation,
    //     &TargetFunctionConfig::ExpectationConfig(Element::Cryo, SkillType::NormalAttack)
    // );
    // let value = target_function.target(&graph);
    // println!("value: {}", value);


    // let mut node1 = NodeHandle::new(1.0);
    // node1.set_value("123", 2.0);
    //
    // let mut node2 = NodeHandle::new(1.0);
    // node1.add_edge(
    //     &node2,
    //     |n| (String::from("test"), n.value() * 0.5)
    // );
    //
    // println!("{}, {}", node1.value(), node2.value());
    //
    // node1.set_value("123", 3.0);
    // println!("{}, {}", node1.value(), node2.value());
    //
    // let mut node3 = NodeHandle::new(0.0);
    // node2.add_edge(
    //     &node3,
    //     |n| (String::from("test"), n.value() * 0.5)
    // );
    // node1.add_edge(
    //     &node3,
    //     |n| (String::from("test2"), n.value() * 0.5)
    // );
    //
    // println!("{}", node3.value());
}
