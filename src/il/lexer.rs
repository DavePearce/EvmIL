pub use delta_inc::lex::{Error,Result,Span};
use delta_inc::lex::{Scanner,TableTokenizer};
use delta_inc::lex;

// =================================================================
// Token
// =================================================================
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Token {
    AmpersandAmpersand,
    Assert,
    BarBar,
    Comma,
    Dot,
    EOF,
    Equals,
    EqualsEquals,
    Fail,
    Gap,
    Goto,
    Hex,
    If,
    Identifier,
    Integer,
    LeftAngle,
    LeftAngleEquals,
    LeftBrace,
    LeftSquare,
    Minus,
    NewLine,
    Percent,
    Plus,
    Revert,
    RightAngle,
    RightAngleEquals,
    RightBrace,
    RightSlash,
    RightSquare,
    SemiColon,
    ShreakEquals,
    Succeed,
    Star,
    Stop
}

// ======================================================
// Rules
// ======================================================

const ASSERT : &'static [char] = &['a','s','s','e','r','t'];
const FAIL : &'static [char] = &['f','a','i','l'];
const GOTO : &'static [char] = &['g','o','t','o'];
const IF : &'static [char] = &['i','f'];
const REVERT : &'static [char] = &['r','e','v','e','r','t'];
const SUCCEED : &'static [char] = &['s','u','c','c','e','e','d'];
const STOP : &'static [char] = &['s','t','o','p'];

/// Handy type alias for the result type used for all of the lexical
/// rules.
type ScannerResult = std::result::Result<Span<Token>,()>;

/// Scan an (unsigned) integer literal.
fn scan_uint_literal(input: &[char]) -> ScannerResult {
    scan_whilst(input, Token::Integer, |c| c.is_digit(10))
}

/// Scan a hex literal (e.g. `0x12ffc`).
fn scan_hex_literal(input: &[char]) -> ScannerResult {
    if input.len() < 2 || input[0] != '0' || input[1] != 'x' {
        Err(())
    } else {
        let r = scan_whilst(&input[2..], Token::Hex, |c| c.is_digit(16))?;
        // Update span information
        Ok(Span::new(Token::Hex,0..r.region.end+2))
    }
}

/// Scan a keyword, which is simple identifier matching a predefined
/// pattern.
fn scan_keyword(input: &[char]) -> ScannerResult {
    // Extract keyword identifier (if applicable)
    let r = scan_whilst(input, Token::Gap, |c| c.is_ascii_alphabetic())?;
    // Attempt to match it
    let t = match &input[r.range()] {
        ASSERT => Token::Assert,
        FAIL => Token::Fail,
        GOTO => Token::Goto,
        IF => Token::If,
        REVERT => Token::Revert,
        SUCCEED => Token::Succeed,
        STOP => Token::Stop,
        _ => { return Err(()); }
    };
    // Success!
    Ok(Span::new(t,r.region.into()))
}

/// Scan an identifier which starts with an alpabetic character, or an
/// underscore and subsequently contains zero or more alpha-number
/// characters or underscores.
fn scan_identifier(input: &[char]) -> ScannerResult {
    if input.len() > 0 && is_identifier_start(input[0]) {
        scan_whilst(input, Token::Identifier, is_identifier_middle)
    } else {
        Err(())
    }
}

/// Scan all single-character operators.
fn scan_single_operators(input: &[char]) -> ScannerResult {
    if input.len() == 0 {
        Err(())
    } else {
        let t = match input[0] {
            ',' => Token::Comma,
            '.' => Token::Dot,
            '=' => Token::Equals,
            '<' => Token::LeftAngle,
            '(' => Token::LeftBrace,
            '[' => Token::LeftSquare,
            '-' => Token::Minus,
            '%' => Token::Percent,
            '+' => Token::Plus,
            '>' => Token::RightAngle,
            ')' => Token::RightBrace,
            '/' => Token::RightSlash,
            ']' => Token::RightSquare,
            ';' => Token::SemiColon,
            '*' => Token::Star,
            _ => { return Err(()); }
        };
        //
        Ok(Span::new(t,0..1))
    }
}

