/// This module contains Token structure

/// Defines list of valid tokens that can be constructed from arithmetic expression by Tokenizer
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Add,
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}

// Operator precedence rules
/// Defines all the OperPrec levels, from lowest to highest.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Substract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,

            _ => DefaultZero
        }
    }
}