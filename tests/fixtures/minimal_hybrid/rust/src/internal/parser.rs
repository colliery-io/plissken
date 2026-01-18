//! Expression parser for scale formulas.
//!
//! This module parses string expressions into scale operations.
//! Demonstrates `rustscale::internal::parser` - a deeply nested namespace.

/// Token types for the expression parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A numeric literal.
    Number(f64),
    /// Addition operator.
    Plus,
    /// Multiplication operator.
    Star,
    /// Left parenthesis.
    LParen,
    /// Right parenthesis.
    RParen,
    /// An identifier (variable name).
    Ident(String),
}

/// A parsed scale expression.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A literal number.
    Literal(f64),
    /// A variable reference.
    Variable(String),
    /// Binary addition.
    Add(Box<Expr>, Box<Expr>),
    /// Binary multiplication.
    Mul(Box<Expr>, Box<Expr>),
}

/// Parser for scale expressions.
pub struct ExprParser {
    tokens: Vec<Token>,
    position: usize,
}

impl ExprParser {
    /// Create a new parser from tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to parse.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    /// Parse the tokens into an expression.
    ///
    /// # Returns
    ///
    /// The parsed expression, or None if parsing fails.
    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplicative()?;

        while self.current() == Some(&Token::Plus) {
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Add(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    fn parse_multiplicative(&mut self) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        while self.current() == Some(&Token::Star) {
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::Mul(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.current()?.clone() {
            Token::Number(n) => {
                self.advance();
                Some(Expr::Literal(n))
            }
            Token::Ident(name) => {
                self.advance();
                Some(Expr::Variable(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_additive()?;
                if self.current() != Some(&Token::RParen) {
                    return None;
                }
                self.advance();
                Some(expr)
            }
            _ => None,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}

/// Tokenize a string expression.
///
/// # Arguments
///
/// * `input` - The expression string to tokenize.
///
/// # Returns
///
/// A vector of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(n) = num.parse() {
                    tokens.push(Token::Number(n));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}
