use std::collections::HashSet;
use mona::artifacts::{Artifact, ArtifactList, ArtifactSetName, ArtifactSlotName};
use mona::artifacts::artifact_set_type::ArtifactSetType;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{AttributeUtils, SimpleAttributeGraph2};
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use strum::EnumCount;

pub struct ValueFunction<'a> {
    pub character: &'a Character<SimpleAttributeGraph2>,
    pub weapon: &'a Weapon<SimpleAttributeGraph2>,
    pub tf: &'a Box<dyn TargetFunction>,
    pub artifact_config: &'a ArtifactEffectConfig,
    pub buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
    pub enemy: &'a Enemy,
}

impl<'a> ValueFunction<'a> {
    pub fn call(&self, artifacts: &[&Artifact]) -> f64 {
        let artifact_list = ArtifactList {
            artifacts,
        };
        let attribute = AttributeUtils::create_attribute_from_big_config(
            &artifact_list,
            &self.artifact_config,
            &self.character,
            &self.weapon,
            &self.buffs
        );

        self.tf.target(
            &attribute,
            &self.character,
            &self.weapon,
            artifacts,
            &self.enemy,
        )
    }

    pub fn call_with_stat(&self, stat_name: StatName, value: f64) -> f64 {
        let artifact = Artifact::new(
            ArtifactSetName::Empty,
            ArtifactSlotName::Flower,
            0,
            5,
            vec![],
            (stat_name, value)
        );

        self.call(&[&artifact])
    }

    pub fn call_with_set_type(&self, set_type: ArtifactSetType) -> f64 {
        let create_empty_artifact = |set_name: ArtifactSetName| {
            Artifact::new(
                set_name,
                ArtifactSlotName::Flower,
                0,
                5,
                vec![],
                (StatName::ATKFixed, 0.0)
            )
        };

        let set_names = match set_type {
            ArtifactSetType::Set4(s) => {
                vec![s, s, s, s, ArtifactSetName::Empty]
            },
            ArtifactSetType::Set22(s1, s2) => {
                vec![s1, s1, s2, s2, ArtifactSetName::Empty]
            },
            ArtifactSetType::Set2(s) => {
                vec![s, s, ArtifactSetName::Empty, ArtifactSetName::Empty, ArtifactSetName::Empty]
            },
            ArtifactSetType::Misc => {
                vec![ArtifactSetName::Empty; 5]
            }
        };

        let artifacts = set_names.iter().map(|x| create_empty_artifact(*x)).collect::<Vec<_>>();
        let a2 = artifacts.iter().map(|x| x).collect::<Vec<_>>();
        self.call(&a2)
    }

    pub fn get_effective_stat_name(&self) -> HashSet<StatName> {
        let mut result = HashSet::new();
        let base_value = self.call(&[]);

        for i in 0..StatName::COUNT {
            let stat_name: StatName = num::FromPrimitive::from_usize(i).unwrap();
            let value = self.call_with_stat(stat_name, 10000.0);
            if value > base_value {
                result.insert(stat_name);
            }
        }

        result
    }

    pub fn get_effective_artifact_set_type(&self) -> HashSet<ArtifactSetType> {
        let mut result = HashSet::new();
        let base_value = self.call(&[]);

        let len: usize = ArtifactSetName::LEN;

        for i in 0..len {
            let set_name: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();
            let t = ArtifactSetType::Set4(set_name);
            let value = self.call_with_set_type(t);
            if value > base_value {
                result.insert(t);
            }

            let t2 = ArtifactSetType::Set2(set_name);
            let value = self.call_with_set_type(t2);
            if value > base_value {
                result.insert(t2);
            }
        }

        result
    }
}