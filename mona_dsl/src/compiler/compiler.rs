use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::ast::expression::expression::{ASTBinaryExpression, ASTBool, ASTExpression, ASTFieldExpression, ASTFunctionCallExpression, ASTIdentifier, ASTNumber, ASTString, ASTUnaryExpression, ExpressionEnum};
use crate::ast::program::ASTProgram;
use crate::ast::statement::{ASTAssignmentStatement, ASTDamageStatement, ASTExpressionStatement, ASTPropStatement, ASTStatement, StatementEnum};
use crate::ast::traverse::{ASTTraverse, ASTTraverser};
use crate::code::byte_code::ByteCode;
use crate::error::CompileError;
use crate::object::damage_config::MonaObjectDamageConfig;
use crate::object::prop::MonaObjectPropConfig;
use crate::parser::pest::parse_to_cst;
use crate::parser::to_ast::ToAST;


pub struct CodeObject {
    pub names: Vec<String>,
    pub numbers: Vec<f64>,
    pub damage_configs: Vec<MonaObjectDamageConfig>,
    pub prop_configs: Vec<MonaObjectPropConfig>,
    pub codes: Vec<Box<dyn ByteCode>>
}

impl Display for CodeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for code in self.codes.iter() {
            write!(f, "{}\n", code.to_string(self))?
        }
        Ok(())
    }
}

pub struct CompileContext {
    pub code_object: CodeObject,
    name_index_map: HashMap<String, usize>,
    number_index_map: HashMap<u64, usize>,
    damage_config_index_map: HashMap<String, usize>,
    prop_config_index_map: HashMap<String, usize>,
}

impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            code_object: CodeObject {
                names: Vec::new(),
                numbers: Vec::new(),
                codes: Vec::new(),
                damage_configs: Vec::new(),
                prop_configs: Vec::new(),
            },
            name_index_map: HashMap::new(),
            number_index_map: HashMap::new(),
            damage_config_index_map: HashMap::new(),
            prop_config_index_map: HashMap::new(),
        }
    }

    pub fn add_code(&mut self, code: Box<dyn ByteCode>) {
        self.code_object.codes.push(code);
    }

    /// add damage config constant
    pub fn add_damage_config(&mut self, config: MonaObjectDamageConfig) -> bool {
        let var_name = config.var_name.clone();
        if self.damage_config_index_map.contains_key(&var_name) {
            return false;
        }
        self.code_object.damage_configs.push(config);
        let index = self.code_object.damage_configs.len() - 1;

        self.damage_config_index_map.insert(var_name, index);

        true
    }

    /// add prop config constant
    pub fn add_prop_config(&mut self, config: MonaObjectPropConfig) -> bool {
        let var_name = config.var_name.clone();
        if self.prop_config_index_map.contains_key(&var_name) {
            return false;
        }
        self.code_object.prop_configs.push(config);
        let index = self.code_object.prop_configs.len() - 1;
        self.prop_config_index_map.insert(var_name, index);

        true
    }

    /// add name/string constant
    pub fn add_name_const(&mut self, s: &str) {
        let s = String::from(s);
        if self.name_index_map.contains_key(&s) {
            return;
        }

        self.code_object.names.push(s.clone());
        let index = self.code_object.names.len() - 1;
        self.name_index_map.insert(s, index);
    }

    // pub fn set_names(&mut self, names: Vec<String>) {
    //     self.name_index_map.clear();
    //     for (index, item) in names.iter().enumerate() {
    //         self.name_index_map.insert(item.clone(), index);
    //     }
    //     self.code_object.names = names;
    // }

    pub fn set_numbers(&mut self, numbers: Vec<f64>) {
        self.number_index_map.clear();
        for (index, &item) in numbers.iter().enumerate() {
            let bits = item.to_bits();
            self.number_index_map.insert(bits, index);
        }
        self.code_object.numbers = numbers;
    }

    pub fn get_name_index(&self, name: &str) -> usize {
        *self.name_index_map.get(name).unwrap()
    }

    pub fn get_number_index(&self, value: f64) -> usize {
        let bits = value.to_bits();
        *self.number_index_map.get(&bits).unwrap()
    }
}

pub trait Compiler<'i> {
    fn compile_expression(&self, node: Rc<RefCell<ASTExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        match &node.borrow().expression {
            ExpressionEnum::BinaryExpression(x) => self.compile_binary_expression(x.clone(), ctx)?,
            ExpressionEnum::Number(x) => self.compile_number(x.clone(), ctx)?,
            ExpressionEnum::Identifier(x) => self.compile_identifier_expression(x.clone(), ctx)?,
            ExpressionEnum::UnaryExpression(x) => self.compile_unary_expression(x.clone(), ctx)?,
            ExpressionEnum::FunctionCall(x) => self.compile_function_call_expression(x.clone(), ctx)?,
            ExpressionEnum::Bool(x) => self.compile_bool(x.clone(), ctx)?,
            ExpressionEnum::FieldExpression(x) => self.compile_field_expression(x.clone(), ctx)?,
            ExpressionEnum::String(x) => self.compile_string(x.clone(), ctx)?,
            _ => todo!()
        }

        Ok(())
    }

    fn compile_string(&self, node: Rc<RefCell<ASTString<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_field_expression(&self, node: Rc<RefCell<ASTFieldExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_bool(&self, node: Rc<RefCell<ASTBool<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_function_call_expression(&self, node: Rc<RefCell<ASTFunctionCallExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_identifier_expression(&self, node: Rc<RefCell<ASTIdentifier<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_binary_expression(&self, node: Rc<RefCell<ASTBinaryExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_unary_expression(&self, node: Rc<RefCell<ASTUnaryExpression<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_number(&self, node: Rc<RefCell<ASTNumber<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_statement(&self, node: Rc<RefCell<ASTStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        match &node.borrow().statement {
            StatementEnum::AssignmentStatement(x) => self.compile_assignment_statement(x.clone(), ctx)?,
            StatementEnum::ExpressionStatement(x) => self.compile_expression_statement(x.clone(), ctx)?,
            StatementEnum::DamageStatement(x) => self.compile_damage_statement(x.clone(), ctx)?,
            StatementEnum::PropStatement(x) => self.compile_prop_statement(x.clone(), ctx)?,
            _ => todo!()
        }

        Ok(())
    }

    fn compile_prop_statement(&self, node: Rc<RefCell<ASTPropStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_damage_statement(&self, node: Rc<RefCell<ASTDamageStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_expression_statement(&self, node: Rc<RefCell<ASTExpressionStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_assignment_statement(&self, node: Rc<RefCell<ASTAssignmentStatement<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError>;

    fn compile_program(&self, node: Rc<RefCell<ASTProgram<'i>>>, ctx: &mut CompileContext) -> Result<(), CompileError> {
        for item in node.borrow().statements.iter() {
            self.compile_statement(item.clone(), ctx)?;
        }

        Ok(())
    }
}
