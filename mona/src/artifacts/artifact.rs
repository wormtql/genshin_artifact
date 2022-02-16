use std::collections::HashMap;

use rand::{thread_rng, Rng};
use num_derive::FromPrimitive;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use mona_derive::{ArtifactData, EnumLen};

use crate::common::StatName;
use crate::attribute::{Attribute};
use super::effect_config::ArtifactEffectConfig;
use crate::character::{Character};
use super::effects::get_effect;
use crate::artifacts::effect::ArtifactEffect;

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[derive(FromPrimitive, ArtifactData, EnumLen, Display)]
pub enum ArtifactSetName {
    Adventurer,
    ArchaicPetra,
    Berserker,
    BlizzardStrayer,
    BloodstainedChivalry,
    BraveHeart,
    CrimsonWitchOfFlames,
    DefendersWill,
    EmblemOfSeveredFate,
    Gambler,
    GladiatorsFinale,
    HeartOfDepth,
    HuskOfOpulentDreams,
    Instructor,
    Lavawalker,
    LuckyDog,
    MaidenBeloved,
    MartialArtist,
    NoblesseOblige,
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
    ViridescentVenerer,
    WanderersTroupe,
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
    pub id: usize
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
        // todo currently it's not random
        Artifact {
            set_name: ArtifactSetName::random(),
            slot,
            level: 20,
            star: 5,
            sub_stats: vec![
                (StatName::ATKPercentage, 0.1),
                (StatName::ATKFixed, 10.0),
                (StatName::Recharge, 0.1),
                (StatName::ElementalMastery, 10.0),
            ],
            main_stat: (StatName::ATKPercentage, 0.1),
            id: thread_rng().gen()
        }
    }
}

pub struct ArtifactList<'a, 'b> {
    pub artifacts: &'a Vec<&'b Artifact>
    // pub artifacts: &'a SmallVec<[&'b Artifact; 5]>
}

impl<'a, 'b> ArtifactList<'a, 'b> {
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
