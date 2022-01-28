use std::collections::HashMap;

use crate::common::{StatName, SUB_STAT_VALUE_5};
use crate::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use crate::utils::log;

pub struct TargetFunctionOptConfig {
    pub atk_fixed: f64,
    pub atk_percentage: f64,
    pub hp_fixed: f64,
    pub hp_percentage: f64,
    pub def_fixed: f64,
    pub def_percentage: f64,
    pub recharge: f64,
    pub elemental_mastery: f64,
    pub critical: f64,
    pub critical_damage: f64,
    pub bonus_electro: f64,
    pub bonus_pyro: f64,
    pub bonus_hydro: f64,
    pub bonus_anemo: f64,
    pub bonus_cryo: f64,
    pub bonus_geo: f64,
    pub bonus_dendro: f64,
    pub bonus_physical: f64,

    pub sand_main_stats: Vec<StatName>,
    pub goblet_main_stats: Vec<StatName>,
    pub head_main_stats: Vec<StatName>,

    pub set_names: Option<Vec<ArtifactSetName>>,
}

impl TargetFunctionOptConfig {
    pub fn score(&self, artifact: &Artifact) -> f64 {
        let mut map: HashMap<StatName, i32> = HashMap::new();

        *map.entry(artifact.main_stat.0).or_insert(0) += 1;

        for (stat_name, _) in artifact.sub_stats.iter() {
            *map.entry(*stat_name).or_insert(0) += 1;
        }

        let mut s = 0.0;
        s += self.atk_fixed / SUB_STAT_VALUE_5.atk_fixed[3] * map.get(&StatName::ATKFixed).cloned().unwrap_or(0) as f64;
        s += self.atk_percentage / SUB_STAT_VALUE_5.atk_percentage[3] * map.get(&StatName::ATKPercentage).cloned().unwrap_or(0) as f64;
        s += self.hp_fixed / SUB_STAT_VALUE_5.hp_fixed[3] * map.get(&StatName::HPFixed).cloned().unwrap_or(0) as f64;
        s += self.hp_percentage / SUB_STAT_VALUE_5.hp_percentage[3] * map.get(&StatName::HPPercentage).cloned().unwrap_or(0) as f64;
        s += self.def_fixed / SUB_STAT_VALUE_5.def_fixed[3] * map.get(&StatName::DEFFixed).cloned().unwrap_or(0) as f64;
        s += self.def_percentage / SUB_STAT_VALUE_5.def_percentage[3] * map.get(&StatName::DEFPercentage).cloned().unwrap_or(0) as f64;
        s += self.recharge / SUB_STAT_VALUE_5.recharge[3] * map.get(&StatName::Recharge).cloned().unwrap_or(0) as f64;
        s += self.elemental_mastery / SUB_STAT_VALUE_5.elemental_mastery[3] * map.get(&StatName::ElementalMastery).cloned().unwrap_or(0) as f64;
        s += self.critical / SUB_STAT_VALUE_5.critical_rate[3] * map.get(&StatName::CriticalRate).cloned().unwrap_or(0) as f64;
        s += self.critical_damage / SUB_STAT_VALUE_5.critical_damage[3] * map.get(&StatName::CriticalDamage).cloned().unwrap_or(0) as f64;
        s += self.bonus_hydro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::HydroBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_anemo / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::AnemoBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_cryo / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::CryoBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_electro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::ElectroBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_pyro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::PyroBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_geo / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::GeoBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_dendro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::DendroBonus).cloned().unwrap_or(0) as f64;
        s += self.bonus_physical / SUB_STAT_VALUE_5.physical_bonus[3] * map.get(&StatName::PhysicalBonus).cloned().unwrap_or(0) as f64;

