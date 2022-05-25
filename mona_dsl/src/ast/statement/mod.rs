use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::expression::expression::{ASTExpression, ASTIdentifier, ASTObjectLiteral};
use crate::ast::node::NodeCommon;

pub struct ASTLeftValueFieldAccessItem<'i> {
    pub common: NodeCommon<'i>,
    pub expression: Rc<RefCell<ASTExpression<'i>>>
}

pub struct ASTAssignmentStatement<'i> {
    pub common: NodeCommon<'i>,
    pub left_value_name: Rc<RefCell<ASTIdentifier<'i>>>,
    pub left_value_field_access_list: Vec<Rc<RefCell<ASTLeftValueFieldAccessItem<'i>>>>,
    pub expression: Rc<RefCell<ASTExpression<'i>>>
}

impl<'i> ASTAssignmentStatement<'i> {
    pub fn wrap_statement(self) -> ASTStatement<'i> {
        ASTStatement {
            common: self.common.clone(),
            statement: StatementEnum::AssignmentStatement(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTDamageStatement<'i> {
    pub common: NodeCommon<'i>,
    pub name: Rc<RefCell<ASTIdentifier<'i>>>,
    pub character_name: Rc<RefCell<ASTIdentifier<'i>>>,
    pub skill_index_name: Rc<RefCell<ASTIdentifier<'i>>>,
    pub skill_param: Option<Rc<RefCell<ASTObjectLiteral<'i>>>>,
}

impl<'i> ASTDamageStatement<'i> {
    pub fn wrap_statement(self) -> ASTStatement<'i> {
        ASTStatement {
            common: self.common.clone(),
            statement: StatementEnum::DamageStatement(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTPropStatement<'i> {
    pub common: NodeCommon<'i>,
    pub name: Rc<RefCell<ASTIdentifier<'i>>>,
    pub character_name: Rc<RefCell<ASTIdentifier<'i>>>,
    pub prop_name: Rc<RefCell<ASTIdentifier<'i>>>,
}

impl<'i> ASTPropStatement<'i> {
    pub fn wrap_statement(self) -> ASTStatement<'i> {
        ASTStatement {
            common: self.common.clone(),
            statement: StatementEnum::PropStatement(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTExpressionStatement<'i> {
    pub common: NodeCommon<'i>,
    pub expression: Rc<RefCell<ASTExpression<'i>>>
}

impl<'i> ASTExpressionStatement<'i> {
    pub fn wrap_statement(self) -> ASTStatement<'i> {
        ASTStatement {
            common: self.common.clone(),
            statement: StatementEnum::ExpressionStatement(Rc::new(RefCell::new(self)))
        }
    }
}

pub enum StatementEnum<'i> {
    AssignmentStatement(Rc<RefCell<ASTAssignmentStatement<'i>>>),
    DamageStatement(Rc<RefCell<ASTDamageStatement<'i>>>),
    ExpressionStatement(Rc<RefCell<ASTExpressionStatement<'i>>>),
    PropStatement(Rc<RefCell<ASTPropStatement<'i>>>),
}

pub struct ASTStatement<'i> {
    pub common: NodeCommon<'i>,
    pub statement: StatementEnum<'i>
}

impl<'i> ASTStatement<'i> {
    pub fn as_assignment_statement(&self) -> Rc<RefCell<ASTAssignmentStatement<'i>>> {
        match &self.statement {
            StatementEnum::AssignmentStatement(x) => x.clone(),
            _ => panic!("cannot convert to assignment statement")
        }
    }

    pub fn as_damage_statement(&self) -> Rc<RefCell<ASTDamageStatement<'i>>> {
        match &self.statement {
            StatementEnum::DamageStatement(x) => x.clone(),
            _ => panic!("cannot convert to damage statement")
        }
    }
}
