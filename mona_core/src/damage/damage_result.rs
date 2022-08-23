use std::ops::Mul;
use serde::{Serialize, Deserialize};
use crate::damage::transformative_damage::TransformativeDamage;

#[derive(Debug, Clone, Copy)]
#[derive(Deserialize, Serialize)]
pub struct DamageResult {
    pub critical: f64,
    pub non_critical: f64,
    pub expectation: f64,

    pub is_heal: bool,
    pub is_shield: bool
}

impl Mul<f64> for DamageResult {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        DamageResult {
            critical: self.critical * rhs,
            non_critical: self.non_critical * rhs,
            expectation: self.expectation * rhs,
            is_shield: false,
            is_heal: false
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleDamageResult {
    pub normal: DamageResult,
    pub melt: Option<DamageResult>,
    pub vaporize: Option<DamageResult>,
    pub spread: Option<DamageResult>,
    pub aggravate: Option<DamageResult>,
    pub is_heal: bool,
    pub is_shield: bool,
}
