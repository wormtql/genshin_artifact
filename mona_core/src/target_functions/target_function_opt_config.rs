use std::collections::HashMap;

use crate::common::{StatName, SUB_STAT_VALUE_5};
use crate::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};

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
    pub healing_bonus: f64,
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

    // if none, every set is critical
    pub set_names: Option<Vec<ArtifactSetName>>,
    // if none, no set is very critical
    pub very_critical_set_names: Option<Vec<ArtifactSetName>>,

    pub normal_threshold: f64,      // defaults to 0.7
    pub critical_threshold: f64,    // defaults to 0.5
    pub very_critical_threshold: f64,   // defaults to 0.0
}

impl TargetFunctionOptConfig {
    pub const DEFAULT_NORMAL_THRESHOLD: f64 = 0.75;
    pub const DEFAULT_CRITICAL_THRESHOLD: f64 = 0.5;
    pub const DEFAULT_VERY_CRITICAL_THRESHOLD: f64 = 0.1;

    pub fn score(&self, artifact: &Artifact) -> f64 {
        let mut map: HashMap<StatName, f64> = HashMap::new();

        *map.entry(artifact.main_stat.0).or_insert(0.0) += artifact.main_stat.1;

        for (stat_name, stat_value) in artifact.sub_stats.iter() {
            *map.entry(*stat_name).or_insert(0.0) += *stat_value;
        }

        let mut s = 0.0;
        s += self.atk_fixed / SUB_STAT_VALUE_5.atk_fixed[3] * map.get(&StatName::ATKFixed).cloned().unwrap_or(0.0) as f64;
        s += self.atk_percentage / SUB_STAT_VALUE_5.atk_percentage[3] * map.get(&StatName::ATKPercentage).cloned().unwrap_or(0.0) as f64;
        s += self.hp_fixed / SUB_STAT_VALUE_5.hp_fixed[3] * map.get(&StatName::HPFixed).cloned().unwrap_or(0.0) as f64;
        s += self.hp_percentage / SUB_STAT_VALUE_5.hp_percentage[3] * map.get(&StatName::HPPercentage).cloned().unwrap_or(0.0) as f64;
        s += self.def_fixed / SUB_STAT_VALUE_5.def_fixed[3] * map.get(&StatName::DEFFixed).cloned().unwrap_or(0.0) as f64;
        s += self.def_percentage / SUB_STAT_VALUE_5.def_percentage[3] * map.get(&StatName::DEFPercentage).cloned().unwrap_or(0.0) as f64;
        s += self.recharge / SUB_STAT_VALUE_5.recharge[3] * map.get(&StatName::Recharge).cloned().unwrap_or(0.0) as f64;
        s += self.elemental_mastery / SUB_STAT_VALUE_5.elemental_mastery[3] * map.get(&StatName::ElementalMastery).cloned().unwrap_or(0.0) as f64;
        s += self.critical / SUB_STAT_VALUE_5.critical_rate[3] * map.get(&StatName::CriticalRate).cloned().unwrap_or(0.0) as f64;
        s += self.critical_damage / SUB_STAT_VALUE_5.critical_damage[3] * map.get(&StatName::CriticalDamage).cloned().unwrap_or(0.0) as f64;
        s += self.healing_bonus / SUB_STAT_VALUE_5.healing_bonus[3] * map.get(&StatName::HealingBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_hydro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::HydroBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_anemo / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::AnemoBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_cryo / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::CryoBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_electro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::ElectroBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_pyro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::PyroBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_geo / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::GeoBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_dendro / SUB_STAT_VALUE_5.elemental_bonus[3] * map.get(&StatName::DendroBonus).cloned().unwrap_or(0.0) as f64;
        s += self.bonus_physical / SUB_STAT_VALUE_5.physical_bonus[3] * map.get(&StatName::PhysicalBonus).cloned().unwrap_or(0.0) as f64;

        s
    }

    pub fn score_normalized(&self, artifact: &Artifact) -> f64 {
        let sum = self.atk_percentage + self.atk_fixed + self.hp_percentage + self.hp_fixed + self.def_percentage + self.def_fixed
            + self.recharge + self.elemental_mastery + self.critical + self.critical_damage
            + self.healing_bonus
            + self.bonus_anemo + self.bonus_geo + self.bonus_cryo + self.bonus_pyro + self.bonus_hydro + self.bonus_dendro + self.bonus_electro
            + self.bonus_physical;
        let score1 = self.score(artifact);

        score1 / sum
    }

    pub fn is_critical_set_name(&self, set_name: ArtifactSetName) -> bool {
        match self.set_names {
            None => true,
            Some(ref x) => x.contains(&set_name),
        }
    }

    pub fn is_very_critical_set_name(&self, set_name: ArtifactSetName) -> bool {
        match self.very_critical_set_names {
            None => false,
            Some(ref x) => x.contains(&set_name)
        }
    }

    pub fn filter_main_stats_aggressive<'a>(&self, artifacts: Vec<&'a Artifact>) -> Vec<&'a Artifact> {
        let mut sands: Vec<&Artifact> = Vec::new();
        let mut goblets: Vec<&Artifact> = Vec::new();
        let mut heads: Vec<&Artifact> = Vec::new();
        let mut results: Vec<&'a Artifact> = Vec::new();