        s
    }

    pub fn is_critical_set_name(&self, set_name: ArtifactSetName) -> bool {
        match self.set_names {
            None => true,
            Some(ref x) => x.contains(&set_name),
        }
    }

    pub fn filter_main_stats<'a>(&self, artifacts: Vec<&'a Artifact>) -> Vec<&'a Artifact> {
        let mut sands: HashMap<ArtifactSetName, Vec<&Artifact>> = HashMap::new();
        let mut goblets: HashMap<ArtifactSetName, Vec<&Artifact>> = HashMap::new();
        let mut heads: HashMap<ArtifactSetName, Vec<&Artifact>> = HashMap::new();
        let mut results: Vec<&'a Artifact> = vec![];

        for &artifact in artifacts.iter() {
            match artifact.slot {
                ArtifactSlotName::Sand => sands.entry(artifact.set_name).or_insert(Vec::new()).push(artifact),
                ArtifactSlotName::Goblet => goblets.entry(artifact.set_name).or_insert(Vec::new()).push(artifact),
                ArtifactSlotName::Head => heads.entry(artifact.set_name).or_insert(Vec::new()).push(artifact),
                _ => results.push(artifact),
            }
        }

        // let mut sand_flag = false;
        // let mut goblet_flag = false;
        // let mut head_flag = false;

        for (&set_name, arts) in sands.iter() {
            let flag = arts.iter().any(|x| self.sand_main_stats.contains(&x.main_stat.0));
            if !flag {
                results.extend(arts.iter());
            } else {
                results.extend(arts.iter().filter(|x| self.sand_main_stats.contains(&x.main_stat.0)));
            }
        }

        // log!("{}", results.len());
        for (set_name, arts) in goblets.iter() {
            let flag = arts.iter().any(|x| self.goblet_main_stats.contains(&x.main_stat.0));
            // log!("{:?}", arts);
            if !flag {
                results.extend(arts.iter());
            } else {
                results.extend(arts.iter().filter(|x| self.goblet_main_stats.contains(&x.main_stat.0)));
            }
        }
        // log!("{}", results.len());

        for (&set_name, arts) in heads.iter() {
            let flag = arts.iter().any(|x| self.head_main_stats.contains(&x.main_stat.0));
            if !flag {
                results.extend(arts.iter());
            } else {
                results.extend(arts.iter().filter(|x| self.head_main_stats.contains(&x.main_stat.0)));
            }
        }

        results
    }

    pub fn filter_sub_stats<'a>(&self, artifacts: Vec<&'a Artifact>) -> Vec<&'a Artifact> {
        // log!("{:?}", artifacts);
        let mut flowers: HashMap<StatName, Vec<(&Artifact, f64)>> = HashMap::new();
        let mut feathers: HashMap<StatName, Vec<(&Artifact, f64)>> = HashMap::new();
        let mut sands: HashMap<StatName, Vec<(&Artifact, f64)>> = HashMap::new();
        let mut goblets: HashMap<StatName, Vec<(&Artifact, f64)>> = HashMap::new();
        let mut heads: HashMap<StatName, Vec<(&Artifact, f64)>> = HashMap::new();

        for &artifact in artifacts.iter() {
            let score = self.score(artifact);
            match artifact.slot {
                ArtifactSlotName::Flower => flowers.entry(artifact.main_stat.0).or_insert(Vec::new()).push((artifact, score)),
                ArtifactSlotName::Feather => feathers.entry(artifact.main_stat.0).or_insert(Vec::new()).push((artifact, score)),
                ArtifactSlotName::Sand => sands.entry(artifact.main_stat.0).or_insert(Vec::new()).push((artifact, score)),
                ArtifactSlotName::Goblet => goblets.entry(artifact.main_stat.0).or_insert(Vec::new()).push((artifact, score)),
                ArtifactSlotName::Head => heads.entry(artifact.main_stat.0).or_insert(Vec::new()).push((artifact, score)),
            }
        }

        let mut results: Vec<&'a Artifact> = Vec::new();
        let temp = vec![flowers, feathers, sands, goblets, heads];
        for entry in temp.iter() {
            for (stat_name, arts) in entry.iter() {
                let max_score = arts.iter().map(|x| (*x).1).fold(-f64::INFINITY, f64::max);
                for (art, score) in arts.iter() {
                    let relative = score / max_score;
                    let is_critical_set_name = self.is_critical_set_name(art.set_name);
                    if (!is_critical_set_name && relative >= 0.8) || (is_critical_set_name && relative >= 0.5) {
                        results.push(art);
                    }
                }
            }
        }

        let flag = results.iter().any(|x| x.slot == ArtifactSlotName::Goblet);

        results
    }

    pub fn filter<'a>(&self, artifacts: Vec<&'a Artifact>) -> Vec<&'a Artifact> {
        let mut temp = self.filter_main_stats(artifacts);
        self.filter_sub_stats(temp)

        // temp
    }
}