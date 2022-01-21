use std::ops::Mul;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
#[derive(Deserialize, Serialize)]
pub struct DamageResult {
    pub critical: f64,
    pub non_critical: f64,
    pub expectation: f64,
}

impl Mul<f64> for DamageResult {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        DamageResult {
            critical: self.critical * rhs,
            non_critical: self.non_critical * rhs,
            expectation: self.expectation * rhs,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleDamageResult {
    pub normal: DamageResult,
    pub melt: Option<DamageResult>,
    pub vaporize: Option<DamageResult>
}