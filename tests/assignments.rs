use evmil::il::Parser;
use evmil::ll::Bytecode;
use evmil::util::ToHexString;

// ============================================================================
// Memory
// ============================================================================

#[test]
pub fn test_memory_01() {
    let p = "memory[0] = 1;";
    check(&p, "0x6001600052");
}

#[test]
pub fn test_memory_02() {
    let p = "memory[0+1] = 2;";
    check(&p, "0x6002600160000152");
}

#[test]
pub fn test_memory_03() {
    let p = "memory[0] = 1+2;";
    check(&p, "0x6002600101600052");
}

// ============================================================================
// Storage
// ============================================================================

#[test]
pub fn test_storage_01() {
    let p = "storage[0] = 1;";
    check(&p, "0x6001600055");
}

#[test]
pub fn test_storage_02() {
    let p = "storage[0+1] = 2;";
    check(&p, "0x6002600160000155");
}

#[test]
pub fn test_storage_03() {
    let p = "storage[0] = 1+2;";
    check(&p, "0x6002600101600055");
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
