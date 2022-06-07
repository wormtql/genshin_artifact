use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::builtin_function::{FunctionReturnType, MonaObjectBuiltinFunction, ParamVecType};
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::vm::env::MonaEnv;
use crate::vm::namespace::Namespace;

fn mona_print_internal(obj: &MonaObject, env: &mut MonaEnv) -> Result<(), RuntimeError> {
    match &obj.data {
        MonaObjectEnum::Number(x) => env.ostream.append_str(&format!("MONA: {}", x.value)),
        MonaObjectEnum::String(x) => env.ostream.append_str(&format!("MONA: {}", x.value)),
        MonaObjectEnum::BuiltinFunction(x) => env.ostream.append_str(&format!("MONA: [[function `{}`]]", x.name)),
        MonaObjectEnum::DamageNumber(x) => env.ostream.append_str(&format!("MONA: {:?}", x)),
        MonaObjectEnum::Bool(x) => env.ostream.append_str(&format!("MONA: {}", x.value)),
        // MonaObjectEnum::TransformativeDamage(x) => env.ostream.append_str(&format!("MONA: {:?}", x.damage)),
        _ => {
            return Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, &format!("print type `{}` not implelented", obj.get_type())));
        }
    }

    Ok(())
}

pub fn mona_print(params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
    for item in params.iter() {
        mona_print_internal(&*item.borrow(), env)?;
    }

    Ok(None)
}

pub fn mona_type(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    let item = params[0].clone();
    let name = item.borrow().get_type();

    let s = MonaObject::new_string(Rc::new(String::from(name)));

    Ok(Some(Rc::new(RefCell::new(s))))
}

pub fn mona_max(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    let mut result = -f64::INFINITY;
    for item in params.iter() {
        let v = item.borrow().assert_number()?;
        if v > result {
            result = v;
        }
    }

    let obj = MonaObject::new_number(result);
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn mona_min(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    let mut result = f64::INFINITY;
    for item in params.iter() {
        let v = item.borrow().assert_number()?;
        if v < result {
            result = v;
        }
    }

    let obj = MonaObject::new_number(result);
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn mona_select(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    if params.len() != 3 {
        return Err(RuntimeError::new(RuntimeErrorEnum::ParamError, &format!("requiring exact 3 params, got {}", params.len())));
    }
    let flag = params[0].borrow().assert_bool()?;

    let obj = if flag {
        params[1].clone()
    } else {
        params[2].clone()
    };

    return Ok(Some(obj))
}

pub fn mona_abs(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    if params.len() != 1 {
        return Err(RuntimeError::new(RuntimeErrorEnum::ParamError, &format!("requiring 1 param, got {}", params.len())));
    }

    let obj = &params[0];
    let number = obj.borrow().assert_number()?;

    let obj = MonaObject::new_number(number.abs());
    Ok(Some(Rc::new(RefCell::new(obj))))
}

macro insert_global($m:ident, $name:expr, $func:ident) {
    let t = MonaObjectBuiltinFunction {
        name: String::from($name),
        handler: Box::new($func)
    };
    ($m).insert(String::from($name), Rc::new(RefCell::new(MonaObject {
        data: MonaObjectEnum::BuiltinFunction(t)
    })));
}

pub fn setup_global_namespace() -> Namespace {
    let mut map = HashMap::new();

    insert_global!(map, "print", mona_print);
    insert_global!(map, "type", mona_type);
    insert_global!(map, "max", mona_max);
    insert_global!(map, "min", mona_min);
    insert_global!(map, "select", mona_select);
    insert_global!(map, "abs", mona_abs);

    Namespace {
        map
    }
}