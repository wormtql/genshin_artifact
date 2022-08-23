use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::expression::expression::{ASTBinaryExpression, ASTBool, ASTExpression, ASTFieldExpression, ASTFunctionCallExpression, ASTIdentifier, ASTNumber, ASTString, ASTUnaryExpression, ExpressionEnum};
use crate::ast::program::ASTProgram;
use crate::ast::statement::{ASTAssignmentStatement, ASTDamageStatement, ASTExpressionStatement, ASTLeftValueFieldAccessItem, ASTPropStatement, ASTStatement, StatementEnum};

pub trait ASTTraverser<'i> {
    fn enter_program(&mut self, node: Rc<RefCell<ASTProgram<'i>>>) {}
    fn leave_program(&mut self, node: Rc<RefCell<ASTProgram<'i>>>) {}

    fn enter_identifier(&mut self, node: Rc<RefCell<ASTIdentifier<'i>>>) {}
    fn leave_identifier(&mut self, node: Rc<RefCell<ASTIdentifier<'i>>>) {}

    fn enter_number(&mut self, node: Rc<RefCell<ASTNumber<'i>>>) {}
    fn leave_number(&mut self, node: Rc<RefCell<ASTNumber<'i>>>) {}

    fn enter_string(&mut self, node: Rc<RefCell<ASTString<'i>>>) {}
    fn leave_string(&mut self, node: Rc<RefCell<ASTString<'i>>>) {}
}

pub struct ASTTraverse<'i, T> {
    pub traverser: &'i mut T,
}

impl<'i, 'a, T: ASTTraverser<'i>> ASTTraverse<'a, T> {
    pub fn traverse_program(&mut self, node: Rc<RefCell<ASTProgram<'i>>>) {
        self.traverser.enter_program(node.clone());
        for stat in node.borrow().statements.iter() {
            self.traverse_statement(stat.clone());
        }
        self.traverser.leave_program(node.clone());
    }

    pub fn traverse_statement(&mut self, node: Rc<RefCell<ASTStatement<'i>>>) {
        match &node.borrow().statement {
            StatementEnum::AssignmentStatement(x) => self.traverse_assignment_statement(x.clone()),
            StatementEnum::DamageStatement(x) => self.traverse_damage_statement(x.clone()),
            StatementEnum::ExpressionStatement(x) => self.traverse_expression_statement(x.clone()),
            StatementEnum::PropStatement(x) => self.traverse_prop_statement(x.clone()),
        }
    }

    pub fn traverse_expression_statement(&mut self, node: Rc<RefCell<ASTExpressionStatement<'i>>>) {
        self.traverse_expression(node.borrow().expression.clone());
    }

    pub fn traverse_damage_statement(&mut self, node: Rc<RefCell<ASTDamageStatement<'i>>>) {
        // todo!()
    }

    pub fn traverse_prop_statement(&mut self, node: Rc<RefCell<ASTPropStatement<'i>>>) {
        // todo!()
    }

    pub fn traverse_assignment_statement(&mut self, node: Rc<RefCell<ASTAssignmentStatement<'i>>>) {
        self.traverse_identifier(node.borrow().left_value_name.clone());
        self.traverse_expression(node.borrow().expression.clone());
    }

    pub fn traverse_left_value_field_access_item(&mut self, node: Rc<RefCell<ASTLeftValueFieldAccessItem<'i>>>) {
        self.traverse_expression(node.borrow().expression.clone());
    }

    pub fn traverse_expression(&mut self, node: Rc<RefCell<ASTExpression<'i>>>) {
        match &node.borrow().expression {
            ExpressionEnum::Identifier(x) => self.traverse_identifier(x.clone()),
            ExpressionEnum::FieldExpression(x) => self.traverse_field_expression(x.clone()),
            ExpressionEnum::BinaryExpression(x) => self.traverse_binary_expression(x.clone()),
            ExpressionEnum::Number(x) => self.traverse_number(x.clone()),
            ExpressionEnum::FunctionCall(x) => self.traverse_function_call_expression(x.clone()),
            ExpressionEnum::Bool(x) => self.traverse_bool(x.clone()),
            ExpressionEnum::String(x) => self.traverse_string(x.clone()),
            ExpressionEnum::UnaryExpression(x) => self.traverse_unary_expression(x.clone()),
            _ => todo!()
        }
    }

    pub fn traverse_unary_expression(&mut self, node: Rc<RefCell<ASTUnaryExpression<'i>>>) {
        self.traverse_expression(node.borrow().expr.clone());
    }

    pub fn traverse_field_expression(&mut self, node: Rc<RefCell<ASTFieldExpression<'i>>>) {
        self.traverse_expression(node.borrow().expression.clone());

        // if node.borrow().is_dot {
        //
        // } else {
            self.traverse_expression(node.borrow().key.clone());
        // }
    }

    pub fn traverse_string(&mut self, node: Rc<RefCell<ASTString<'i>>>) {
        self.traverser.enter_string(node.clone());
        self.traverser.leave_string(node);
    }

    pub fn traverse_bool(&mut self, node: Rc<RefCell<ASTBool<'i>>>) {
        // todo
    }

    pub fn traverse_function_call_expression(&mut self, node: Rc<RefCell<ASTFunctionCallExpression<'i>>>) {
        self.traverse_expression(node.borrow().expression.clone());

        for param in node.borrow().param_list.iter() {
            self.traverse_expression(param.clone());
        }
    }

    pub fn traverse_number(&mut self, node: Rc<RefCell<ASTNumber<'i>>>) {
        self.traverser.enter_number(node.clone());
        self.traverser.leave_number(node);
    }

    pub fn traverse_binary_expression(&mut self, node: Rc<RefCell<ASTBinaryExpression<'i>>>) {
        self.traverse_expression(node.borrow().left.clone());
        self.traverse_expression(node.borrow().right.clone());
    }

    pub fn traverse_identifier(&mut self, node: Rc<RefCell<ASTIdentifier<'i>>>) {
        self.traverser.enter_identifier(node.clone());
        self.traverser.leave_identifier(node);
    }
}