use std::collections::HashMap;
use crate::attribute::Attribute;
use crate::buffs::Buff;
use crate::character::CharacterName;
use crate::target_functions::TargetFunction;
use crate::team::team::Team;
use crate::team_target::team_name::TeamName;
use crate::team_target::team_target_config::TeamTargetFunctionConfig;

pub trait TeamTargetFunction {
    fn target(&self, values: &HashMap<usize, f64>) -> f64;

    fn get_default_individual_targets(&self) -> HashMap<usize, Box<dyn TargetFunction>>;
}

pub struct TeamTargetFunctionMetaData {
    pub name: TeamName,
    pub chs: &'static str,
}

pub trait TeamTargetFunctionMetaTrait {
    const MEMBERS: &'static [CharacterName];

    const META: TeamTargetFunctionMetaData;

    fn get_default_buffs<A: Attribute>(team: &Team<A>) -> HashMap<usize, Vec<Box<dyn Buff<A>>>>;

    fn create<A: Attribute>(config: &TeamTargetFunctionConfig, team: &Team<A>) -> Box<dyn TeamTargetFunction>;
}
