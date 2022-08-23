use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;
use mona::character::CharacterName;
use mona::character::skill_config::CharacterSkillConfig;
use mona::common::Element;
use pest::Span;
use crate::ast::expression::expression::{ASTBinaryExpression, ASTBool, ASTExpression, ASTFieldExpression, ASTFunctionCallExpression, ASTIdentifier, ASTNumber, ASTString, ASTUnaryExpression};
use crate::ast::program::ASTProgram;
use crate::ast::statement::{ASTAssignmentStatement, ASTDamageStatement, ASTExpressionStatement, ASTPropStatement, ASTStatement};
use crate::ast::traverse::{ASTTraverse, ASTTraverser};
use crate::code::byte_code::{ByteCodeAdd, ByteCodeSaveName};
use crate::code::byte_code::access::ByteCodeAccess;
use crate::code::byte_code::call::ByteCodeCall;
use crate::code::byte_code::div::ByteCodeDiv;
use crate::code::byte_code::eq::ByteCodeEq;
use crate::code::byte_code::ge::ByteCodeGe;
use crate::code::byte_code::gt::ByteCodeGt;
use crate::code::byte_code::le::ByteCodeLe;
use crate::code::byte_code::load_bool::ByteCodeLoadBool;
use crate::code::byte_code::load_name::ByteCodeLoadName;
use crate::code::byte_code::load_number::ByteCodeLoadNumber;
use crate::code::byte_code::load_string::ByteCodeLoadString;
use crate::code::byte_code::lt::ByteCodeLt;
use crate::code::byte_code::mul::ByteCodeMul;
use crate::code::byte_code::ne::ByteCodeNe;
use crate::code::byte_code::neg::ByteCodeNeg;
use crate::code::byte_code::pow::ByteCodePow;
use crate::code::byte_code::sub::ByteCodeSub;
use crate::compiler::compiler::{CodeObject, CompileContext, Compiler};
use crate::error::{CompileError, CompileErrorType};
use crate::object::damage_config::MonaObjectDamageConfig;
use crate::object::prop::{is_valid_prop_name, MonaObjectPropConfig};

struct Collector {
    names: HashSet<String>,
    numbers_bit: HashSet<u64>,
}

fn get_my_span(x: &Span) -> crate::common::Span {
    let (start_line, start_col) = x.start_pos().line_col();
    let (end_line, end_col) = x.end_pos().line_col();
    crate::common::Span {
        start_row: start_line,
        start_col,
        end_row: end_line,
        end_col
    }
}

impl Collector {
    fn new() -> Self {
        Collector {
            names: HashSet::new(),
            numbers_bit: HashSet::new()
        }
    }

    fn get_numbers(&self) -> Vec<f64> {
        let mut result = Vec::new();

        for &item in self.numbers_bit.iter() {
            result.push(f64::from_bits(item));
        }

        result
    }

    fn get_names(&self) -> Vec<String> {
        let mut result = Vec::new();

        for item in self.names.iter() {
            result.push(item.clone());
        }

        result
    }
}

impl<'i> ASTTraverser<'i> for Collector {
    fn enter_identifier(&mut self, node: Rc<RefCell<ASTIdentifier<'i>>>) {
        self.names.insert(node.borrow().ident.clone());
    }

    fn enter_number(&mut self, node: Rc<RefCell<ASTNumber<'i>>>) {
        let value = (*(*node).borrow()).value;
        let bits = value.to_bits();
        self.numbers_bit.insert(bits);
    }

    fn enter_string(&mut self, node: Rc<RefCell<ASTString<'i>>>) {
        self.names.insert((*node.borrow()).value.clone());
    }
}

pub struct MonaCompilerASTToCode<'i> {
    pub input: &'i str,
    pub ast: Rc<RefCell<ASTProgram<'i>>>,
}

impl<'i> MonaCompilerASTToCode<'i> {
    pub fn new(ast: Rc<RefCell<ASTProgram<'i>>>) -> MonaCompilerASTToCode<'i> {
        let input = ast.borrow().common.input;

        MonaCompilerASTToCode {
            input,
            ast
        }
    }

    pub fn compile(&self) -> Result<CodeObject, CompileError> {
        let ast = self.ast.clone();

        let mut collector = Collector::new();
        let mut traverse = ASTTraverse {
            traverser: &mut collector
        };
        traverse.traverse_program(ast);

        let names = collector.get_names();
        let numbers = collector.get_numbers();

        let mut compile_context = CompileContext::new();
        println!("{:?}", names);
        println!("{:?}", numbers);

        for name in names.iter() {
            compile_context.add_name_const(name);
        }

        compile_context.set_numbers(numbers);


        self.compile_program(self.ast.clone(), &mut compile_context)?;

        Ok(compile_context.code_object)
    }
}

impl<'i> Compiler<'i> for MonaCompilerASTToCode<'i> {
    fn compile_string(&self, node: Rc<RefCell<ASTString<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let code = ByteCodeLoadString {
            index: ctx.get_name_index(&(*node.borrow()).value) as u32,
        };

        ctx.add_code(Box::new(code));

        Ok(())
    }

