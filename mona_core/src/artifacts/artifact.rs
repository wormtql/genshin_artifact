use std::collections::HashMap;

use mona_derive::{ArtifactData, EnumLen};
use num_derive::FromPrimitive;
use rand::{Rng, thread_rng};
use rand::distributions::{WeightedIndex};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use crate::artifacts::eff::ARTIFACT_EFF5;

use crate::artifacts::effect::ArtifactEffect;
use crate::attribute::Attribute;
use crate::character::Character;
use crate::common::StatName;

use super::effect_config::ArtifactEffectConfig;
use super::effects::get_effect;

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[derive(FromPrimitive, ArtifactData, EnumLen, Display, EnumIter)]
pub enum ArtifactSetName {
    Empty,  // use to construct fake super artifact
    Adventurer,
    ArchaicPetra,
    Berserker,
    BlizzardStrayer,
    BloodstainedChivalry,
    BraveHeart,
    CrimsonWitchOfFlames,
    DeepwoodMemories,
    DefendersWill,
    EchoesOfAnOffering,
    EmblemOfSeveredFate,
    FlowerOfParadiseLost,
    Gambler,
    DesertPavilionChronicle,
    GildedDreams,
    GladiatorsFinale,
    GoldenTroupe,
    HeartOfDepth,
    HuskOfOpulentDreams,
    Instructor,
    Lavawalker,
    LuckyDog,
    MaidenBeloved,
    MarechausseeHunter,
    MartialArtist,
    NoblesseOblige,
    NymphsDream,
    OceanHuedClam,
    PaleFlame,
    PrayersForDestiny,
    PrayersForIllumination,
    PrayersForWisdom,
    PrayersToSpringtime,
    ResolutionOfSojourner,
    RetracingBolide,
    Scholar,
    ShimenawasReminiscence,
    TenacityOfTheMillelith,
    TheExile,
    ThunderingFury,
    Thundersoother,
    TinyMiracle,
    TravelingDoctor,
    VermillionHereafter,
    ViridescentVenerer,
    VourukashasGlow,
    WanderersTroupe,
    SongOfDaysPast,
    NighttimeWhispersInTheEchoingWoods,
    FragmentOfHarmonicWhimsy,
    UnfinishedReverie,
}

impl ArtifactSetName {
    pub fn random() -> ArtifactSetName {
        let max = ArtifactSetName::WanderersTroupe as usize;
        let mut rng = thread_rng();
        let n: usize = rng.gen::<usize>() % max;
        num::FromPrimitive::from_usize(n).unwrap()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(FromPrimitive, EnumLen, EnumIter)]
pub enum ArtifactSlotName {
    Flower,
    Feather,
    Sand,
    Goblet,
    Head,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artifact {
    pub set_name: ArtifactSetName,
    pub slot: ArtifactSlotName,
    pub level: i32,
    pub star: i32,
    pub sub_stats: Vec<(StatName, f64)>,
    pub main_stat: (StatName, f64),
    pub id: u64
}

impl Artifact {
    pub fn new(
        set_name: ArtifactSetName,
        slot: ArtifactSlotName,
        level: i32,
        star: i32,
        sub_stats: Vec<(StatName, f64)>,
        main_stat: (StatName, f64)
    ) -> Artifact {
        Artifact {
            set_name,
            slot,
            level,
            star,
            main_stat,
            sub_stats,
            id: 0
        }
    }

    pub fn new_random(slot: ArtifactSlotName) -> Artifact {
        let main_stat = StatName::random_artifact_main_stat(slot);
        let main_stat_value = StatName::artifact_main_stat_max_value(main_stat);
        // todo currently it's not random
        Artifact {
            set_name: ArtifactSetName::random(),
            // set_name: ArtifactSetName::NoblesseOblige,
            slot,
            level: 20,
            star: 5,
            sub_stats: vec![
                (StatName::ATKPercentage, 0.1),
                (StatName::ATKFixed, 10.0),
                (StatName::Recharge, 0.1),
                (StatName::ElementalMastery, 10.0),
            ],
            main_stat: (main_stat, main_stat_value),
            id: thread_rng().gen()
        }
    }

    pub fn is_max_level(&self) -> bool {
        (self.star == 5 && self.level == 20)
        || (self.star == 4 && self.level == 16)
        || (self.star <= 3 && self.level == 12)
    }

