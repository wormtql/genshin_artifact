use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use mona::attribute::SimpleAttributeGraph2;
use mona::character::{Character, CharacterName};
use mona::character::characters::damage;
use mona::damage::damage_result::SimpleDamageResult;
use mona::damage::{DamageContext, SimpleDamageBuilder};
use crate::builtin::global_function::setup_global_namespace;
use crate::common::UnsafeDamageContext;
use crate::compiler::compiler::CodeObject;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::damage::{MonaObjectDamage, MonaObjectTransformativeDamage};
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::object::MonaObjectBool;
use crate::object::prop::get_prop_value;
use crate::vm::namespace::Namespace;
use crate::vm::stream::{OutputStream, PrintOutputStream};

pub struct MonaEnv {
    pub stack: Vec<Rc<RefCell<MonaObject>>>,
    pub namespace: Namespace,
    pub global_namespace: Namespace,
    pub code_object: Rc<CodeObject>,
    pub pc: usize,

    pub bool_true: Rc<RefCell<MonaObject>>,
    pub bool_false: Rc<RefCell<MonaObject>>,

    pub damage_ctx: HashMap<CharacterName, UnsafeDamageContext>,

    pub ostream: Box<dyn OutputStream>
}

impl MonaEnv {
    pub fn new(codes: CodeObject) -> MonaEnv {
        let global_ns = setup_global_namespace();

        MonaEnv {
            stack: Vec::new(),
            namespace: Namespace::new(),
            global_namespace: global_ns,
            code_object: Rc::new(codes),
            pc: 0,

            bool_false: Rc::new(RefCell::new(MonaObject::new_bool(false))),
            bool_true: Rc::new(RefCell::new(MonaObject::new_bool(true))),
            damage_ctx: HashMap::new(),

            ostream: Box::new(PrintOutputStream)
        }
    }

    /// invoked by anything using mona dsl, providing a damage context
    pub fn add_damage_context(&mut self, ctx: UnsafeDamageContext) {
        let name = unsafe { (*ctx.character_common_data).name };
        self.damage_ctx.insert(name, ctx);
    }

    pub fn clear_local_state(&mut self) {
        self.namespace.map.clear();
        self.damage_ctx.clear();
    }

    /// when damage context is added, use damage configs in code object to generate Damage or DamageTransformative
    pub fn init_damage(&mut self) -> Result<(), RuntimeError> {
        for damage_config in self.code_object.damage_configs.iter() {
            let name = damage_config.character_name;
            if !self.damage_ctx.contains_key(&name) {
                // context not exist
                let e = RuntimeError::new(
                    RuntimeErrorEnum::CharacterContextNotFound,
                    &format!("character context `{}` not found", name.to_string())
                );
                return Err(e);
            }

            let unsafe_context = self.damage_ctx.get(&name).unwrap();
            let context: DamageContext<'_, SimpleAttributeGraph2> = unsafe {
                DamageContext {
                    character_common_data: &*unsafe_context.character_common_data,
                    enemy: &*unsafe_context.enemy,
                    attribute: &*unsafe_context.attribute
                }
            };

            if damage_config.is_transformative {
                let t_damage = context.transformative();
                let obj = MonaObjectTransformativeDamage {
                    damage: t_damage
                };
                let obj = MonaObject {
                    data: MonaObjectEnum::TransformativeDamage(obj)
                };

                let var_name = &damage_config.var_name;
                self.namespace.insert(var_name.clone(), obj);
            } else {
                let damage: SimpleDamageResult = damage::<SimpleDamageBuilder>(
                    &context, damage_config.skill_index, &damage_config.skill_config, damage_config.fumo.clone(),
                );
                // let damage: SimpleDamageResult = name.damage(context, damage_config.skill_index, &damage_config.skill_config);
                let obj = MonaObjectDamage {
                    normal: damage.normal.clone(),
                    melt: damage.melt.clone(),
                    vaporize: damage.vaporize.clone(),
                    spread: damage.spread.clone(),
                    aggravate: damage.aggravate.clone(),
                    is_heal: damage.is_heal,
                    is_shield: damage.is_shield
                };
                let obj = MonaObject {
                    data: MonaObjectEnum::Damage(obj)
                };

                let var_name = &damage_config.var_name;
                self.namespace.insert(var_name.clone(), obj);
            }
        }

        Ok(())
    }

    /// when damage context is added, use prop configs in code object to generate Number
    pub fn init_prop(&mut self) -> Result<(), RuntimeError> {
        for prop_config in self.code_object.prop_configs.iter() {
            let name = prop_config.character_name;
            if !self.damage_ctx.contains_key(&name) {
                // context not exist
                let e = RuntimeError::new(
                    RuntimeErrorEnum::CharacterContextNotFound,
                    &format!("character context `{}` not found", name.to_string())
                );
                return Err(e);
            }

            let context = self.damage_ctx.get(&name).unwrap();
            let attribute = context.attribute;

            let prop_name = &prop_config.prop_name;
            let prop_value = unsafe {
                get_prop_value(&*attribute, prop_name)?
            };

            let number = MonaObject::new_number(prop_value);
            let var_name = &prop_config.var_name;
            self.namespace.insert(var_name.clone(), number);
        }

        Ok(())
    }

    /// set output stream
    pub fn set_ostream(&mut self, os: Box<dyn OutputStream>) {
        self.ostream = os;
    }

    pub fn pop_stack(&mut self) -> Rc<RefCell<MonaObject>> {
        self.stack.pop().unwrap()
    }

    pub fn push_stack(&mut self, value: Rc<RefCell<MonaObject>>) {
        self.stack.push(value);
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        let iter = self.code_object.clone();
        for c in iter.codes.iter() {
            (*c).execute(self)?;
        }

        Ok(())
    }
}