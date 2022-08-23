use std::cell::RefCell;
use std::rc::Rc;
use pest::Span;
use crate::ast::node::NodeCommon;

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div
}

pub enum UnaryOperator {
    Neg
}

#[derive(Clone)]
pub enum ConstValueType {
    Number(f64),
    Bool(bool),
    String(String),
}

impl ConstValueType {
    pub fn to_string(&self) -> String {
        match *self {
            ConstValueType::Number(x) => x.to_string(),
            ConstValueType::Bool(x) => x.to_string(),
            ConstValueType::String(ref x) => x.clone(),
        }
    }

    pub fn is_number(&self) -> bool {
        if let ConstValueType::Number(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let ConstValueType::Bool(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn get_number(&self) -> f64 {
        if let ConstValueType::Number(x) = *self {
            x
        } else {
            panic!("value is not number")
        }
    }

    pub fn get_bool(&self) -> bool {
        if let ConstValueType::Bool(x) = *self { x } else { panic!("value is not bool") }
    }
}

pub struct ASTObjectLiteral<'i> {
    pub common: NodeCommon<'i>,
    pub items: Vec<(Rc<RefCell<ASTIdentifier<'i>>>, Rc<RefCell<ASTExpression<'i>>>)>
}

impl<'i> ASTObjectLiteral<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: None,
            expression: ExpressionEnum::Object(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTArrayLiteral<'i> {
    pub common: NodeCommon<'i>,
    pub items: Vec<Rc<RefCell<ASTExpression<'i>>>>
}

pub struct ASTBool<'i> {
    pub common: NodeCommon<'i>,
    pub value: bool,
}

impl<'i> ASTBool<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: Some(ConstValueType::Bool(self.value)),
            expression: ExpressionEnum::Bool(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTString<'i> {
    pub common: NodeCommon<'i>,
    pub value: String,
}

impl<'i> ASTString<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: Some(ConstValueType::String(self.value.clone())),
            expression: ExpressionEnum::String(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTNumber<'i> {
    pub common: NodeCommon<'i>,
    pub value: f64,
}

impl<'i> ASTNumber<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: Some(ConstValueType::Number(self.value)),
            expression: ExpressionEnum::Number(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTIdentifier<'i> {
    pub common: NodeCommon<'i>,
    pub ident: String,
}

impl<'i> ASTIdentifier<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: None,
            expression: ExpressionEnum::Identifier(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTFieldExpression<'i> {
    pub expression: Rc<RefCell<ASTExpression<'i>>>,
    pub key: Rc<RefCell<ASTExpression<'i>>>,
    pub common: NodeCommon<'i>,
    pub is_dot: bool,
}

impl<'i> ASTFieldExpression<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: None,
            expression: ExpressionEnum::FieldExpression(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTFunctionCallExpression<'i> {
    pub expression: Rc<RefCell<ASTExpression<'i>>>,
    pub param_list: Vec<Rc<RefCell<ASTExpression<'i>>>>,
    pub common: NodeCommon<'i>,
}

impl<'i> ASTFunctionCallExpression<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: None,
            expression: ExpressionEnum::FunctionCall(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTBinaryExpression<'i> {
    pub left: Rc<RefCell<ASTExpression<'i>>>,
    pub right: Rc<RefCell<ASTExpression<'i>>>,
    pub op: String,
    pub common: NodeCommon<'i>,
    pub value: Option<ConstValueType>,
}

impl<'i> ASTBinaryExpression<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: self.value.clone(),
            expression: ExpressionEnum::BinaryExpression(Rc::new(RefCell::new(self)))
        }
    }
}

pub struct ASTUnaryExpression<'i> {
    pub expr: Rc<RefCell<ASTExpression<'i>>>,
    pub op: String,
    pub common: NodeCommon<'i>,
    pub value: Option<ConstValueType>,
}

impl<'i> ASTUnaryExpression<'i> {
    pub fn wrap_expression(self) -> ASTExpression<'i> {
        ASTExpression {
            common: self.common.clone(),
            value: self.value.clone(),
            expression: ExpressionEnum::UnaryExpression(Rc::new(RefCell::new(self)))
        }
    }
}

pub enum ExpressionEnum<'i> {
    BinaryExpression(Rc<RefCell<ASTBinaryExpression<'i>>>),
    UnaryExpression(Rc<RefCell<ASTUnaryExpression<'i>>>),
    Bool(Rc<RefCell<ASTBool<'i>>>),
    Number(Rc<RefCell<ASTNumber<'i>>>),
    Identifier(Rc<RefCell<ASTIdentifier<'i>>>),
    Object(Rc<RefCell<ASTObjectLiteral<'i>>>),
    Array(Rc<RefCell<ASTArrayLiteral<'i>>>),
    FieldExpression(Rc<RefCell<ASTFieldExpression<'i>>>),
    FunctionCall(Rc<RefCell<ASTFunctionCallExpression<'i>>>),
    String(Rc<RefCell<ASTString<'i>>>),
}

pub struct ASTExpression<'i> {
    pub common: NodeCommon<'i>,
    pub value: Option<ConstValueType>,
    pub expression: ExpressionEnum<'i>
}

impl<'i> ASTExpression<'i> {
    pub fn as_object_literal(&self) -> Option<Rc<RefCell<ASTObjectLiteral<'i>>>> {
        match &self.expression {
            ExpressionEnum::Object(o) => Some(o.clone()),
            _ => None
        }
    }

    pub fn as_identifier(&self) -> Option<Rc<RefCell<ASTIdentifier<'i>>>> {
        match &self.expression {
            ExpressionEnum::Identifier(i) => Some(i.clone()),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<Rc<RefCell<ASTString<'i>>>> {
        match &self.expression {
            ExpressionEnum::String(s) => Some(s.clone()),
            _ => None
        }
    }
}