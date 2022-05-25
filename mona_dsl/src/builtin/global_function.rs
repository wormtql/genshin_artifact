use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::builtin_function::{FunctionReturnType, MonaObjectBuiltinFunction, ParamVecType};
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::vm::namespace::Namespace;

fn mona_print_internal(obj: &MonaObject) {
    match &obj.data {
        MonaObjectEnum::Number(x) => println!("MONA: {}", x.value),
        MonaObjectEnum::String(x) => println!("MONA: {}", x.value),
        MonaObjectEnum::BuiltinFunction(x) => println!("MONA: [[function `{}`]]", x.name),
        MonaObjectEnum::DamageNumber(x) => println!("MONA: {:?}", x),
        _ => todo!()
    }
}

pub fn mona_print(params: ParamVecType) -> FunctionReturnType {
    for item in params.iter() {
        mona_print_internal(&*item.borrow());
    }

    Ok(None)
}

pub fn mona_type(params: ParamVecType) -> FunctionReturnType {
    let item = params[0].clone();
    let name = item.borrow().get_type();

    let s = MonaObject::new_string(Rc::new(String::from(name)));

    Ok(Some(Rc::new(RefCell::new(s))))
}

pub fn mona_max(params: ParamVecType) -> FunctionReturnType {
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

pub fn mona_min(params: ParamVecType) -> FunctionReturnType {
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

    Namespace {
        map
    }
}