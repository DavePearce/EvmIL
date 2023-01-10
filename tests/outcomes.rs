use evmil::ll::Bytecode;
use evmil::il::Parser;
use evmil::util::ToHexString;

// ============================================================================
// Fail
// ============================================================================

#[test]
pub fn test_fail_01() {
    let p = "fail;";
    check(&p, "0xfe");
}

// ============================================================================
// Succeed
// ============================================================================

#[test]
pub fn test_succeed_01() {
    let p = "succeed;";
    check(&p, "0x00");
}

#[test]
pub fn test_succeed_02() {
    let p = "succeed 1;";
    check(&p, "0x600160005260006020f3");
}

// ============================================================================
// Revert
// ============================================================================

#[test]
pub fn test_revert_01() {
    let p = "revert;";
    check(&p, "0x60006000fd");
}

#[test]
pub fn test_revert_02() {
    let p = "revert 1;";
    check(&p, "0x600160005260006020fd");
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