/// Scan all double-character operators.
fn scan_double_operators(input: &[char]) -> ScannerResult {
    if input.len() <= 1 {
        Err(())
    } else {
        let t = match (input[0], input[1]) {
            ('&','&') => Token::AmpersandAmpersand,
            ('|','|') => Token::BarBar,
            ('=','=') => Token::EqualsEquals,
            ('<','=') => Token::LeftAngleEquals,
            ('>','=') => Token::RightAngleEquals,
            ('!','=') => Token::ShreakEquals,
            _ => { return Err(()); }
        };
        //
        Ok(Span::new(t,0..2))
    }
}

/// Determine whether a given character is the start of an identifier.
fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Determine whether a given character can occur in the middle of an
/// identifier
fn is_identifier_middle(c: char) -> bool {
    c.is_digit(10) || is_identifier_start(c)
}

/// Scan a "gap" which is a sequence of zero or more tabs and spaces.
fn scan_gap(input: &[char]) -> ScannerResult {
    scan_whilst(input, Token::Gap, |c| c == ' ' || c == '\t')
}

fn scan_newline(input: &[char]) -> ScannerResult {
    scan_one(input,Token::NewLine,'\n')
}

/// If there is nothing left to scan, then we've reached the
/// End-Of-File.
fn scan_eof(input: &[char]) -> ScannerResult {
    if input.len() == 0 {
        Ok(Span::new(Token::EOF,0..0))
    } else {
        Err(())
    }
}

/// Helper which scans an item matching a given predicate.  If no
/// characters match, then it fails.
fn scan_whilst<P>(input: &[char], t: Token, pred: P) -> ScannerResult
where P: Fn(char) -> bool {
    let mut i = 0;
    // Continue whilst predicate matches
    while i < input.len() && pred(input[i]) { i = i + 1; }
    // Check what happened
    if i == 0 {
        // Nothing matched
        Err(())
    } else {
        // Something matched
        Ok(Span::new(t, 0..i))
    }
}

fn scan_one(input: &[char], t: Token, c: char) -> ScannerResult {
    if input.len() > 0 && input[0] == c {
        Ok(Span::new(t, 0..1))
    } else {
        Err(())
    }
}

/// The set of rules used for lexing.
static RULES : &'static [Scanner<char,Token>] = &[
    scan_double_operators,
    scan_single_operators,
    scan_keyword,
    scan_identifier,
    scan_hex_literal,
    scan_uint_literal,
    scan_gap,
    scan_newline,
    scan_eof
];

// ======================================================
// Lexer
// ======================================================

pub struct Lexer {
    /// Internal lexer used for the heavy lifting.
    lexer: lex::Lexer<TableTokenizer<char,Token>>
}

impl Lexer {
    /// Construct a `Lexer` from a given string slice.
    pub fn new(input: &str) -> Lexer {
        let tokenizer = TableTokenizer::new(RULES.to_vec());
        let chars = input.chars().collect();
        Lexer{lexer:lex::Lexer::new(chars, tokenizer)}
    }

    /// Turn an integer token into a `i32`.  Observe that this will
    /// panic if the underlying characters of the token don't parse.
    pub fn get_int(&self, t: Span<Token>) -> u32 {
        // Sanity check this makes sense.
        assert!(t.kind == Token::Integer);
        // Extract characters from token.
        let chars = self.lexer.get(t);
        // Convert to string
        let s: String = chars.into_iter().collect();
        // Parse to i32
        s.parse().unwrap()
    }

    pub fn get_str(&self, t: Span<Token>) -> String {
        // Extract characters from token.
        let chars = self.lexer.get(t);
        // Convert to string
        chars.into_iter().collect()
    }

    pub fn get(&self, t: Span<Token>) -> &[char] {
        self.lexer.get(t)
    }

    /// Pass through request to underlying lexer
    pub fn is_eof(&self) -> bool { self.lexer.is_eof() }
    /// Pass through request to underlying lexer
    pub fn peek(&self) -> Span<Token> { self.lexer.peek() }
    /// Pass through request to underlying lexer
    pub fn snap(&mut self, kind : Token) -> Result<Token> {
        self.lexer.snap(kind)
    }
    /// Pass through request to underlying lexer
    pub fn snap_any(&mut self, kinds : &[Token]) -> Result<Token> {
        self.lexer.snap_any(kinds)
    }
}

// ======================================================
// Tests
// ======================================================


#[cfg(test)]
mod tests {
    use crate::il::{Lexer,Token};

