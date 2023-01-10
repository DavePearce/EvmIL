use std::fmt;
use crate::il::{BinOp,Region,Term};
use crate::il::lexer;
use crate::il::{Lexer,Span,Token};

/// Defines the set of tokens which are considered to identify logical
/// connectives (e.g. `&&`, `||`, etc).
pub const LOGICAL_CONNECTIVES : &'static [Token] = &[
    Token::AmpersandAmpersand,
    Token::BarBar
];

/// Defines the set of tokens which are considered to identify
/// arithmetic comparators (e.g. `<`, `<=`, `==`, etc).
pub const ARITHMETIC_COMPARATORS : &'static [Token] = &[
    Token::EqualsEquals,
    Token::ShreakEquals,
    Token::LeftAngle,
    Token::LeftAngleEquals,
    Token::RightAngle,
    Token::RightAngleEquals
];

/// Defines the set of tokens which are considered to identify
/// arithmetic operators (e.g. `+`, `-`, `*`, etc).
pub const ARITHMETIC_OPERATORS : &'static [Token] = &[
    Token::Minus,
    Token::Percent,
    Token::Plus,
    Token::RightSlash,
    Token::Star
];

pub const BINARY_CONNECTIVES : &'static [ &'static [Token] ] = &[
    &ARITHMETIC_OPERATORS,
    &ARITHMETIC_COMPARATORS,
    &LOGICAL_CONNECTIVES,
];

// =========================================================================
// Error
// =========================================================================

#[derive(Clone,Debug,PartialEq)]
pub enum ErrorCode {
    UnexpectedToken,
    UnexpectedEof,
    ExpectedToken(Token),
    ExpectedTokenIn(Vec<Token>)
}

/// Identifies possible errors stemming from the parser.
#[derive(Debug)]
pub struct Error {
    pub span: Span<Token>,
    pub code: ErrorCode
}

impl Error {
    pub fn new(span: Span<Token>, code: ErrorCode) -> Error {
	Error{span,code}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // temporary for now.
        write!(f,"{:?}",self)
    }
}

impl std::error::Error for Error { }

impl From<lexer::Error<Token>> for Error {
    fn from(p:lexer::Error<Token>) -> Error {
        match p {
            lexer::Error::Expected(t,s) => {
                Error{span:s,code:ErrorCode::ExpectedToken(t)}
            }
            lexer::Error::ExpectedIn(ts,s) => {
                Error{span:s,code:ErrorCode::ExpectedTokenIn(ts)}
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// =========================================================================
// Parser
// =========================================================================

pub struct Parser {
    /// Provides access to our token stream.
    lexer: Lexer
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self { lexer: Lexer::new(input) }
    }

    /// Parse a line of text into a term.
    pub fn parse(&mut self) -> Result<Vec<Term>> {
        let mut terms = Vec::new();
        while !self.lexer.is_eof() {
            terms.push(self.parse_stmt()?);
        }
        Ok(terms)
    }

    // =========================================================================
    // Statements
    // =========================================================================

    fn parse_stmt(&mut self) -> Result<Term> {
        // Skip any leading whitespace
        self.skip_whitespace();
    	// Dispatch on lookahead
    	match self.lexer.peek().kind {
    	    Token::Assert => self.parse_stmt_assert(),
    	    Token::Fail => self.parse_stmt_fail(),
    	    Token::Stop => self.parse_stmt_stop(),
    	    Token::Goto => self.parse_stmt_goto(),
    	    Token::If => self.parse_stmt_if(),
            Token::Dot => self.parse_stmt_label(),
    	    Token::Revert => self.parse_stmt_revert(),
            Token::Succeed => self.parse_stmt_succeed(),
            _ => self.parse_stmt_assign()
        }
    }

    pub fn parse_stmt_assert(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Assert)?;
    	let expr = self.parse_expr()?;
        self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Assert(Box::new(expr)))
    }

    pub fn parse_stmt_assign(&mut self) -> Result<Term> {
    	let lhs = self.parse_expr()?;
        self.skip_whitespace();
        self.lexer.snap(Token::Equals)?;
    	let rhs = self.parse_expr()?;
        self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Assignment(Box::new(lhs),Box::new(rhs)))
    }

