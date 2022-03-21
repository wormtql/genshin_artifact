use std::time;
use mona::applications::bonus_per_stat::bonus_per_stat::{bonus_per_stat, BonusPerStatInput};

use mona::attribute::{AttributeName, AttributeUtils, ComplicatedAttributeGraph};
use mona::common::{ChangeAttribute, StatName, Element, SkillType};
use mona::weapon::{WeaponName, WeaponConfig, Weapon};
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::character::character_config::CharacterConfig;
use mona::target_functions::{TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils};
use mona::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::applications::calculator::interface_calculator::CalculatorInterface;
use mona::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode};
use mona::applications::optimize_artifacts::single_optimize::{optimize_single, optimize_single_interface};
use mona::character::{Character, CharacterName};
use mona::character::skill_config::CharacterSkillConfig;
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona::potential_function::potential_functions::artifact_eff::PotentialFunctionArtifactEffConfig;
use mona::potential_function::potential_functions::PotentialFunctionArtifactEff;
use mona::potential_function::potential_function::{PotentialFunctionMeta, PotentialFunction, calc_potential};
use mona::potential_function::potential_function_config::PotentialFunctionConfig;
use mona::potential_function::potential_function_engine::{ExpectationPotentialFunctionEngine, NaivePotentialFunctionEngine, PotentialFunctionEngine};
use mona::target_functions::target_functions::get_target_function;


fn test_bonus_per_stat() {
    let character = Character::new(CharacterName::Amber, 90, false, 0, 7, 7, 7, &CharacterConfig::NoConfig);
    let weapon = Weapon::new(WeaponName::PolarStar, 90, false, 1, &WeaponConfig::PolarStar { stack: 1 }, &character);
    let tf = get_target_function(
        TargetFunctionName::AmberDefault,
        &character, &weapon, &TargetFunctionConfig::NoConfig
    );

    let output = bonus_per_stat(BonusPerStatInput {
        character: &character,
        weapon: &weapon,
        artifacts: &[],
        enemy: &Default::default(),
        tf: &tf,
        buffs: &[],
        artifacts_config: None
    });

    println!("{:?}", output);
}

fn perf() {
    let character = CharacterInterface {
        name: CharacterName::Bennett,
        level: 90,
        ascend: false,
        constellation: 6,
        params: CharacterConfig::Ganyu { talent2_rate: 0.5, talent1_rate: 0.5 },
        skill1: 6,
        skill2: 6,
        skill3: 6,
    };
    let weapon = WeaponInterface {
        name: WeaponName::FesteringDesire,
        level: 90,
        ascend: false,
        refine: 1,
        params: WeaponConfig::NoConfig,
        // params: WeaponConfig::AmosBow { stack: 1.0 }
    };
    let target_function = TargetFunctionInterface {
        name: TargetFunctionName::BennettDamage,
        // params: TargetFunctionConfig::NoConfig,
        params: TargetFunctionConfig::BennettDamage { recharge_demand: 1.0, other_dmg_ratio: 1.0 }
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

    let constraint = ConstraintConfig {
        set_mode: Some(ConstraintSetMode::Set4(ArtifactSetName::BlizzardStrayer)),
        hp_min: None,
        atk_min: None,
        def_min: None,
        recharge_min: None,
        em_min: None,
        crit_min: None,
        crit_dmg_min: None
    };

    let results = optimize_single_interface(
        &artifacts_ref,
        None,
        &character,
        &weapon,
        &target_function,
        None,
        // Some(&constraint),
        &Vec::new(),
        10
    );
    println!("{:?}", results);

    // for art in artifacts.iter() {
    //     if art.id == results[0].flower.unwrap() {
    //         println!("{:?}", art);
    //     }
    //     if art.id == results[0].feather.unwrap() {
    //         println!("{:?}", art);
    //     }
    //     if art.id == results[0].sand.unwrap() {
    //         println!("{:?}", art);
    //     }
    //     if art.id == results[0].goblet.unwrap() {
    //         println!("{:?}", art);
    //     }
    //     if art.id == results[0].head.unwrap() {
    //         println!("{:?}", art);
    //     }
    // }

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
        &CharacterSkillConfig::NoConfig,
        &Default::default()
    );

    println!("{:?}", result);
}

fn test_potential() {
    let artifact = Artifact {
        set_name: ArtifactSetName::Adventurer,
        slot: ArtifactSlotName::Sand,
        level: 0,
        star: 5,
        sub_stats: vec![
            (StatName::ATKPercentage, 0.053),
            (StatName::CriticalDamage, 0.054),
            (StatName::DEFFixed, 10.0),
            (StatName::CriticalRate, 0.039)
        ],
        main_stat: (StatName::Recharge, 0.0),
        id: 0
    };

    let potential_config = PotentialFunctionArtifactEffConfig {
        atk_use: false,
        atk_weight: 1.,

        atk_p_use: false,
        atk_p_weight: 1.,

        hp_use: false,
        hp_weight: 1.,

        hp_p_use: false,
        hp_p_weight: 1.,

        def_use: false,
        def_weight: 1.,

        def_p_use: false,
        def_p_weight: 1.,

        critical_use: true,
        critical_weight: 1.,

        critical_damage_use: true,
        critical_damage_weight: 1.,

        elemental_mastery_use: false,
        elemental_mastery_weight: 1.,

        recharge_use: false,
        recharge_weight: 1.
    };

    let pf = PotentialFunctionArtifactEff::create(&PotentialFunctionConfig::ArtifactEff(potential_config));

    // let engine = NaivePotentialFunctionEngine;
    // let engine = ExpectationPotentialFunctionEngine;

    // let score = engine.value(&pf, &artifact);

    // let score = pf.potential(&artifact);

    let score = calc_potential(&pf, &artifact);
    println!("{}", score);
}

fn main() {
    // perf::<ComplicatedAttributeGraph>();
    // perf();
    // test_bonus_per_stat();
    test_potential();

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