    fn compile_field_expression(&self, node: Rc<RefCell<ASTFieldExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let node_borrow = node.borrow();

        self.compile_expression(node_borrow.key.clone(), ctx);
        self.compile_expression(node_borrow.expression.clone(), ctx);

        ctx.add_code(Box::new(ByteCodeAccess));

        Ok(())
    }

    fn compile_bool(&self, node: Rc<RefCell<ASTBool<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let value = (*node.borrow()).value;
        ctx.add_code(Box::new(ByteCodeLoadBool {
            value
        }));
        Ok(())
    }

    fn compile_function_call_expression(&self, node: Rc<RefCell<ASTFunctionCallExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        for item in node.borrow().param_list.iter().rev() {
            self.compile_expression(item.clone(), ctx)?;
        }

        self.compile_expression(node.borrow().expression.clone(), ctx)?;

        let code = ByteCodeCall {
            param_count: node.borrow().param_list.len() as u8
        };

        ctx.add_code(Box::new(code));

        Ok(())
    }

    fn compile_identifier_expression(&self, node: Rc<RefCell<ASTIdentifier<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let ident = &node.borrow().ident;
        let index = ctx.get_name_index(ident);
        let code = ByteCodeLoadName {
            index: index as u32,
        };

        ctx.add_code(Box::new(code));

        Ok(())
    }

    fn compile_binary_expression(&self, node: Rc<RefCell<ASTBinaryExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let op = &node.borrow().op;

        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        self.compile_expression(left, ctx);
        self.compile_expression(right, ctx);

        match op.as_str() {
            "+" => ctx.add_code(Box::new(ByteCodeAdd)),
            "*" => ctx.add_code(Box::new(ByteCodeMul)),
            "/" => ctx.add_code(Box::new(ByteCodeDiv)),
            "-" => ctx.add_code(Box::new(ByteCodeSub)),
            "^" => ctx.add_code(Box::new(ByteCodePow)),
            "==" => ctx.add_code(Box::new(ByteCodeEq)),
            "!=" => ctx.add_code(Box::new(ByteCodeNe)),
            "<" => ctx.add_code(Box::new(ByteCodeLt)),
            "<=" => ctx.add_code(Box::new(ByteCodeLe)),
            ">" => ctx.add_code(Box::new(ByteCodeGt)),
            ">=" => ctx.add_code(Box::new(ByteCodeGe)),
            _ => todo!()
        }

        Ok(())
    }

    fn compile_unary_expression(&self, node: Rc<RefCell<ASTUnaryExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let op = &node.borrow().op;

        let expression = node.borrow().expr.clone();

        self.compile_expression(expression, ctx);

        match op.as_str() {
            "-" => ctx.add_code(Box::new(ByteCodeNeg)),
            _ => {
                let e = CompileError::new_no_span(CompileErrorType::InternalFatal, &format!("unrecognize operator `{}`", op));
                return Err(e);
            }
        }

        Ok(())
    }

    fn compile_number(&self, node: Rc<RefCell<ASTNumber<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let index = ctx.get_number_index((*(*node).borrow()).value);
        let code = ByteCodeLoadNumber {
            index: index as u32,
        };

        ctx.add_code(Box::new(code));

        Ok(())
    }

    fn compile_prop_statement(&self, node: Rc<RefCell<ASTPropStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let node_borrow = node.borrow();
        let character_name = &node_borrow.character_name.borrow().ident;
        let prop_name = &node_borrow.prop_name.borrow().ident;
        let var_name = &node_borrow.name.borrow().ident;

        // check character name existence
        let character_name_enum: CharacterName = match CharacterName::from_str(character_name.as_str()) {
            Ok(c) => c,
            Err(_) => {
                let e = CompileError {
                    span: get_my_span(&node.borrow().character_name.borrow().common.span),
                    t: CompileErrorType::CharacterNameNotFound,
                    desc: format!("character name `{}` not found", character_name)
                };

                return Err(e);
            }
        };

        // check prop existence
        if !is_valid_prop_name(prop_name) {
            let e = CompileError {
                span: get_my_span(&node.borrow().prop_name.borrow().common.span),
                t: CompileErrorType::PropNameNotFound,
                desc: format!("prop name `{}` not found", prop_name)
            };
            return Err(e);
        }

        let config_obj = MonaObjectPropConfig {
            character_name: character_name_enum,
            var_name: var_name.clone(),
            prop_name: prop_name.clone()
        };

        if !ctx.add_prop_config(config_obj) {
            let span = get_my_span(&node.borrow().name.borrow().common.span);
            let e = CompileError {
                span,
                t: CompileErrorType::PropNameDup,
                desc: format!("prop name must be unique")
            };
            return Err(e);
        }

        Ok(())
    }