    pub fn parse_stmt_fail(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Fail)?;
    	self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Fail)
    }

    pub fn parse_stmt_stop(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Stop)?;
    	self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Stop)
    }

    pub fn parse_stmt_goto(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Goto)?;
        self.skip_whitespace();
    	let target = self.lexer.snap(Token::Identifier)?;
        self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Goto(self.lexer.get_str(target)))
    }

    pub fn parse_stmt_if(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::If)?;
    	let expr = self.parse_expr()?;
        self.skip_whitespace();
    	self.lexer.snap(Token::Goto)?;
        self.skip_whitespace();
    	let target = self.lexer.snap(Token::Identifier)?;
        self.lexer.snap(Token::SemiColon)?;
        Ok(Term::IfGoto(Box::new(expr),self.lexer.get_str(target)))
    }

    pub fn parse_stmt_label(&mut self) -> Result<Term> {
        self.lexer.snap(Token::Dot)?;
    	let target = self.lexer.snap(Token::Identifier)?;
        Ok(Term::Label(self.lexer.get_str(target)))
    }

    pub fn parse_stmt_revert(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Revert)?;
        let exprs = self.parse_expr_list(Token::SemiColon)?;
    	self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Revert(exprs))
    }

    pub fn parse_stmt_succeed(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Succeed)?;
        let exprs = self.parse_expr_list(Token::SemiColon)?;
    	self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Succeed(exprs))
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    pub fn parse_expr(&mut self) -> Result<Term> {
        self.parse_expr_binary(3)
    }

    /// Parse a binary expression at a given _level_.  Higher levels
    /// indicate expressions which bind _less tightly_.  Furthermore,
    /// level `0` corresponds simply to parsing a unary expression.
    pub fn parse_expr_binary(&mut self, level: usize) -> Result<Term> {
        if level == 0 {
            self.parse_expr_postfix()
        } else {
            let tokens = BINARY_CONNECTIVES[level-1];
            // Parse level below
    	    let lhs = self.parse_expr_binary(level-1)?;
            // Skip remaining whitespace (on this line)
            self.skip_whitespace();
	    // Check whether logical connective follows
    	    let lookahead = self.lexer.snap_any(tokens);
            //
            match lookahead {
                Ok(s) => {
                    // FIXME: turn this into a loop?
	            let rhs = self.parse_expr_binary(level)?;
                    let bop = Self::binop_from_token(s.kind).unwrap();
	            Ok(Term::Binary(bop,Box::new(lhs),Box::new(rhs)))
                }
                Err(_) => {
                    Ok(lhs)
                }
            }
        }
    }

    pub fn parse_expr_postfix(&mut self) -> Result<Term> {
        let mut expr = self.parse_expr_term()?;
        // Check for postfix unary operator.
    	let lookahead = self.lexer.peek();
    	// FIXME: managed nested operators
        expr = match lookahead.kind {
            Token::LeftSquare => self.parse_expr_arrayaccess(expr)?,
            //TokenType::LeftBrace => self.parse_expr_invoke(expr)?,
            _ => expr
        };
        // Done
        Ok(expr)
    }

    pub fn parse_expr_arrayaccess(&mut self, src: Term) -> Result<Term> {
        self.lexer.snap(Token::LeftSquare)?;
        let index = self.parse_expr()?;
        self.lexer.snap(Token::RightSquare)?;
        let expr = Term::ArrayAccess(Box::new(src),Box::new(index));
        // Done
        Ok(expr)
    }

    pub fn parse_expr_term(&mut self) -> Result<Term> {
        // Skip whitespace
        self.skip_whitespace();
        //
    	let lookahead = self.lexer.peek();
    	//
    	let expr = match lookahead.kind {
    	    Token::Integer => self.parse_literal_int()?,
    	    Token::Hex => self.parse_literal_hex()?,
            Token::Identifier => self.parse_variable_access()?,
    	    Token::LeftBrace => self.parse_expr_bracketed()?,
    	    _ => {
    		return Err(Error::new(lookahead,ErrorCode::UnexpectedToken));
    	    }
    	};
        // Done
        Ok(expr)
    }

    pub fn parse_literal_int(&mut self) -> Result<Term> {
        let tok = self.lexer.snap(Token::Integer)?;
        // Extract characters making up literal
        let chars = self.lexer.get_str(tok);
        // Convert characters into digits
        let digits = chars.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        // All good!
        Ok(Term::Int(digits))
    }

    pub fn parse_literal_hex(&mut self) -> Result<Term> {
        let tok = self.lexer.snap(Token::Hex)?;
        // Extract characters making up literal
        let chars = &self.lexer.get_str(tok)[2..];
        // Convert characters into digits
        let digits = chars.chars().map(|c| c.to_digit(16).unwrap() as u8).collect();
        // All good!
        Ok(Term::Hex(digits))
    }

    pub fn parse_variable_access(&mut self) -> Result<Term> {
    	let tok = self.lexer.snap(Token::Identifier)?;
        // Extract characters making up literal
        let chars = self.lexer.get_str(tok);
        // Match built-ins
        let expr = match chars.as_str() {
            "memory" => Term::MemoryAccess(Region::Memory),
            "storage" => Term::MemoryAccess(Region::Storage),
            "calldata" => Term::MemoryAccess(Region::CallData),
    	    _ => {
    		return Err(Error::new(tok,ErrorCode::UnexpectedToken));
    	    }
        };
        //
        Ok(expr)
    }

    pub fn parse_expr_bracketed(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::LeftBrace)?;
    	let expr = self.parse_expr();
    	self.lexer.snap(Token::RightBrace)?;
        expr
    }

    /// Parse a sequence of expression separated by a comma.
    pub fn parse_expr_list(&mut self, terminator: Token) -> Result<Vec<Term>> {
        let mut exprs = Vec::new();
        while !self.lexer.is_eof() && self.lexer.peek().kind != terminator {
            if exprs.len() > 0 {
                self.skip_whitespace();
                self.lexer.snap(Token::Comma)?;
            }
            exprs.push(self.parse_expr()?);
        }
        // Done
        Ok(exprs)
    }



    // =========================================================================
    // Helpers
    // =========================================================================

    fn skip_whitespace(&mut self) {
        let lookahead = self.lexer.peek();
        //
        match lookahead.kind {
            Token::Gap|Token::NewLine => {
                self.lexer.snap(lookahead.kind).unwrap();
                self.skip_whitespace()
            }
            _ => {
                // Do nothing!
            }
        }
    }

    fn binop_from_token(token: Token) -> Option<BinOp> {
	let bop = match token {
            // // Equality
            Token::EqualsEquals => BinOp::Equals,
            Token::ShreakEquals => BinOp::NotEquals,
            // // Comparison
	    Token::LeftAngle => BinOp::LessThan,
            Token::LeftAngleEquals => BinOp::LessThanOrEquals,
            Token::RightAngle => BinOp::GreaterThan,
            Token::RightAngleEquals => BinOp::GreaterThanOrEquals,
            // Arithmetic
            Token::Minus => BinOp::Subtract,
	    Token::Percent => BinOp::Remainder,
	    Token::Plus => BinOp::Add,
            Token::RightSlash => BinOp::Divide,
            Token::Star => BinOp::Multiply,
            // // Logical
            Token::AmpersandAmpersand => BinOp::LogicalAnd,
            Token::BarBar => BinOp::LogicalOr,
            // No match
	    _ => { return None; }
	};
        Some(bop)
    }
}
