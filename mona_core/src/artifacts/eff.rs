use crate::common::StatName;

pub struct ArtifactEff {
    pub atk: [f64; 4],
    pub atk_percentage: [f64; 4],
    pub def: [f64; 4],
    pub def_percentage: [f64; 4],
    pub hp: [f64; 4],
    pub hp_percentage: [f64; 4],
    pub critical_rate: [f64; 4],
    pub critical_damage: [f64; 4],
    pub recharge: [f64; 4],
    pub elemental_mastery: [f64; 4],
    pub healing_bonus: [f64; 4],
}

impl ArtifactEff {
    pub fn get_value(&self, stat: StatName, index: usize) -> f64 {
        use StatName::*;
        match stat {
            ATKFixed => self.atk[index],
            ATKPercentage => self.atk_percentage[index],
            DEFFixed => self.def[index],
            DEFPercentage => self.def_percentage[index],
            HPFixed => self.hp[index],
            HPPercentage => self.hp_percentage[index],
            CriticalRate => self.critical_rate[index],
            CriticalDamage => self.critical_damage[index],
            Recharge => self.recharge[index],
            ElementalMastery => self.elemental_mastery[index],
            HealingBonus => self.healing_bonus[index],
            PhysicalBonus => self.def_percentage[index],
            _ => self.atk_percentage[index],
            // _ => unreachable!()
        }
    }
}

pub const ARTIFACT_EFF4: ArtifactEff = ArtifactEff {
    atk: [11.0, 12.0, 14.0, 16.0],
    atk_percentage: [0.033, 0.037, 0.042, 0.047],
    def: [13.0, 15.0, 17.0, 19.0],
    def_percentage: [0.041, 0.047, 0.053, 0.058],
    hp: [167.0, 191.0, 215.0, 239.0],
    hp_percentage: [0.033, 0.037, 0.042, 0.047],
    critical_rate: [0.022, 0.025, 0.028, 0.031],
    critical_damage: [0.044, 0.05, 0.056, 0.062],
    recharge: [0.036, 0.041, 0.047, 0.052],
    elemental_mastery: [13.0, 15.0, 17.0, 19.0],
    healing_bonus: [0.025, 0.029, 0.032, 0.036],
};

pub const ARTIFACT_EFF5: ArtifactEff = ArtifactEff {
    atk: [14.0, 16.0, 18.0, 19.0],
    atk_percentage: [0.041, 0.047, 0.053, 0.058],
    def: [16.0, 19.0, 21.0, 23.0],
    def_percentage: [0.051, 0.058, 0.066, 0.073],
    hp: [209.0, 239.0, 269.0, 299.0],
    hp_percentage: [0.041, 0.047, 0.053, 0.058],
    critical_rate: [0.027, 0.031, 0.035, 0.039],
    critical_damage: [0.054, 0.062, 0.07, 0.078],
    recharge: [0.045, 0.052, 0.058, 0.065],
    elemental_mastery: [16.0, 19.0, 21.0, 23.0],
    healing_bonus: [0.031, 0.036, 0.04, 0.045],
};
