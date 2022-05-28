use std::cell::RefCell;
use std::rc::Rc;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::{MonaObjectBool, MonaObjectNumber, MonaObjectObject};
use crate::object::builtin_function::MonaObjectBuiltinFunction;
use crate::object::damage::{MonaObjectDamage, MonaObjectDamageNumber, MonaObjectTransformativeDamage};
use crate::object::damage_config::MonaObjectDamageConfig;
use crate::object::string::MonaObjectString;

pub enum MonaObjectEnum {
    Number(MonaObjectNumber),
    Bool(MonaObjectBool),
    Object(MonaObjectObject),
    BuiltinFunction(MonaObjectBuiltinFunction),
    String(MonaObjectString),

    DamageConfig(MonaObjectDamageConfig),
    Damage(MonaObjectDamage),
    TransformativeDamage(MonaObjectTransformativeDamage),
    DamageNumber(MonaObjectDamageNumber),
}

pub struct MonaObject {
    pub data: MonaObjectEnum,
}

impl MonaObject {
    pub fn get_type(&self) -> &'static str {
        use MonaObjectEnum::*;
        match &self.data {
            Number(_) => "number",
            Object(_) => "object",
            Bool(_) => "bool",
            BuiltinFunction(_) => "builtin_function",
            DamageConfig(_) => "damage_config",
            String(_) => "string",
            Damage(_) => "normal_damage",
            DamageNumber(_) => "damage_number",
            TransformativeDamage(_) => "transformative_damage",
        }
    }

    pub fn access(&self, key: Rc<RefCell<MonaObject>>) -> Result<Rc<RefCell<MonaObject>>, RuntimeError> {
        match &self.data {
            MonaObjectEnum::Object(x) => x.access(&*key.borrow()),
            MonaObjectEnum::Damage(x) => x.access(&*key.borrow()),
            MonaObjectEnum::DamageNumber(x) => x.access(&*key.borrow()),
            MonaObjectEnum::TransformativeDamage(x) => x.access(&*key.borrow()),
            _ => todo!()
        }
    }

    pub fn is_number(&self) -> bool {
        if let MonaObjectEnum::Number(_) = self.data { true } else { false }
    }

    pub fn assert_number(&self) -> Result<f64, RuntimeError> {
        if let MonaObjectEnum::Number(x) = &self.data {
            Ok(x.value)
        } else {
            Err(RuntimeError::new(RuntimeErrorEnum::TypeError, &format!("expecting `number`, found `{}`", self.get_type())))
        }
    }

    pub fn get_number(&self) -> f64 {
        if let MonaObjectEnum::Number(x) = &self.data { x.value } else { panic!("object is not number") }
    }

    pub fn is_bool(&self) -> bool {
        if let MonaObjectEnum::Bool(_) = self.data { true } else { false }
    }

    pub fn get_bool(&self) -> bool {
        if let MonaObjectEnum::Bool(x) = &self.data { x.value } else { panic!("object is not bool") }
    }

    pub fn assert_bool(&self) -> Result<bool, RuntimeError> {
        if let MonaObjectEnum::Bool(x) = &self.data {
            Ok(x.value)
        } else {
            Err(RuntimeError::new(RuntimeErrorEnum::TypeError, &format!("expecting `bool`, found `{}`", self.get_type())))
        }
    }

    pub fn is_string(&self) -> bool {
        if let MonaObjectEnum::String(_) = self.data { true } else { false }
    }

    pub fn get_string(&self) -> &str {
        if let MonaObjectEnum::String(x) = &self.data { x.value.as_str() } else { panic!("object is not string") }
    }

    pub fn new_number(v: f64) -> Self {
        let obj = MonaObjectNumber {
            value: v,
            hash: v.to_bits()
        };
        MonaObject {
            data: MonaObjectEnum::Number(obj)
        }
    }

    pub fn new_bool(value: bool) -> Self {
        MonaObject {
            data: MonaObjectEnum::Bool(MonaObjectBool { value })
        }
    }

    pub fn new_string(s: Rc<String>) -> Self {
        MonaObject {
            data: MonaObjectEnum::String(MonaObjectString::new(s))
        }
    }
}

pub trait MonaObjectTrait {
    fn access(&self, key: &MonaObject) -> Result<Rc<RefCell<MonaObject>>, RuntimeError> {
        Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, "cannot access member in `bool`"))
    }
}

// impl