    /// Handy definition
    macro_rules! assert_ok {
        ($result:expr) => { assert!($result.is_ok()); };
    }

    #[test]
    fn test_01() {
        let mut l = Lexer::new("");
        assert!(l.peek().kind == Token::EOF);
        assert_ok!(l.snap(Token::EOF));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_02() {
        let mut l = Lexer::new(" ");
        assert!(l.peek().kind == Token::Gap);
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_03() {
        let mut l = Lexer::new("  ");
        assert!(l.peek().kind == Token::Gap);
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_04() {
        let mut l = Lexer::new("\n");
        assert_ok!(l.snap(Token::NewLine));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_05() {
        let mut l = Lexer::new(" \n");
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::NewLine));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_06() {
        let mut l = Lexer::new("\n ");
        assert!(l.peek().kind == Token::NewLine);
        assert_ok!(l.snap(Token::NewLine));
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_07() {
        let mut l = Lexer::new("\t");
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_08() {
        let mut l = Lexer::new("\t ");
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_09() {
        let mut l = Lexer::new(" \t");
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    // Literals

    #[test]
    fn test_10() {
        let mut l = Lexer::new("1");
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_11() {
        let mut l = Lexer::new("  1");
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_12() {
        let mut l = Lexer::new("1234");
        assert!(l.get_int(l.peek()) == 1234);
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_13() {
        let mut l = Lexer::new("1234 ");
        assert!(l.get_int(l.peek()) == 1234);
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_14() {
        let mut l = Lexer::new("1234_");
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_15() {
        let mut l = Lexer::new("1234X");
        assert!(l.get_int(l.peek()) == 1234);
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_16() {
        let mut l = Lexer::new("1234 12");
        assert!(l.get_int(l.peek()) == 1234);
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::EOF));
    }

    // Identifiers

    #[test]
    fn test_20() {
        let mut l = Lexer::new("abc");
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_21() {
        let mut l = Lexer::new("  abc");
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_22() {
        let mut l = Lexer::new("_abc");
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_23() {
        let mut l = Lexer::new("a_bD12233_");
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_24() {
        let mut l = Lexer::new("_abc cd");
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::Gap));
        assert_ok!(l.snap(Token::Identifier));
        assert_ok!(l.snap(Token::EOF));
    }

    // Keywords

    #[test]
    fn test_30() {
        let mut l = Lexer::new("if");
        assert_ok!(l.snap(Token::If));
        assert_ok!(l.snap(Token::EOF));
    }

    // Operators

    #[test]
    fn test_40() {
        let mut l = Lexer::new("(");
        assert_ok!(l.snap(Token::LeftBrace));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_41() {
        let mut l = Lexer::new("((");
        assert_ok!(l.snap(Token::LeftBrace));
        assert_ok!(l.snap(Token::LeftBrace));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_42() {
        let mut l = Lexer::new(")");
        assert_ok!(l.snap(Token::RightBrace));
    }

    #[test]
    fn test_43() {
        let mut l = Lexer::new("))");
        assert_ok!(l.snap(Token::RightBrace));
        assert_ok!(l.snap(Token::RightBrace));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_44() {
        let mut l = Lexer::new("()");
        assert_ok!(l.snap(Token::LeftBrace));
        assert_ok!(l.snap(Token::RightBrace));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_45() {
        let mut l = Lexer::new("<=");
        assert_ok!(l.snap(Token::LeftAngleEquals));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_46() {
        let mut l = Lexer::new(">=");
        assert_ok!(l.snap(Token::RightAngleEquals));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_47() {
        let mut l = Lexer::new("==");
        assert_ok!(l.snap(Token::EqualsEquals));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_48() {
        let mut l = Lexer::new("!=");
        assert_ok!(l.snap(Token::ShreakEquals));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_49() {
        let mut l = Lexer::new("&&");
        assert_ok!(l.snap(Token::AmpersandAmpersand));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_50() {
        let mut l = Lexer::new("||");
        assert_ok!(l.snap(Token::BarBar));
        assert_ok!(l.snap(Token::EOF));
    }

    #[test]
    fn test_61() {
        let mut l = Lexer::new("12345(");
        assert_ok!(l.snap(Token::Integer));
        assert_ok!(l.snap(Token::LeftBrace));
        assert_ok!(l.snap(Token::EOF));
    }
}
