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
}

pub const ARTIFACT_EFF5: ArtifactEff = ArtifactEff {
    atk: [11.0, 12.0, 14.0, 16.0],
    atk_percentage: [0.033, 0.037, 0.042, 0.047],
    def: [13.0, 15.0, 17.0, 19.0],
    def_percentage: [0.041, 0.047, 0.053, 0.058],
    hp: [167.0, 191.0, 215.0, 239.0],
    hp_percentage: [0.033, 0.037, 0.042, 0.047],
    critical_rate: [0.022, 0.025, 0.028, 0.031],
    critical_damage: [0.044, 0.05, 0.056, 0.062],
    recharge: [0.036, 0.041, 0.047, 0.052],
    elemental_mastery: [13.0, 15.0, 17.0, 19.0]
};
