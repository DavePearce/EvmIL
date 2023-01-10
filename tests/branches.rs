use evmil::il::Parser;
use evmil::ll::Bytecode;
use evmil::util::ToHexString;

// ============================================================================
// Goto
// ============================================================================

#[test]
pub fn test_goto_01() {
    let p = "goto lab;";
    check(&p, "0x600056");
}

#[test]
pub fn test_goto_02() {
    let p = "goto lab; .lab";
    check(&p, "0x6003565b");
}

#[test]
pub fn test_goto_03() {
    let p = "goto lab; assert 0; .lab";
    check(&p, "0x600a566000600957fe5b5b");
}

// ============================================================================
// If/Goto
// ============================================================================

#[test]
pub fn test_ifgoto_01() {
    let p = "if 1 goto lab;";
    check(&p, "0x6001600057");
}

#[test]
pub fn test_ifgoto_02() {
    let p = "if 1 goto lab; .lab";
    check(&p, "0x60016005575b");
}

#[test]
pub fn test_ifgoto_03() {
    let p = "if memory[0] > 0 goto lab; memory[0] = 1; .lab";
    check(&p, "0x600060005111600e5760016000525b");
}

#[test]
pub fn test_ifgoto_04() {
    let p = "if 1 && 0 goto lab; .lab";
    check(&p, "0x600115600b576000600c575b5b");
}

#[test]
pub fn test_ifgoto_05() {
    let p = "if 1 || 0 goto lab; .lab";
    check(&p, "0x6001600a576000600a575b");
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that compiling a given sequence of terms produces a given
/// hex string.
fn check(stmt: &str, hex: &str) {
    // Parse statement into a term
    let ts = Parser::new(stmt).parse().unwrap();
    // Translate statements into bytecode instructions
    let code = Bytecode::try_from(ts.as_slice()).unwrap();
    // Translate instructions into bytes
    let bytes : Vec<u8> = code.try_into().unwrap();
    // Check against expected hex string
    assert_eq!(hex, bytes.to_hex_string());
}
