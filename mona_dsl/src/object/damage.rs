use std::cell::RefCell;
use std::rc::Rc;
use mona::common::DamageResult;
use mona::damage::transformative_damage::TransformativeDamage;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::mona_object::{MonaObject, MonaObjectEnum, MonaObjectTrait};

pub struct MonaObjectDamage {
    pub normal: DamageResult,
    pub melt: Option<DamageResult>,
    pub vaporize: Option<DamageResult>,
    pub spread: Option<DamageResult>,
    pub aggravate: Option<DamageResult>,
    pub is_heal: bool,
    pub is_shield: bool,
}

impl MonaObjectTrait for MonaObjectDamage {
    fn access(&self, key: &MonaObject) -> Result<Rc<RefCell<MonaObject>>, RuntimeError> {
        let k = match &key.data {
            MonaObjectEnum::String(x) => x.value.clone(),
            _ => {
                return Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, &format!("cannot access `Damage` object with type `{}`", key.get_type())));
            }
        };

        let result = match k.as_str() {
            "normal" | "n" => MonaObjectDamageNumber::from_damage_result(&self.normal),
            "melt" | "m" => {
                if let Some(ref x) = self.melt {
                    MonaObjectDamageNumber::from_damage_result(x)
                } else {
                    return Err(RuntimeError::new(RuntimeErrorEnum::DamageNotFound, "damage `melt` not exist"));
                }
            },
            "vaporize" | "v" => {
                if let Some(ref x) = self.vaporize {
                    MonaObjectDamageNumber::from_damage_result(x)
                } else {
                    return Err(RuntimeError::new(RuntimeErrorEnum::DamageNotFound, "damage `vaporize` not exist"));
                }
            },
            "spread" => {
                if let Some(ref x) = self.spread {
                    MonaObjectDamageNumber::from_damage_result(x)
                } else {
                    return Err(RuntimeError::new(RuntimeErrorEnum::DamageNotFound, "damage `spread` not exist"));
                }
            },
            "aggravate" => {
                if let Some(ref x) = self.aggravate {
                    MonaObjectDamageNumber::from_damage_result(x)
                } else {
                    return Err(RuntimeError::new(RuntimeErrorEnum::DamageNotFound, "damage `aggravate` not exist"));
                }
            }
            x => return Err(RuntimeError::new(RuntimeErrorEnum::DamageNotFound, &format!("damage `{}` not exist", x)))
        };

        let obj = MonaObject {
            data: MonaObjectEnum::DamageNumber(result)
        };

        Ok(Rc::new(RefCell::new(obj)))
    }
}

#[derive(Debug)]
pub struct MonaObjectDamageNumber {
    pub expect: f64,
    pub critical: f64,
    pub non_critical: f64
}

impl MonaObjectDamageNumber {
    pub fn from_damage_result(r: &DamageResult) -> Self {
        Self {
            expect: r.expectation,
            critical: r.critical,
            non_critical: r.non_critical
        }
    }
}

impl MonaObjectTrait for MonaObjectDamageNumber {
    fn access(&self, key: &MonaObject) -> Result<Rc<RefCell<MonaObject>>, RuntimeError> {
        let k = match &key.data {
            MonaObjectEnum::String(x) => x.value.clone(),
            _ => {
                return Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, &format!("cannot access `DamageNumber` object with type `{}`", key.get_type())));
            }
        };

        let value = match k.as_str() {
            "expect" | "expectation" | "e" => self.expect,
            "critical" | "crit" | "c" => self.critical,
            "non_critical" | "non_crit" | "n" => self.non_critical,
            x => {
                return Err(RuntimeError::new(RuntimeErrorEnum::DamageNotFound, &format!("damage value `{}` not exist", x)))
            }
        };

        let obj = MonaObject::new_number(value);
        Ok(Rc::new(RefCell::new(obj)))
    }
}

pub struct MonaObjectTransformativeDamage {
    pub damage: TransformativeDamage
}

impl MonaObjectTrait for MonaObjectTransformativeDamage {
    fn access(&self, key: &MonaObject) -> Result<Rc<RefCell<MonaObject>>, RuntimeError> {
        let k = match &key.data {
            MonaObjectEnum::String(x) => x.value.clone(),
            _ => {
                return Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, &format!("cannot access `TransformativeDamage` object with type `{}`", key.get_type())));
            }
        };

        let value = match k.as_str() {
            "swirl_cryo" => self.damage.swirl_cryo,
            "swirl_pyro" => self.damage.swirl_pyro,
            "swirl_hydro" => self.damage.swirl_hydro,
            "swirl_electro" => self.damage.swirl_electro,
            "overload" => self.damage.overload,
            "electro_charged" => self.damage.electro_charged,
            "shatter" => self.damage.shatter,
            "superconduct" | "super_conduct" => self.damage.superconduct,
            "bloom" => self.damage.bloom,
            "burgeon" => self.damage.burgeon,
            "hyperbloom" => self.damage.hyperbloom,
            "crystallize" => self.damage.crystallize,
            "burning" => self.damage.burning,
            x => {
                return Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, &format!("`TransformativeDamage` doesn't have prop name `{}`", x)));
            }
        };

        let obj = MonaObject::new_number(value);
        Ok(Rc::new(RefCell::new(obj)))
    }
}
