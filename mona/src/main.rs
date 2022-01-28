use std::time;

use mona::attribute::{AttributeName, AttributeUtils, ComplicatedAttributeGraph};
use mona::common::{ChangeAttribute, StatName, Element, SkillType};
use mona::weapon::{WeaponName, WeaponConfig, Weapon};
use mona::artifacts::{Artifact, ArtifactSlotName};
use mona::character::character_config::CharacterConfig;
use mona::target_functions::{TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils};
use mona::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::applications::calculator::interface_calculator::CalculatorInterface;
use mona::applications::optimize_artifacts::single_optimize::{optimize_single, optimize_single_interface};
use mona::character::{Character, CharacterName};
use mona::character::skill_config::CharacterSkillConfig;
use mona::damage::DamageContext;
use mona::enemies::Enemy;


fn perf() {
    let character = CharacterInterface {
        name: CharacterName::Diluc,
        level: 90,
        ascend: false,
        constellation: 0,
        params: CharacterConfig::Ganyu { talent2_rate: 0.5, talent1_rate: 0.5 },
        skill1: 6,
        skill2: 6,
        skill3: 6,
    };
    let weapon = WeaponInterface {
        name: WeaponName::AmosBow,
        level: 90,
        ascend: false,
        refine: 1,
        params: WeaponConfig::AmosBow { stack: 1.0 }
    };
    let target_function = TargetFunctionInterface {
        name: TargetFunctionName::KamisatoAyakaDefault,
        params: TargetFunctionConfig::GanyuDefault { melt_rate: 0.5 }
        // name: TargetFunctionName::Max,
        // params: TargetFunctionConfig::MaxConfig { element: Element::Cryo, skill_type: SkillType::NormalAttack },
    };
    let mut artifacts = vec![];
    for _ in 0..25 {
        artifacts.push(Artifact::new_random(ArtifactSlotName::Flower));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Feather));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Sand));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Goblet));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Head));
    }
    let artifacts_ref: Vec<&Artifact> = artifacts.iter().collect();

    let now = time::SystemTime::now();

    let results = optimize_single_interface(
        &artifacts_ref,
        None,
        &character,
        &weapon,
        &target_function,
        None,
        &Vec::new(),
        5
    );
    println!("{:?}", results);

    println!("{}s", now.elapsed().unwrap().as_secs())
}

// fn skill() {
//     let character: Character<ComplicatedAttributeGraph> = Character::new(
//         CharacterName::HuTao,
//         90, false, 0,
//         8, 8, 8,
//         &CharacterConfig::HuTao { le_50: true },
//     );
//     let weapon: Weapon<ComplicatedAttributeGraph> = Weapon::new(
//         WeaponName::StaffOfHoma,
//         90,
//         false,
//         1,
//         &WeaponConfig::NoConfig,
//         &character
//     );
//     let attribute = AttributeUtils::create_attribute_from_c_w_bs(
//         &character,
//         &weapon,
//         &Vec::new()
//     );
//     let enemy = Enemy::default();
//     let context = DamageContext {
//         enemy: &enemy,
//         character_common_data: &character.common_data,
//         attribute: &attribute
//     };
//     let result = HuTaoDamage::damage(&context, HuTaoSkillEnum::Charged, true);
//     println!("{:?}", result);
// }

fn skill_interface() {
    let character: Character<ComplicatedAttributeGraph> = Character::new(
        CharacterName::HuTao,
        90, false, 0,
        8, 8, 8,
        &CharacterConfig::HuTao { le_50: true },
    );
    let weapon: Weapon<ComplicatedAttributeGraph> = Weapon::new(
        WeaponName::StaffOfHoma,
        90,
        false,
        1,
        &WeaponConfig::NoConfig,
        &character
    );
    // let tf = TargetFunctionUtils::new_target_function(
    //     TargetFunctionName::HuTaoDefault,
    //     &character,
    //     &weapon,
    //     &TargetFunctionConfig::HuTaoDefault { vaporize_rate: 0.5 }
    // );
    // let artifact_config = tf.get_default_artifact_config(&Default::default());

    let result = CalculatorInterface::get_damage_analysis_internal(
        &character,
        &weapon,
        &Vec::new(),
        Vec::new(),
        &Default::default(),
        0,
        &CharacterSkillConfig::NoConfig
    );

    println!("{:?}", result);
}

fn main() {
    // perf::<ComplicatedAttributeGraph>();
    perf();

    // skill();
    // skill_interface();
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
