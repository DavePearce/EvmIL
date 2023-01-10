use evmil::il::Parser;
use evmil::ll::Bytecode;
use evmil::util::ToHexString;

// ============================================================================
// Integer encodings
// ============================================================================

#[test]
pub fn test_int_01() {
    let p = "assert 1;";
    check(&p, "0x6001600657fe5b");
}

#[test]
pub fn test_int_02() {
    let p = "assert 10;";
    check(&p, "0x600a600657fe5b");
}

#[test]
pub fn test_int_03() {
    let p = "assert 203;";
    check(&p, "0x60cb600657fe5b");
}

#[test]
pub fn test_int_04() {
    let p = "assert 5203;";
    check(&p, "0x611453600757fe5b");
}

#[test]
pub fn test_int_05() {
    let p = "assert 33873;";
    check(&p, "0x618451600757fe5b");
}

#[test]
pub fn test_int_06() {
    let p = "assert 56872345;";
    check(&p, "0x630363cd99600957fe5b");
}

#[test]
pub fn test_int_07() {
    let p = "assert 340282366920938463463374607431768211455;";
    check(&p, "0x6fffffffffffffffffffffffffffffffff601557fe5b");
}

// #[test]
// pub fn test_int_08() {
//     // Largest possible digit.
//     let p = "assert 115792089237316195423570985008687907853269984665640564039457584007913129639935";
//     //
//     check(&p, "0x600a600657fe5b");
// }

// ============================================================================
// Integer encodings
// ============================================================================

#[test]
pub fn test_hex_01() {
    let p = "assert 0x1;";
    check(&p, "0x6001600657fe5b");
}

#[test]
pub fn test_hex_02() {
    let p = "assert 0x0a;";
    check(&p, "0x600a600657fe5b");
}

#[test]
pub fn test_hex_03() {
    let p = "assert 0xcb;";
    check(&p, "0x60cb600657fe5b");
}

#[test]
pub fn test_hex_04() {
    let p = "assert 0x1453;";
    check(&p, "0x611453600757fe5b");
}

#[test]
pub fn test_hex_05() {
    let p = "assert 0x008451;";
    check(&p, "0x618451600757fe5b");
}

#[test]
pub fn test_hex_06() {
    let p = "assert 0x363cd99;";
    check(&p, "0x630363cd99600957fe5b");
}

#[test]
pub fn test_hex_07() {
    let p = "assert 0xffffffffffffffffffffffffffffffff;";
    check(&p, "0x6fffffffffffffffffffffffffffffffff601557fe5b");
}

// ============================================================================
// Binary Operators
// ============================================================================

#[test]
pub fn test_add_01() {
    let p = "assert 1+2;";
    check(&p, "0x6002600101600957fe5b");
}

#[test]
pub fn test_add_02() {
    let p = "assert (1+2);";
    check(&p, "0x6002600101600957fe5b");
}

#[test]
pub fn test_add_03() {
    let p = "assert 1+(3+2);";
    check(&p, "0x6002600301600101600c57fe5b");
}

#[test]
pub fn test_sub_01() {
    let p = "assert 1-2;";
    check(&p, "0x6002600103600957fe5b");
}

#[test]
pub fn test_sub_02() {
    let p = "assert 1-(3-2);";
    check(&p, "0x6002600303600103600c57fe5b");
}

// #[test]
// pub fn test_sub_03() {
//     let p = "assert 1-3-2";
//     check(&p, "0x6001600303600203600c57fe5b");
// }

#[test]
pub fn test_mul_01() {
    let p = "assert 1*2;";
    check(&p, "0x6002600102600957fe5b");
}

#[test]
pub fn test_mul_02() {
    let p = "assert 1*2*3;";
    check(&p, "0x6003600202600102600c57fe5b");
}

#[test]
pub fn test_div_01() {
    let p = "assert 1 / 2;";
    check(&p, "0x6002600104600957fe5b");
}

// #[test]
// pub fn test_div_02() {
//     let p = "assert 3 / 2 / 3";
//     check(&p, "0x6003600204600304600c57fe5b");
// }

#[test]
pub fn test_mod_01() {
    let p = "assert 1 % 2;";
    check(&p, "0x6002600106600957fe5b");
}

// ============================================================================
// Binary Comparators
// ============================================================================

#[test]
pub fn test_lt_01() {
    let p = "assert 1 < 2;";
    check(&p, "0x6002600110600957fe5b");
}

#[test]
pub fn test_lteq_01() {
    let p = "assert 1 <= 2;";
    check(&p, "0x600260011115600a57fe5b");
}

#[test]
pub fn test_gt_01() {
    let p = "assert 1 > 2;";
    check(&p, "0x6002600111600957fe5b");
}

#[test]
pub fn test_gteq_01() {
    let p = "assert 1 >= 2;";
    check(&p, "0x600260011015600a57fe5b");
}

#[test]
pub fn test_eq_01() {
    let p = "assert 1 == 2;";
    check(&p, "0x6002600114600957fe5b");
}

#[test]
pub fn test_neq_01() {
    let p = "assert 1 != 2;";
    check(&p, "0x600260011415600a57fe5b");
}

// ============================================================================
// Logical Operators
// ============================================================================

#[test]
pub fn test_and_01() {
    let p = "assert (1 < 2) && (2 < 3);";
    check(&p, "0x60026001101560115760036002106013575bfe5b");
}

#[test]
pub fn test_or_01() {
    let p = "assert (1 < 2) || (2 < 3);";
    check(&p, "0x60026001106011576003600210601157fe5b");
}

// ============================================================================
// Memory / Storage / Calldata
// ============================================================================

#[test]
pub fn test_array_access_01() {
    let p = "assert memory[1] >= 0;";
    check(&p, "0x60006001511015600b57fe5b");
}

#[test]
pub fn test_array_access_02() {
    let p = "assert storage[1] >= 0;";
    check(&p, "0x60006001541015600b57fe5b");
}

#[test]
pub fn test_array_access_03() {
    let p = "assert calldata[1] >= 0;";
    check(&p, "0x60006001351015600b57fe5b");
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