    pub fn random0(set_name: ArtifactSetName, slot: ArtifactSlotName, main_stat: StatName) -> Artifact {
        let mut temp = Artifact {
            set_name,
            slot,
            level: 0,
            star: 5,
            sub_stats: vec![],
            main_stat: (main_stat, StatName::artifact_main_stat_max_value(main_stat)),
            id: thread_rng().gen()
        };

        let flag = thread_rng().gen::<usize>() % 3;
        let count = if flag == 0 { 4 } else { 3 };

        for _ in 0..count {
            temp.upgrade();
        }
        temp.level = 0;

        temp
    }

    pub fn upgrade(&mut self) {
        if self.is_max_level() {
            return;
        }

        self.level = (self.level / 4 + 1) * 4;

        if self.sub_stats.len() == 4 {
            let index = thread_rng().gen::<usize>() % 4;
            let stat_name = self.sub_stats[index].0;
            let add_value = ARTIFACT_EFF5.get_value(stat_name, thread_rng().gen::<usize>() % 4);
            self.sub_stats[index].1 += add_value;
        } else {
            let next_stat_name_dist = self.get_next_stat_name_dist().unwrap();
            let w = WeightedIndex::new(next_stat_name_dist.iter().map(|x| x.1)).unwrap();
            let index = w.sample(&mut thread_rng());
            let selected_main_stat = next_stat_name_dist[index].0;

            // random value;
            let value_index = thread_rng().gen::<usize>() % 4;
            let value = ARTIFACT_EFF5.get_value(selected_main_stat, value_index);
            self.sub_stats.push((selected_main_stat, value));
        }
    }

    // return statname, and the probability of that stat
    pub fn get_next_stat_name_dist(&self) -> Option<Vec<(StatName, f64)>> {
        if self.sub_stats.len() == 4 {
            None
        } else {
            use StatName::*;
            let mut weights = vec![150.0, 100.0, 150.0, 100.0, 150.0, 100.0, 75.0, 75.0, 100.0, 100.0];
            let mut stat_names = vec![HPFixed, HPPercentage, ATKFixed, ATKPercentage, DEFFixed, DEFPercentage, CriticalRate, CriticalDamage, ElementalMastery, Recharge];

            let mut existing_stat_names = Vec::new();
            existing_stat_names.push(self.main_stat.0);
            for stat in self.sub_stats.iter() {
                existing_stat_names.push(stat.0);
            }

            for stat_name in existing_stat_names {
                let maybe_index = stat_names.iter().position(|x| *x == stat_name);
                if let Some(index) = maybe_index {
                    stat_names.remove(index);
                    weights.remove(index);
                }
            }

            let weight_sum: f64 = weights.iter().sum();

            let mut result = Vec::new();
            for i in 0..weights.len() {
                let prob = weights[i] / weight_sum;
                let stat_name = stat_names[i];
                result.push((stat_name, prob));
            }

            Some(result)
        }
    }
}

pub struct ArtifactList<'a> {
    pub artifacts: &'a [&'a Artifact]
    // pub artifacts: &'a SmallVec<[&'b Artifact; 5]>
}

impl<'a> ArtifactList<'a> {
    pub fn apply<T: Attribute>(&self, attribute: &mut T, character: &Character<T>, config: &ArtifactEffectConfig) {
        let mut attributes_hash: HashMap<StatName, f64> = HashMap::new();
        let mut set_name_count: HashMap<ArtifactSetName, i32> = HashMap::new();

        for &artifact in self.artifacts.iter() {
            *set_name_count.entry(artifact.set_name).or_insert(0) += 1;

            *attributes_hash.entry(artifact.main_stat.0).or_insert(0.0) += artifact.main_stat.1;
            for sub_stat in artifact.sub_stats.iter() {
                *attributes_hash.entry(sub_stat.0).or_insert(0.0) += sub_stat.1;
            }
        }

        // calculate stats
        let key = "圣遗物词条";
        for stats in attributes_hash.iter() {
            stats.0.apply(attribute, key, *stats.1);
        }

        // calculate set effect
        for (&set_name, &count) in set_name_count.iter() {
            let effect = get_effect(set_name, config, character);
            if count >= 5 {
                effect.effect5(attribute);
            }
            if count >= 4 {
                effect.effect4(attribute);
            }
            if count >= 3 {
                effect.effect3(attribute);
            }
            if count >= 2 {
                effect.effect2(attribute);
            }
            if count >= 1 {
                effect.effect1(attribute);
            }
        }
    }
}
