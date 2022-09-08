use std::collections::HashMap;
use mona::artifacts::{Artifact, ArtifactList, ArtifactSetName, ArtifactSlotName};
use mona::artifacts::eff::{ARTIFACT_EFF5, ArtifactEff};
use mona::attribute::{AttributeUtils, SimpleAttributeGraph2};
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;

pub trait WeightHeuristicAlgorithm {
    fn generate_stat(&self, target_function: &Box<dyn TargetFunction>) -> HashMap<StatName, f64>;

    fn generate_set(&self, target_function: &Box<dyn TargetFunction>) -> HashMap<ArtifactSetName, f64>;
}

// pub struct PSOWeightHeuristicHelper {
//     pub particle_count: usize,
//
//     // there are 19 stats
//     pub particles: Vec<[f64; 19]>,
//     pub personal_best: Vec<[f64; 19]>,
//     pub global_best: [f64; 19],
// }
//
// impl PSOWeightHeuristicHelper {
//     pub fn optimize(&mut self) {
//         for i in
//     }
// }
//
// impl WeightHeuristicAlgorithm for PSOWeightHeuristic {
//     fn generate_stat(&self, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, target_function: &Box<dyn TargetFunction>, enemy: &Enemy) -> HashMap<StatName, f64> {
//
//     }
// }

pub struct NaiveWeightHeuristic<'a> {
    pub character: &'a Character<SimpleAttributeGraph2>,
    pub weapon: &'a Weapon<SimpleAttributeGraph2>,
}

impl<'a> WeightHeuristicAlgorithm for NaiveWeightHeuristic<'a> {
    fn generate_stat(&self, target_function: &Box<dyn TargetFunction>) -> HashMap<StatName, f64> {
        use StatName::*;
        let critical_stats = [
            ATKPercentage,
            HealingBonus,
            HPPercentage,
            DEFPercentage,
            CriticalRate,
            CriticalDamage,
            ElementalMastery,
            Recharge,
            ElectroBonus,
            PyroBonus,
            HydroBonus,
            CryoBonus,
            AnemoBonus,
            GeoBonus,
            DendroBonus,
            PhysicalBonus,
        ];

        let base_value = {
            let base_attribute = AttributeUtils::create_attribute_from_c_w_bs(&self.character, &self.weapon, &Vec::new());
            let target_value = target_function.target(&base_attribute, &self.character, &self.weapon, &[], &Default::default());
            target_value
        };

        if base_value == 0.0 {
            return HashMap::new();
        }

        let mut result = HashMap::new();
        for &stat in critical_stats.iter() {
            let virtual_artifact = Artifact::new(
                ArtifactSetName::Empty, ArtifactSlotName::Flower, 20, 5,
                vec![(stat, ARTIFACT_EFF5.get_value(stat, 3) * 10.0)],
                (StatName::HPFixed, 0.0)
            );

            let arts = vec![&virtual_artifact];
            let art_list = ArtifactList {
                artifacts: &arts
            };

            let attribute = AttributeUtils::create_attribute_from_big_config(
                &art_list,
                &Default::default(),
                &self.character,
                &self.weapon,
                &[]
            );

            let value = target_function.target(&attribute, &self.character, &self.weapon, &[], &Default::default());
            let ratio = (value - base_value) / base_value;
            if ratio > 0.0 {
                result.insert(stat, 1.0);
            }
            // result.insert(stat, ratio);
        }

        result
    }

    fn generate_set(&self, target_function: &Box<dyn TargetFunction>) -> HashMap<ArtifactSetName, f64> {
        HashMap::from([
            (ArtifactSetName::EmblemOfSeveredFate, 1.0),
            (ArtifactSetName::BlizzardStrayer, 1.0),
        ])
    }
}