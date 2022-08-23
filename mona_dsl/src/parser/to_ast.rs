use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use pest::iterators::Pair;
use pest::Span;
use crate::ast::expression::expression::{ASTBinaryExpression, ASTBool, ASTExpression, ASTFieldExpression, ASTFunctionCallExpression, ASTIdentifier, ASTNumber, ASTObjectLiteral, ASTString, ASTUnaryExpression, ConstValueType, ExpressionEnum};
use crate::ast::node::NodeCommon;
use crate::ast::program::ASTProgram;
use crate::ast::statement::{ASTAssignmentStatement, ASTDamageStatement, ASTExpressionStatement, ASTLeftValueFieldAccessItem, ASTPropStatement, ASTStatement};
use crate::error::{CompileError, CompileErrorType};
use crate::object::mona_object::MonaObject;
use crate::parser::pest::MonaRule;

pub struct ToAST<'i> {
    pub input: &'i str,
}

fn convert_my_span(span: &Span) -> crate::common::Span {
    let (a, b) = span.start_pos().line_col();
    let (c, d) = span.end_pos().line_col();

    crate::common::Span {
        start_row: a,
        start_col: b,
        end_row: c,
        end_col: d
    }
}

type WrapperStatement<'i> = Rc<RefCell<ASTStatement<'i>>>;
type WrapperExpression<'i> = Rc<RefCell<ASTExpression<'i>>>;
fn merge_value_binary<'i>(a: WrapperExpression<'i>, b: WrapperExpression<'i>, op: &str) -> Result<WrapperExpression<'i>, CompileError> {
    let v1 = &(*(*a).borrow()).value;
    let v2 = &(*(*b).borrow()).value;

    if op == "/" && v2.is_some() && v2.as_ref().unwrap().is_number() && v2.as_ref().unwrap().get_number() == 0.0 {
        let (line, col) = (*b).borrow().common.span.start_pos().line_col();
        return Err(CompileError::new_line_col(line, col, "divide by zero", CompileErrorType::DivByZero));
    }

    let v = if v1.is_none() || v2.is_none() {
        None
    } else {
        let x = v1.as_ref().unwrap();
        let y = v2.as_ref().unwrap();

        if x.is_number() && y.is_number() {
            let x = x.get_number();
            let y = y.get_number();
            let result = match op {
                "+" => x + y,
                "-" => x - y,
                "*" => x * y,
                "/" => x / y,
                "^" => x.powf(y),
                "<" => if x < y { 1.0 } else { 0.0 },
                "<=" => if x <= y { 1.0 } else { 0.0 },
                ">" => if x > y { 1.0 } else { 0.0 },
                ">=" => if x >= y { 1.0 } else { 0.0 },
                "==" => if x == y { 1.0 } else { 0.0 },
                "!=" => if x != y { 1.0 } else { 0.0 },
                "&&" | "||" => {
                    return Err(
                        CompileError::new_line_col(0, 0, "expecting bool", CompileErrorType::TypeError)
                    );
                },
                _ => return Err(CompileError::new_line_col(0, 0, &format!("unrecognized operator {}", op), CompileErrorType::InternalFatal)),
            };
            Some(ConstValueType::Number(result))
        } else if x.is_bool() && y.is_bool() {
            let x = x.get_bool();
            let y = y.get_bool();
            let result = match op {
                "&&" => x && y,
                "||" => x || y,
                "==" => x == y,
                "!=" => x != y,
                _ => return Err(CompileError::new_line_col(0, 0, "expecting number", CompileErrorType::TypeError)),
            };
            Some(ConstValueType::Bool(result))
        } else {
            return Err(CompileError::new_line_col(0, 0, "type not compatible", CompileErrorType::TypeError));
        }
    };

    let merge_common = (*a).borrow().common.merge_span(&(*b).borrow().common);
    let expression = ASTBinaryExpression {
        left: a.clone(),
        right: b.clone(),
        op: String::from(op),
        common: merge_common,
        value: v
    }.wrap_expression();

    Ok(Rc::new(RefCell::new(expression)))
}