    fn compile_damage_statement(&self, node: Rc<RefCell<ASTDamageStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let node_borrow = node.borrow();
        let character_name = &node_borrow.character_name.borrow().ident;
        let skill_name = &node_borrow.skill_index_name.borrow().ident;
        let var_name = &node_borrow.name.borrow().ident;

        // check character name existence
        let character_name_enum: CharacterName = match CharacterName::from_str(character_name.as_str()) {
            Ok(c) => c,
            Err(_) => {
                let e = CompileError {
                    span: get_my_span(&node.borrow().character_name.borrow().common.span),
                    t: CompileErrorType::CharacterNameNotFound,
                    desc: format!("character name `{}` not found", character_name)
                };

                return Err(e);
            }
        };

        let transformative = skill_name.as_str() == "transformative";
        // check character skill existence
        let character_skill: usize = match character_name_enum.get_skill_from_str(skill_name.as_str()) {
            Some(s) => s,
            None => {
                if transformative {
                    0
                } else {
                    let span = get_my_span(&node.borrow().skill_index_name.borrow().common.span);
                    let e = CompileError {
                        span,
                        t: CompileErrorType::CharacterSkillNotFound,
                        desc: format!("character `{}` skill `{}` not found", character_name, skill_name)
                    };
                    return Err(e);
                }
            }
        };

        let mut fumo: Option<Element> = None;
        let skill_config = match &node.borrow().skill_param {
            None => {
                CharacterSkillConfig::NoConfig
            },
            Some(x) => {
                let mut temp = String::new();
                for (k, v) in x.borrow().items.iter() {
                    if k.borrow().ident == "fumo" {
                        let value_str = if let Some(x) = v.borrow().as_string() {
                            (*x.borrow()).value.clone()
                        } else {
                            let err = CompileError {
                                span: get_my_span(&v.borrow().common.span),
                                desc: format!("fumo have to be a string"),
                                t: CompileErrorType::TypeError
                            };
                            return Err(err);
                        };

                        let element = serde_json::from_str::<Element>(&value_str);
                        fumo = match element {
                            Ok(v) => Some(v),
                            Err(_) => {
                                let err = CompileError {
                                    span: get_my_span(&v.borrow().common.span),
                                    desc: format!("fumo have to be an element, got `{}`", v.borrow().common.input),
                                    t: CompileErrorType::ElementNotFound
                                };
                                return Err(err);
                            }
                        };
                        continue;
                    }

                    let value_str = if let Some(x) = &(*v.borrow()).value {
                        x.to_string()
                    } else {
                        let span = get_my_span(&v.borrow().common.span);
                        let e = CompileError {
                            span,
                            t: CompileErrorType::SkillConfigNotConstant,
                            desc: String::from("skill config value not constant")
                        };
                        return Err(e);
                    };

                    temp.push_str(&format!("\"{}\": {},", k.borrow().ident, value_str));
                }

                if temp.len() == 0 {
                    CharacterSkillConfig::NoConfig
                } else {
                    temp.remove(temp.len() - 1);
                    let config_json = format!("{{ \"{name}\": {{ {config} }} }}", name=character_name, config=temp);
                    // println!("{}", config_json);

                    match serde_json::from_str::<CharacterSkillConfig>(&config_json) {
                        Ok(v) => v,
                        Err(_) => {
                            let span = get_my_span(&node.borrow().common.span);
                            let e = CompileError {
                                span,
                                t: CompileErrorType::SkillConfigNotExist,
                                desc: String::from("skill config not exist")
                            };
                            return Err(e);
                        }
                    }
                }
            },
        };

        let config_obj = MonaObjectDamageConfig {
            character_name: character_name_enum,
            skill_index: character_skill,
            skill_config,
            var_name: var_name.clone(),
            is_transformative: transformative,
            fumo,
        };

        if !ctx.add_damage_config(config_obj) {
            let span = get_my_span(&node_borrow.name.borrow().common.span);
            let e = CompileError {
                span,
                t: CompileErrorType::DamageNameDuplicate,
                desc: String::from("damage name must be unique")
            };
            return Err(e);
        }

        Ok(())
    }

    fn compile_expression_statement(&self, node: Rc<RefCell<ASTExpressionStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        self.compile_expression(node.borrow().expression.clone(), ctx)
    }

    fn compile_assignment_statement(&self, node: Rc<RefCell<ASTAssignmentStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let expression = node.borrow().expression.clone();

        self.compile_expression(expression, ctx);

        let left_value = &node.borrow().left_value_name;
        let name = &left_value.borrow().ident;
        let code = ByteCodeSaveName {
            name_index: ctx.get_name_index(name) as u32,
        };

        ctx.add_code(Box::new(code));

        Ok(())
    }
}