        for &art in artifacts.iter() {
            match art.slot {
                ArtifactSlotName::Sand => sands.push(art),
                ArtifactSlotName::Goblet => goblets.push(art),
                ArtifactSlotName::Head => heads.push(art),
                _ => results.push(art)
            }
        }

        let sands_filter: Vec<&Artifact> = sands.iter().cloned().filter(|x| self.sand_main_stats.contains(&x.main_stat.0) || self.is_very_critical_set_name(x.set_name)).collect();
        if sands_filter.len() > 0 {
            results.extend(sands_filter.iter());
        } else {
            results.extend(sands.iter());
        }

        let goblets_filter: Vec<&Artifact> = goblets.iter().cloned().filter(|x| self.goblet_main_stats.contains(&x.main_stat.0) || self.is_very_critical_set_name(x.set_name)).collect();
        if goblets_filter.len() > 0 {
            results.extend(goblets_filter.iter());
        } else {
            results.extend(goblets.iter());
        }

        let heads_filter: Vec<&Artifact> = heads.iter().cloned().filter(|x| self.head_main_stats.contains(&x.main_stat.0) || self.is_very_critical_set_name(x.set_name)).collect();
        if heads_filter.len() > 0 {
            results.extend(heads_filter.iter());
        } else {
            results.extend(heads.iter());
        }

        results
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

        for (&_set_name, arts) in sands.iter() {
            let flag = arts.iter().any(|x| self.sand_main_stats.contains(&x.main_stat.0));
            if !flag {
                results.extend(arts.iter());
            } else {
                results.extend(arts.iter().filter(|x| self.sand_main_stats.contains(&x.main_stat.0)));
            }
        }

        // log!("{}", results.len());
        for (&_set_name, arts) in goblets.iter() {
            let flag = arts.iter().any(|x| self.goblet_main_stats.contains(&x.main_stat.0));
            // log!("{:?}", arts);
            if !flag {
                results.extend(arts.iter());
            } else {
                results.extend(arts.iter().filter(|x| self.goblet_main_stats.contains(&x.main_stat.0)));
            }
        }
        // log!("{}", results.len());

        for (&_set_name, arts) in heads.iter() {
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
                    let is_very_critical_set_name = self.is_very_critical_set_name(art.set_name);
                    if (!is_critical_set_name && !is_very_critical_set_name && relative >= self.normal_threshold)
                        || (is_critical_set_name && relative >= self.critical_threshold)
                        || (is_very_critical_set_name && relative >= self.very_critical_threshold) {
                        results.push(art);
                    }
                }
            }
        }

        results
    }

    pub fn filter<'a>(&self, artifacts: Vec<&'a Artifact>) -> Vec<&'a Artifact> {
        // let mut temp = self.filter_main_stats(artifacts);
        let temp = self.filter_main_stats_aggressive(artifacts);
        self.filter_sub_stats(temp)

        // temp
    }
}