impl<'i> ToAST<'i> {
    pub fn convert_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let rule = pair.as_rule();
        match rule {
            // MonaRule::additive_expression => self.convert_additive_expression(pair),
            MonaRule::additive_expression => self.convert_binary_expression_left_to_right(pair),
            // MonaRule::multiplicative_expression => self.convert_multiplicative_expression(pair),
            MonaRule::multiplicative_expression => self.convert_binary_expression_left_to_right(pair),
            MonaRule::relational_expression => self.convert_binary_expression_left_to_right(pair),
            MonaRule::equality_expression => self.convert_binary_expression_left_to_right(pair),
            MonaRule::logical_and_expression => self.convert_binary_expression_left_to_right(pair),
            MonaRule::logical_or_expression => self.convert_binary_expression_left_to_right(pair),
            MonaRule::power_expression => self.convert_power_expression(pair),
            MonaRule::unary_expression => self.convert_unary_expression(pair),
            MonaRule::number => self.convert_number(pair),
            MonaRule::expression => self.convert_expression(pair.into_inner().next().unwrap()),
            MonaRule::primary_expression => self.convert_primary_expression(pair),
            MonaRule::paren_expression => self.convert_expression(pair.into_inner().next().unwrap()),
            MonaRule::identifier => self.convert_identifier(pair),
            MonaRule::field_expression => self.convert_field_expression(pair),
            MonaRule::object_literal => self.convert_object_literal(pair),
            MonaRule::bool_false | MonaRule::bool_true => Ok(self.convert_bool(pair)),
            MonaRule::string_literal => Ok(self.convert_string_literal(pair)),
            _ => {
                println!("todo rule: {:?}", rule);
                todo!()
            }
        }
    }

    pub fn convert_string_literal(&self, pair: Pair<'i, MonaRule>) -> WrapperExpression<'i> {
        let node = ASTString {
            common: NodeCommon {
                input: self.input,
                span: pair.as_span(),
            },
            value: pair.as_str().to_string()
        }.wrap_expression();

        Rc::new(RefCell::new(node))
    }

    pub fn convert_bool(&self, pair: Pair<'i, MonaRule>) -> WrapperExpression<'i> {
        let node = ASTBool {
            common: NodeCommon {
                input: self.input,
                span: pair.as_span(),
            },
            value: pair.as_str() == "true"
        }.wrap_expression();

        Rc::new(RefCell::new(node))
    }

    pub fn convert_function_call_param_list(&self, pair: Pair<'i, MonaRule>) -> Result<Vec<WrapperExpression<'i>>, CompileError> {
        if let MonaRule::function_call_item = pair.as_rule() {
            let mut temp: Vec<_> = pair.into_inner().collect();
            if temp.len() == 0 {
                return Ok(Vec::new());
            } else {
                return self.convert_function_call_param_list(temp.pop().unwrap());
            }
        }

        let mut result = Vec::new();
        for expression_pair in pair.into_inner() {
            result.push(self.convert_expression(expression_pair)?);
        }

        Ok(result)
    }

    pub fn convert_field_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let mut temp: Vec<_> = pair.into_inner().rev().collect();

        let first_node = self.convert_expression(temp.pop().unwrap())?;
        let mut node = first_node;

        while temp.len() > 0 {
            let second = temp.pop().unwrap();
            let s = second.as_str();

            let new_common = (*node).borrow().common.merge_span2(&second.as_span());

            let flag1 = s.starts_with(".");
            let flag2 = s.starts_with("[");
            if flag1 || flag2 {
                // is a field access

                let key = if flag1 {
                    let ident_pair = second.into_inner().next().unwrap();
                    let n = ASTString {
                        common: NodeCommon {
                            input: self.input,
                            span: ident_pair.as_span(),
                        },
                        value: ident_pair.as_str().to_string()
                    }.wrap_expression();
                    Rc::new(RefCell::new(n))
                } else {
                    self.convert_expression(second.into_inner().next().unwrap())?
                };

                node = Rc::new(RefCell::new(ASTFieldExpression {
                    expression: node,
                    key,
                    common: new_common,
                    is_dot: s.starts_with(".")
                }.wrap_expression()));
            } else if s.starts_with("(") {
                // is a function call
                let param_list = self.convert_function_call_param_list(second.into_inner().next().unwrap())?;

                node = Rc::new(RefCell::new(ASTFunctionCallExpression {
                    expression: node,
                    param_list,
                    common: new_common
                }.wrap_expression()));
            }
        }

        Ok(node)
    }

    pub fn convert_binary_expression_left_to_right(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let mut temp: Vec<_> = pair.into_inner().rev().collect();

        let first_node = self.convert_expression(temp.pop().unwrap())?;
        let mut node = first_node;

        while temp.len() > 0 {
            let op = temp.pop().unwrap().as_str();
            let second_node = self.convert_expression(temp.pop().unwrap())?;

            node = merge_value_binary(node, second_node, op)?;
        }

        Ok(node)
    }

    // pub fn convert_additive_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
    //     let mut temp: Vec<_> = pair.into_inner().rev().collect();
    //
    //     let first_node = self.convert_expression(temp.pop().unwrap())?;
    //     let mut node = first_node;
    //
    //     while temp.len() > 0 {
    //         let op = temp.pop().unwrap().as_str();
    //         let second_node = self.convert_expression(temp.pop().unwrap())?;
    //
    //         node = merge_value_binary(node, second_node, op)?;
    //     }
    //
    //     Ok(node)
    // }
    //
    // pub fn convert_multiplicative_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
    //     let mut temp: Vec<_> = pair.into_inner().rev().collect();
    //
    //     let first_node = self.convert_expression(temp.pop().unwrap())?;
    //     let mut node = first_node;
    //
    //     while temp.len() > 0 {
    //         let op = temp.pop().unwrap().as_str();
    //         // println!("{}", op);
    //         let second_node = self.convert_expression(temp.pop().unwrap())?;
    //
    //         node = merge_value_binary(node, second_node, op)?;
    //     }
    //
    //     Ok(node)
    // }

    pub fn convert_power_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let mut temp: Vec<_> = pair.into_inner().collect();

        let first_node = self.convert_expression(temp.pop().unwrap())?;
        let mut node = first_node;

        while temp.len() > 0 {
            let op = temp.pop().unwrap().as_str();
            let second_node = self.convert_expression(temp.pop().unwrap())?;

            node = merge_value_binary(second_node, node, op)?;
        }

        Ok(node)
    }

    pub fn convert_unary_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let mut temp: Vec<_> = pair.into_inner().collect();
        if temp.len() == 1 {
            self.convert_expression(temp.pop().unwrap())
        } else if temp.len() == 2 {
            let exp_pair = temp.pop().unwrap();
            let exp_span = exp_pair.as_span();
            let expression = self.convert_expression(exp_pair)?;
            let op_pair = temp.pop().unwrap();
            let op = op_pair.as_str();

            let old_value = &(*(*expression).borrow()).value;
            let new_value = if let Some(x) = old_value {
                if let ConstValueType::Number(n) = x {
                    Some(ConstValueType::Number(-*n))
                } else {
                    let e = CompileError {
                        span: convert_my_span(&exp_span),
                        t: CompileErrorType::TypeError,
                        desc: format!("unary `-` can only be used with number")
                    };
                    return Err(e);
                }
            } else {
                None
            };

            let new_span = (*expression).borrow().common.merge_span2(&op_pair.as_span());

            let new_expression = ASTUnaryExpression {
                expr: expression.clone(),
                op: String::from(op),
                common: new_span,
                value: new_value
            }.wrap_expression();

            Ok(Rc::new(RefCell::new(new_expression)))
        } else {
            panic!("cannot happen")
        }
    }

    pub fn convert_primary_expression(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let sub = pair.into_inner().next().unwrap();
        self.convert_expression(sub)
    }

    pub fn convert_number(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let value = pair.as_str().parse::<f64>().unwrap();

        let common = NodeCommon {
            input: self.input,
            span: pair.as_span()
        };

        let node = ASTNumber {
            common,
            value
        }.wrap_expression();

        Ok(Rc::new(RefCell::new(node)))
    }

    pub fn convert_identifier(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let ident = String::from(pair.as_str());

        let common = NodeCommon {
            input: self.input,
            span: pair.as_span()
        };

        let node = ASTIdentifier {
            common,
            ident
        }.wrap_expression();

        Ok(Rc::new(RefCell::new(node)))
    }

    pub fn convert_object_literal(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperExpression<'i>, CompileError> {
        let span = pair.as_span();
        let object_item_list_pair = pair.into_inner().next().unwrap();
        let temp = object_item_list_pair.into_inner().next();

        let mut result = Vec::new();
        if let Some(x) = temp {
            for item_pair in x.into_inner() {
                let mut iter = item_pair.into_inner();
                let identifier_pair = iter.next().unwrap();
                let expression_pair = iter.next().unwrap();

                let ast_identifier = ASTIdentifier {
                    common: NodeCommon {
                        input: self.input,
                        span: identifier_pair.as_span(),
                    },
                    ident: String::from(identifier_pair.as_str())
                };

                let expression = self.convert_expression(expression_pair)?;

                result.push((Rc::new(RefCell::new(ast_identifier)), expression));
            }
        }

        let ast_object_literal = ASTObjectLiteral {
            common: NodeCommon {
                input: self.input,
                span,
            },
            items: result
        }.wrap_expression();

        Ok(Rc::new(RefCell::new(ast_object_literal)))
    }

    pub fn convert_statement(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperStatement<'i>, CompileError> {
        let rule = pair.as_rule();

        match rule {
            MonaRule::assignment_statement => self.convert_assignment_statement(pair),
            MonaRule::damage_declaration => self.convert_damage_statement(pair),
            MonaRule::expression_statement => self.convert_expression_statement(pair),
            MonaRule::statement => self.convert_statement(pair.into_inner().next().unwrap()),
            MonaRule::prop_declaration => self.convert_prop_statement(pair),
            _ => {
                println!("unimplemented rule: {:?}", rule);
                todo!()
            }
        }
    }

    pub fn convert_assignment_statement(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperStatement<'i>, CompileError> {
        let span = pair.as_span();
        let mut temp: Vec<_> = pair.into_inner().collect();

        let expression = self.convert_expression(temp.pop().unwrap())?;

        let left_value_pair = temp.pop().unwrap();
        let mut left_value_pair_list: Vec<_> = left_value_pair.into_inner().rev().collect();

        let identifier = self.convert_expression(left_value_pair_list.pop().unwrap())?;
        // let i = unsafe { *identifier.as_ptr() };
        let identifier = match unsafe { Rc::try_unwrap(identifier).unwrap_unchecked() }.into_inner().expression {
            ExpressionEnum::Identifier(ident) => ident,
            _ => return Err(CompileError {
                span: crate::common::Span::from_pest_span(&span),
                desc: String::from("Left value start can only be an identifier"),
                t: CompileErrorType::InternalFatal
            })
        };
        let mut left_value_field_access_item = Vec::new();
        while left_value_pair_list.len() > 0 {
            let field_access_item_pair = left_value_pair_list.pop().unwrap();
            let item_span = field_access_item_pair.as_span();
            let field_access_item = self.convert_expression(field_access_item_pair.into_inner().next().unwrap())?;

            let ast_left_value_item = ASTLeftValueFieldAccessItem {
                common: NodeCommon {
                    input: self.input,
                    span: item_span
                },
                expression: field_access_item
            };
            left_value_field_access_item.push(Rc::new(RefCell::new(ast_left_value_item)));
        }

        let node_common = NodeCommon {
            input: self.input,
            span,
        };
        let node = ASTAssignmentStatement {
            common: node_common,
            left_value_name: identifier,
            left_value_field_access_list: left_value_field_access_item,
            expression
        }.wrap_statement();

        Ok(Rc::new(RefCell::new(node)))
    }

    pub fn convert_damage_statement(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperStatement<'i>, CompileError> {
        let span = pair.as_span();

        let mut iter = pair.into_inner();

        let name_pair = iter.next().unwrap();
        let character_name_pair = iter.next().unwrap();
        let skill_index_name_pair = iter.next().unwrap();

        let ast_name = ASTIdentifier {
            common: NodeCommon {
                input: self.input,
                span: name_pair.as_span(),
            },
            ident: name_pair.as_str().to_string(),
        };

        let ast_character_name = ASTIdentifier {
            common: NodeCommon {
                input: self.input,
                span: character_name_pair.as_span(),
            },
            ident: character_name_pair.as_str().to_string()
        };

        let ast_skill_index_name = ASTIdentifier {
            common: NodeCommon {
                input: self.input,
                span: skill_index_name_pair.as_span(),
            },
            ident: skill_index_name_pair.as_str().to_string()
        };

        let config = if let Some(x) = iter.next() {
            let temp = self.convert_expression(x)?;
            let c = unsafe { Rc::try_unwrap(temp).unwrap_unchecked() }.into_inner().as_object_literal().unwrap();
            Some(c)
        } else {
            None
        };
        let ast_damage_statement = ASTDamageStatement {
            common: NodeCommon {
                input: self.input,
                span,
            },
            name: Rc::new(RefCell::new(ast_name)),
            character_name: Rc::new(RefCell::new(ast_character_name)),
            skill_index_name: Rc::new(RefCell::new(ast_skill_index_name)),
            skill_param: config
        }.wrap_statement();

        Ok(Rc::new(RefCell::new(ast_damage_statement)))
    }

    pub fn convert_prop_statement(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperStatement<'i>, CompileError> {
        let span = pair.as_span();

        let mut iter = pair.into_inner();
        let var_name_pair = iter.next().unwrap();
        let character_name_pair = iter.next().unwrap();
        let prop_name_pair = iter.next().unwrap();

        let ast_var_name = ASTIdentifier {
            common: NodeCommon {
                input: self.input,
                span: var_name_pair.as_span(),
            },
            ident: var_name_pair.as_str().to_string(),
        };

        let ast_character_name = ASTIdentifier {
            common: NodeCommon {
                input: self.input,
                span: character_name_pair.as_span(),
            },
            ident: character_name_pair.as_str().to_string()
        };

        let ast_prop_name = ASTIdentifier {
            common: NodeCommon {
                input: self.input,
                span: prop_name_pair.as_span(),
            },
            ident: prop_name_pair.as_str().to_string()
        };

        let ast_prop_statement = ASTPropStatement {
            common: NodeCommon {
                input: self.input,
                span,
            },
            character_name: Rc::new(RefCell::new(ast_character_name)),
            name: Rc::new(RefCell::new(ast_var_name)),
            prop_name: Rc::new(RefCell::new(ast_prop_name))
        }.wrap_statement();

        Ok(Rc::new(RefCell::new(ast_prop_statement)))
    }

    pub fn convert_expression_statement(&self, pair: Pair<'i, MonaRule>) -> Result<WrapperStatement<'i>, CompileError> {
        let span = pair.as_span();
        let expression_pair = pair.into_inner().next().unwrap();

        let expression = self.convert_expression(expression_pair)?;

        let node = ASTExpressionStatement {
            common: NodeCommon {
                input: self.input,
                span,
            },
            expression
        }.wrap_statement();

        Ok(Rc::new(RefCell::new(node)))
    }

    pub fn convert_program(&self, pair: Pair<'i, MonaRule>) -> Result<Rc<RefCell<ASTProgram<'i>>>, CompileError> {
        let span = pair.as_span();
        let mut result = Vec::new();
        for statement_pair in pair.into_inner() {
            // println!("{:?}", statement_pair.as_rule());
            // let rule = statement_pair.as_rule();
            // if rule == MonaRule::EOI {
            //     continue;
            // }
            result.push(self.convert_statement(statement_pair)?);
        }

        let node = ASTProgram {
            common: NodeCommon {
                input: self.input,
                span
            },
            statements: result
        };

        Ok(Rc::new(RefCell::new(node)))
    }
}
