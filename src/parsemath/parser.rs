/// This module reads tokens returned by Tokenizer and converts them into AST.

use super::{tokenizer::Tokenizer, token::{Token, OperPrec}, ast::Node};
use std::fmt;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token
}

// Public methods
impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseErr> {
        let mut tokenizer = Tokenizer::new(expr);
        let current_token = match tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseErr::InvalidOperator("Invalid character".into()))
        };

        Ok(Parser {
            tokenizer,
            current_token
        })
    }

    /// Generates the AST (the node tree) from the tokens
    pub fn parse(&mut self) -> Result<Node, ParseErr> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}

// Private methods
impl<'a> Parser<'a> {
    /// Main method that constructs AST and is invoked recursively
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseErr> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(
                left_expr.clone())?;
            
            left_expr = right_expr;
        }

        Ok(left_expr)

    }

    /// Constructs AST node for number, taking into account negative prefixes and parenthesis
    fn parse_number(&mut self) -> Result<Node, ParseErr> {
        let token = self.current_token.clone();
        match token {
            Token::Substract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            },
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            },
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }

                Ok(expr)
            }
            _ => Err(ParseErr::UnableToParse("Unable to parse".to_string())),
        }

    }

    /// Parses operators and converts to AST
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseErr> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(
                    OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Substract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(
                    OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(
                    OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(
                    OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(
                    OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseErr::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            )))
        }
    }

    /// Checks for matching parenthesis in expression
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseErr> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseErr::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }

    /// Retrieves next Token from Tokenizer and sets current_token field
    fn get_next_token(&mut self) -> Result<(), ParseErr> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseErr::InvalidOperator("Invalid character".into()))
        };
        self.current_token = next_token;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseErr {
    UnableToParse(String),
    InvalidOperator(String)
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseErr::UnableToParse(e) => write!(f,
                "Error in evaluating {}", e),
            self::ParseErr::InvalidOperator(e) => write!(f,
                "Error in evaluating {}", e),
        }
    }
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseErr {
    fn from(_evalerror: std::boxed::Box<dyn std::error::Error>) -> Self {
        return ParseErr::UnableToParse("Unable to parse".into());
    }
}