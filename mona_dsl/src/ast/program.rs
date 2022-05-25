use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::node::NodeCommon;
use crate::ast::statement::ASTStatement;

pub struct ASTProgram<'i> {
    pub common: NodeCommon<'i>,
    pub statements: Vec<Rc<RefCell<ASTStatement<'i>>>>
}
