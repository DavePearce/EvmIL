use evmil::ll::Instruction;
use evmil::ll::Instruction::*;
use evmil::util::FromHexString;
use evmil::analysis::{CfaState,Disassembly};

// ============================================================================
// Instruction Tests
// ============================================================================
//
// The aim of these tests is to execute every single instruction at
// least once.

#[test]
pub fn test_disassemble_insn_00() {
    check("0x00", &[STOP]);
}

#[test]
pub fn test_disassemble_insn_01() {
    let bytecode = format!("0x6008600180{}50565b","01");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),ADD,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_02() {
    let bytecode = format!("0x6008600180{}50565b","02");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),MUL,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_03() {
    let bytecode = format!("0x6008600180{}50565b","03");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SUB,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_04() {
    let bytecode = format!("0x6008600180{}50565b","04");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),DIV,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_05() {
    let bytecode = format!("0x6008600180{}50565b","05");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SDIV,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_06() {
    let bytecode = format!("0x6008600180{}50565b","06");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),MOD,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_07() {
    let bytecode = format!("0x6008600180{}50565b","07");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SMOD,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_08() {
    let bytecode = format!("0x600960018080{}50565b","08");
    check(&bytecode, &[PUSH(vec![0x09]),PUSH(vec![0x01]),DUP(1),DUP(1),ADDMOD,POP,JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_09() {
    let bytecode = format!("0x600960018080{}50565b","09");
    check(&bytecode, &[PUSH(vec![0x09]),PUSH(vec![0x01]),DUP(1),DUP(1),MULMOD,POP,JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_0a() {
    let bytecode = format!("0x6008600180{}50565b","0a");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),EXP,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_0b() {
    let bytecode = format!("0x6008600180{}50565b","0b");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SIGNEXTEND,POP,JUMP,JUMPDEST(8)]);
}

// 10s

#[test]
pub fn test_disassemble_insn_10() {
    let bytecode = format!("0x6008600180{}50565b","10");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),LT,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_11() {
    let bytecode = format!("0x6008600180{}50565b","11");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),GT,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_12() {
    let bytecode = format!("0x6008600180{}50565b","12");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SLT,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_13() {
    let bytecode = format!("0x6008600180{}50565b","13");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SGT,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_14() {
    let bytecode = format!("0x6008600180{}50565b","14");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),EQ,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_15() {
    let bytecode = format!("0x60076001{}50565b","15");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),ISZERO,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_16() {
    let bytecode = format!("0x6008600180{}50565b","16");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),AND,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_17() {
    let bytecode = format!("0x6008600180{}50565b","17");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),OR,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_18() {
    let bytecode = format!("0x6008600180{}50565b","18");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),XOR,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_19() {
    let bytecode = format!("0x60076001{}50565b","19");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),NOT,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_1a() {
    let bytecode = format!("0x6008600180{}50565b","1a");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),BYTE,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_1b() {
    let bytecode = format!("0x6008600180{}50565b","1b");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SHL,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_1c() {
    let bytecode = format!("0x6008600180{}50565b","1c");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SHR,POP,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_1d() {
    let bytecode = format!("0x6008600180{}50565b","1d");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),SAR,POP,JUMP,JUMPDEST(8)]);
}

// 20s

#[test]
pub fn test_disassemble_insn_20() {
    let bytecode = format!("0x6008600180{}50565b","20");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),KECCAK256,POP,JUMP,JUMPDEST(8)]);
}

// 30s

#[test]
pub fn test_disassemble_insn_30() {
    let bytecode = format!("0x6005{}50565b","30");
    check(&bytecode, &[PUSH(vec![0x05]),ADDRESS,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_31() {
    let bytecode = format!("0x60076001{}50565b","31");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),BALANCE,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_32() {
    let bytecode = format!("0x6005{}50565b","32");
    check(&bytecode, &[PUSH(vec![0x05]),ORIGIN,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_33() {
    let bytecode = format!("0x6005{}50565b","33");
    check(&bytecode, &[PUSH(vec![0x05]),CALLER,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_34() {
    check("0x60053450565b", &[PUSH(vec![0x05]),CALLVALUE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_35() {
    check("0x600760003550565b", &[PUSH(vec![0x07]),PUSH(vec![0x00]),CALLDATALOAD,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_36() {
    check("0x60053650565b", &[PUSH(vec![0x05]),CALLDATASIZE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_37() {
    let bytecode = format!("0x600860018080{}565b","37");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),DUP(1),CALLDATACOPY,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_38() {
    let bytecode = format!("0x6005{}50565b","38");
    check(&bytecode, &[PUSH(vec![0x05]),CODESIZE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_39() {
    let bytecode = format!("0x600860018080{}565b","39");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),DUP(1),CODECOPY,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_3a() {
    let bytecode = format!("0x6005{}50565b","3a");
    check(&bytecode, &[PUSH(vec![0x05]),GASPRICE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_3b() {
    let bytecode = format!("0x60076001{}50565b","3b");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),EXTCODESIZE,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_3c() {
    let bytecode = format!("0x60096001808080{}565b","3c");
    check(&bytecode, &[PUSH(vec![0x09]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),EXTCODECOPY,JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_3d() {
    let bytecode = format!("0x6005{}50565b","3d");
    check(&bytecode, &[PUSH(vec![0x05]),RETURNDATASIZE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_3e() {
    let bytecode = format!("0x600860018080{}565b","3e");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),DUP(1),RETURNDATACOPY,JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_3f() {
    let bytecode = format!("0x60076001{}50565b","3f");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),EXTCODEHASH,POP,JUMP,JUMPDEST(7)]);
}

// 40s

#[test]
pub fn test_disassemble_insn_40() {
    let bytecode = format!("0x60076001{}50565b","40");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),BLOCKHASH,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_41() {
    let bytecode = format!("0x6005{}50565b","41");
    check(&bytecode, &[PUSH(vec![0x05]),COINBASE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_42() {
    let bytecode = format!("0x6005{}50565b","42");
    check(&bytecode, &[PUSH(vec![0x05]),TIMESTAMP,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_43() {
    let bytecode = format!("0x6005{}50565b","43");
    check(&bytecode, &[PUSH(vec![0x05]),NUMBER,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_44() {
    let bytecode = format!("0x6005{}50565b","44");
    check(&bytecode, &[PUSH(vec![0x05]),DIFFICULTY,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_45() {
    let bytecode = format!("0x6005{}50565b","45");
    check(&bytecode, &[PUSH(vec![0x05]),GASLIMIT,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_46() {
    let bytecode = format!("0x6005{}50565b","46");
    check(&bytecode, &[PUSH(vec![0x05]),CHAINID,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_47() {
    let bytecode = format!("0x6005{}50565b","47");
    check(&bytecode, &[PUSH(vec![0x05]),SELFBALANCE,POP,JUMP,JUMPDEST(5)]);
}

// 50s

#[test]
pub fn test_disassemble_insn_50() {
    check("0x6006600150565b", &[PUSH(vec![0x06]),PUSH(vec![0x01]),POP,JUMP,JUMPDEST(6)]);
}

#[test]
pub fn test_disassemble_insn_51() {
    check("0x600760015150565b", &[PUSH(vec![0x07]),PUSH(vec![0x01]),MLOAD,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_52() {
    let bytecode = format!("0x6007600180{}565b","52");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),MSTORE,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_53() {
    let bytecode = format!("0x6007600180{}565b","53");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),MSTORE8,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_54() {
    let bytecode = format!("0x60076001{}50565b","54");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),SLOAD,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_55() {
    let bytecode = format!("0x6007600180{}565b","55");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),SSTORE,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_56() {
    // A minimal two-block program
    check("0x60076005565b565b", &[PUSH(vec![7]),PUSH(vec![5]),JUMP,JUMPDEST(5),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_57() {
    // A minimal two-block program
    check("0x600a6000516008575b565b", &[PUSH(vec![0xa]),PUSH(vec![0]),MLOAD,PUSH(vec![0x8]),JUMPI,JUMPDEST(8),JUMP,JUMPDEST(0xa)]);
}

#[test]
pub fn test_disassemble_insn_58() {
    let bytecode = format!("0x6005{}50565b","58");
    check(&bytecode, &[PUSH(vec![0x05]),PC,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_59() {
    let bytecode = format!("0x6005{}50565b","59");
    check(&bytecode, &[PUSH(vec![0x05]),MSIZE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_5a() {
    let bytecode = format!("0x6005{}50565b","5a");
    check(&bytecode, &[PUSH(vec![0x05]),GAS,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_5b() {
    let bytecode = format!("0x6004{}565b","5b");
    check(&bytecode, &[PUSH(vec![0x04]),JUMPDEST(2),JUMP,JUMPDEST(4)]);
}

// 60s

#[test]
pub fn test_disassemble_insn_60() {
    check("0x6003565b", &[PUSH(vec![0x03]),JUMP,JUMPDEST(3)]);
}

#[test]
pub fn test_disassemble_insn_61() {
    check("0x610004565b", &[PUSH(vec![0x0,0x04]),JUMP,JUMPDEST(4)]);
}

#[test]
pub fn test_disassemble_insn_62() {
    check("0x62000005565b", &[PUSH(vec![0x0,0x0,0x05]),JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_63() {
    check("0x6300000006565b", &[PUSH(vec![0x0,0x0,0x0,0x06]),JUMP,JUMPDEST(6)]);
}

#[test]
pub fn test_disassemble_insn_64() {
    check("0x640000000007565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x07]),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_65() {
    check("0x65000000000008565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x08]),JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_66() {
    check("0x6600000000000009565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x09]),JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_67() {
    check("0x67000000000000000a565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0a]),JUMP,JUMPDEST(10)]);
}

#[test]
pub fn test_disassemble_insn_68() {
    check("0x6800000000000000000b565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0b]),JUMP,JUMPDEST(11)]);
}

#[test]
pub fn test_disassemble_insn_69() {
	check("0x690000000000000000000c565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0c]),JUMP,JUMPDEST(12)]);
}

#[test]
pub fn test_disassemble_insn_6a() {
    check("0x6a000000000000000000000d565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0d]),JUMP,JUMPDEST(13)]);
}

#[test]
pub fn test_disassemble_insn_6b() {
    check("0x6b00000000000000000000000e565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0e]),JUMP,JUMPDEST(14)]);
}

#[test]
pub fn test_disassemble_insn_6c() {
    check("0x6c0000000000000000000000000f565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0f]),JUMP,JUMPDEST(15)]);
}

#[test]
pub fn test_disassemble_insn_6d() {
    check("0x6d0000000000000000000000000010565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x10]),JUMP,JUMPDEST(16)]);
}

#[test]
pub fn test_disassemble_insn_6e() {
    check("0x6e000000000000000000000000000011565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x11]),JUMP,JUMPDEST(17)]);
}

#[test]
pub fn test_disassemble_insn_6f() {
    check("0x6f00000000000000000000000000000012565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x12]),JUMP,JUMPDEST(18)]);
}

#[test]
pub fn test_disassemble_insn_70() {
    check("0x700000000000000000000000000000000013565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x13]),JUMP,JUMPDEST(19)]);
}

#[test]
pub fn test_disassemble_insn_71() {
    check("0x71000000000000000000000000000000000014565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x14]),JUMP,JUMPDEST(20)]);
}

#[test]
pub fn test_disassemble_insn_72() {
    check("0x7200000000000000000000000000000000000015565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x15]),JUMP,JUMPDEST(21)]);
}

#[test]
pub fn test_disassemble_insn_73() {
    check("0x730000000000000000000000000000000000000016565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x16]),JUMP,JUMPDEST(22)]);
}

#[test]
pub fn test_disassemble_insn_74() {
    check("0x74000000000000000000000000000000000000000017565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x17]),JUMP,JUMPDEST(23)]);
}

#[test]
pub fn test_disassemble_insn_75() {
    check("0x7500000000000000000000000000000000000000000018565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x18]),JUMP,JUMPDEST(24)]);
}

#[test]
pub fn test_disassemble_insn_76() {
    check("0x760000000000000000000000000000000000000000000019565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x19]),JUMP,JUMPDEST(25)]);
}

#[test]
pub fn test_disassemble_insn_77() {
    check("0x7700000000000000000000000000000000000000000000001a565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1a]),JUMP,JUMPDEST(26)]);
}

#[test]
pub fn test_disassemble_insn_78() {
    check("0x780000000000000000000000000000000000000000000000001b565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1b]),JUMP,JUMPDEST(27)]);
}

#[test]
pub fn test_disassemble_insn_79() {
    check("0x79000000000000000000000000000000000000000000000000001c565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1c]),JUMP,JUMPDEST(28)]);
}

#[test]
pub fn test_disassemble_insn_7a() {
    check("0x7a00000000000000000000000000000000000000000000000000001d565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1d]),JUMP,JUMPDEST(29)]);
}

#[test]
pub fn test_disassemble_insn_7b() {
    check("0x7b0000000000000000000000000000000000000000000000000000001e565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1e]),JUMP,JUMPDEST(30)]);
}

#[test]
pub fn test_disassemble_insn_7c() {
    check("0x7c000000000000000000000000000000000000000000000000000000001f565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1f]),JUMP,JUMPDEST(31)]);
}

#[test]
pub fn test_disassemble_insn_7d() {
    check("0x7d000000000000000000000000000000000000000000000000000000000020565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x20]),JUMP,JUMPDEST(32)]);
}

#[test]
pub fn test_disassemble_insn_7e() {
    check("0x7e00000000000000000000000000000000000000000000000000000000000021565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x21]),JUMP,JUMPDEST(33)]);
}

#[test]
pub fn test_disassemble_insn_7f() {
    check("0x7f0000000000000000000000000000000000000000000000000000000000000022565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x22]),JUMP,JUMPDEST(34)]);
}

// 80s

#[test]
pub fn test_disassemble_insn_80() {
    check("0x600480565b", &[PUSH(vec![0x04]),DUP(1),JUMP,JUMPDEST(0x04)]);
}

#[test]
pub fn test_disassemble_insn_81() {
    check("0x6006600081565b", &[PUSH(vec![0x06]),PUSH(vec![0x00]),DUP(2),JUMP,JUMPDEST(0x06)]);
}

#[test]
pub fn test_disassemble_insn_82() {
    check("0x60086000600082565b", &[PUSH(vec![0x08]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(3),JUMP,JUMPDEST(0x08)]);
}

#[test]
pub fn test_disassemble_insn_83() {
    check("0x600a60006000600083565b", &[PUSH(vec![0x0a]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(4),JUMP,JUMPDEST(0x0a)]);
}

#[test]
pub fn test_disassemble_insn_84() {
    check("0x600c600060006000600084565b", &[PUSH(vec![0x0c]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(5),JUMP,JUMPDEST(0x0c)]);
}

#[test]
pub fn test_disassemble_insn_85() {
    check("0x600e6000600060006000600085565b", &[PUSH(vec![0x0e]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(6),JUMP,JUMPDEST(0x0e)]);
}

#[test]
pub fn test_disassemble_insn_86() {
    check("0x601060006000600060006000600086565b", &[PUSH(vec![0x10]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(7),JUMP,JUMPDEST(0x10)]);
}

#[test]
pub fn test_disassemble_insn_87() {
    check("0x6012600060006000600060006000600087565b", &[PUSH(vec![0x12]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(8),JUMP,JUMPDEST(0x12)]);
}

#[test]
pub fn test_disassemble_insn_88() {
    check("0x60146000600060006000600060006000600088565b", &[PUSH(vec![0x14]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(9),JUMP,JUMPDEST(0x14)]);
}

#[test]
pub fn test_disassemble_insn_89() {
    check("0x601660006000600060006000600060006000600089565b", &[PUSH(vec![0x16]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(10),JUMP,JUMPDEST(0x16)]);
}

#[test]
pub fn test_disassemble_insn_8a() {
    check("0x601860006000600060006000600060006000600060008a565b", &[PUSH(vec![0x18]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(11),JUMP,JUMPDEST(0x18)]);
}

#[test]
pub fn test_disassemble_insn_8b() {
    check("0x601a600060006000600060006000600060006000600060008b565b", &[PUSH(vec![0x1a]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(12),JUMP,JUMPDEST(0x1a)]);
}

#[test]
pub fn test_disassemble_insn_8c() {
    check("0x601c6000600060006000600060006000600060006000600060008c565b", &[PUSH(vec![0x1c]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(13),JUMP,JUMPDEST(0x1c)]);
}

#[test]
pub fn test_disassemble_insn_8d() {
    check("0x601e60006000600060006000600060006000600060006000600060008d565b", &[PUSH(vec![0x1e]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(14),JUMP,JUMPDEST(0x1e)]);
}

#[test]
pub fn test_disassemble_insn_8e() {
    check("0x6020600060006000600060006000600060006000600060006000600060008e565b", &[PUSH(vec![0x20]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(15),JUMP,JUMPDEST(0x20)]);
}

#[test]
pub fn test_disassemble_insn_8f() {
    check("0x60226000600060006000600060006000600060006000600060006000600060008f565b", &[PUSH(vec![0x22]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(16),JUMP,JUMPDEST(0x22)]);
}

// 90s

#[test]
pub fn test_disassemble_insn_90() {
    check("0x60053490565b", &[PUSH(vec![0x05]),CALLVALUE,SWAP(1),JUMP,JUMPDEST(0x05)]);
}

#[test]
pub fn test_disassemble_insn_91() {
    check("0x600760003491565b", &[PUSH(vec![0x07]),PUSH(vec![0x00]),CALLVALUE,SWAP(2),JUMP,JUMPDEST(0x07)]);
}

#[test]
pub fn test_disassemble_insn_92() {
    check("0x6009600060003492565b", &[PUSH(vec![0x09]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(3),JUMP,JUMPDEST(0x09)]);
}

#[test]
pub fn test_disassemble_insn_93() {
    check("0x600b6000600060003493565b", &[PUSH(vec![0x0b]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(4),JUMP,JUMPDEST(0x0b)]);
}

#[test]
pub fn test_disassemble_insn_94() {
    check("0x600d60006000600060003494565b", &[PUSH(vec![0x0d]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(5),JUMP,JUMPDEST(0x0d)]);
}

#[test]
pub fn test_disassemble_insn_95() {
    check("0x600f600060006000600060003495565b", &[PUSH(vec![0x0f]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(6),JUMP,JUMPDEST(0x0f)]);
}

#[test]
pub fn test_disassemble_insn_96() {
    check("0x60116000600060006000600060003496565b", &[PUSH(vec![0x11]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(7),JUMP,JUMPDEST(0x11)]);
}

#[test]
pub fn test_disassemble_insn_97() {
    check("0x601360006000600060006000600060003497565b", &[PUSH(vec![0x13]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(8),JUMP,JUMPDEST(0x13)]);
}

#[test]
pub fn test_disassemble_insn_98() {
    check("0x6015600060006000600060006000600060003498565b", &[PUSH(vec![0x15]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(9),JUMP,JUMPDEST(0x15)]);
}

#[test]
pub fn test_disassemble_insn_99() {
    check("0x60176000600060006000600060006000600060003499565b", &[PUSH(vec![0x17]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(10),JUMP,JUMPDEST(0x17)]);
}

#[test]
pub fn test_disassemble_insn_9a() {
    check("0x60196000600060006000600060006000600060006000349a565b", &[PUSH(vec![0x19]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(11),JUMP,JUMPDEST(0x19)]);
}

#[test]
pub fn test_disassemble_insn_9b() {
    check("0x601b60006000600060006000600060006000600060006000349b565b", &[PUSH(vec![0x1b]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(12),JUMP,JUMPDEST(0x1b)]);
}

#[test]
pub fn test_disassemble_insn_9c() {
    check("0x601d600060006000600060006000600060006000600060006000349c565b", &[PUSH(vec![0x1d]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(13),JUMP,JUMPDEST(0x1d)]);
}

#[test]
pub fn test_disassemble_insn_9d() {
    check("0x601f6000600060006000600060006000600060006000600060006000349d565b", &[PUSH(vec![0x1f]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(14),JUMP,JUMPDEST(0x1f)]);
}

#[test]
pub fn test_disassemble_insn_9e() {
    check("0x602160006000600060006000600060006000600060006000600060006000349e565b", &[PUSH(vec![0x21]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(15),JUMP,JUMPDEST(0x21)]);
}

#[test]
pub fn test_disassemble_insn_9f() {
    check("0x6023600060006000600060006000600060006000600060006000600060006000349f565b", &[PUSH(vec![0x23]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),CALLVALUE,SWAP(16),JUMP,JUMPDEST(0x23)]);
}

// a0s: Logging Operations

#[test]
pub fn test_disassemble_insn_a0() {
    let bytecode = format!("0x6007600180{}565b","a0");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),LOG(0),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_a1() {
    let bytecode = format!("0x600860018080{}565b","a1");
    check(&bytecode, &[PUSH(vec![0x08]),PUSH(vec![0x01]),DUP(1),DUP(1),LOG(1),JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_a2() {
    let bytecode = format!("0x60096001808080{}565b","a2");
    check(&bytecode, &[PUSH(vec![0x09]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),LOG(2),JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_a3() {
    let bytecode = format!("0x600a600180808080{}565b","a3");
    check(&bytecode, &[PUSH(vec![0x0a]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),DUP(1),LOG(3),JUMP,JUMPDEST(0xa)]);
}

#[test]
pub fn test_disassemble_insn_a4() {
    let bytecode = format!("0x600b60018080808080{}565b","a4");
    check(&bytecode, &[PUSH(vec![0x0b]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),LOG(4),JUMP,JUMPDEST(0xb)]);
}

// f0s: System Operations

#[test]
pub fn test_disassemble_insn_f0() {
    let bytecode = format!("0x600960018080{}50565b","f0");
    check(&bytecode, &[PUSH(vec![0x09]),PUSH(vec![0x01]),DUP(1),DUP(1),CREATE,POP,JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_f1() {
    let bytecode = format!("0x600d6001808080808080{}50565b","f1");
    check(&bytecode, &[PUSH(vec![0x0d]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),CALL,POP,JUMP,JUMPDEST(0xd)]);
}

#[test]
pub fn test_disassemble_insn_f2() {
    let bytecode = format!("0x600d6001808080808080{}50565b","f2");
    check(&bytecode, &[PUSH(vec![0x0d]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),CALLCODE,POP,JUMP,JUMPDEST(0xd)]);
}

#[test]
pub fn test_disassemble_insn_f3() {
    let bytecode = format!("0x6007600180{}56","f3");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),RETURN,DATA(vec![0x56])]);
}

#[test]
pub fn test_disassemble_insn_f4() {
    let bytecode = format!("0x600c60018080808080{}50565b","f4");
    check(&bytecode, &[PUSH(vec![0x0c]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),DELEGATECALL,POP,JUMP,JUMPDEST(0xc)]);
}

#[test]
pub fn test_disassemble_insn_f5() {
    let bytecode = format!("0x600a6001808080{}50565b","f5");
    check(&bytecode, &[PUSH(vec![0x0a]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),CREATE2,POP,JUMP,JUMPDEST(0xa)]);
}

#[test]
pub fn test_disassemble_insn_fa() {
    let bytecode = format!("0x600c60018080808080{}50565b","fa");
    check(&bytecode, &[PUSH(vec![0x0c]),PUSH(vec![0x01]),DUP(1),DUP(1),DUP(1),DUP(1),DUP(1),STATICCALL,POP,JUMP,JUMPDEST(0xc)]);
}

#[test]
pub fn test_disassemble_insn_fd() {
    let bytecode = format!("0x6007600180{}56","fd");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),REVERT,DATA(vec![0x56])]);
}

#[test]
pub fn test_disassemble_insn_fe() {
    let bytecode = format!("0x6007600180{}56","fe");
    check(&bytecode, &[PUSH(vec![0x07]),PUSH(vec![0x01]),DUP(1),INVALID,DATA(vec![0x56])]);
}

#[test]
pub fn test_disassemble_insn_ff() {
    let bytecode = format!("0x60066001{}565b","ff");
    check(&bytecode, &[PUSH(vec![0x06]),PUSH(vec![0x01]),SELFDESTRUCT,JUMP,JUMPDEST(6)]);
}

// ============================================================================
// Single block Tests
// ============================================================================

// more complex things here?

// ============================================================================
// Double block Tests
// ============================================================================

#[test]
pub fn test_disassemble_double_01() {
    // A minimal two-block program
    check("0x6003565b", &[PUSH(vec![3]),JUMP,JUMPDEST(3)]);
}

#[test]
pub fn test_disassemble_double_03() {
    // A minimal conditional two-block program
    check("0x60016005575b", &[PUSH(vec![1]),PUSH(vec![5]),JUMPI,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_double_04() {
    // A simple conditional two-block program
    check("0x6001600657005b", &[PUSH(vec![1]),PUSH(vec![6]),JUMPI,STOP,JUMPDEST(6)]);
}

#[test]
pub fn test_disassemble_double_05() {
    // A minimal example requiring different stack heights
    check("0x60ff600054600957505b6000", &[PUSH(vec![0xff]),PUSH(vec![0x00]),SLOAD,PUSH(vec![0x9]),JUMPI,POP,JUMPDEST(9),PUSH(vec![0x0])]);
}

// ============================================================================
// Triple block Tests
// ============================================================================

#[test]
pub fn test_disassemble_triple_01() {
    // Minimal three-block program
    check("0x6003565b6007565b", &[PUSH(vec![3]),JUMP,JUMPDEST(3),PUSH(vec![7]),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_triple_02() {
    // Three-block program with back jump
    check("0x6005565b005b600356", &[PUSH(vec![5]),JUMP,JUMPDEST(3),STOP,JUMPDEST(5),PUSH(vec![3]),JUMP]);
}

// ============================================================================
// Split block Tests
// ============================================================================

#[test]
pub fn test_disassemble_split_01() {
    // A minimal split multiblock program
    check("0x600456005b", &[PUSH(vec![4]),JUMP,DATA(vec![0]),JUMPDEST(4)]);
}

#[test]
pub fn test_disassemble_split_02() {
    // A minimal split multiblock program.  This program contains an
    // invalid JUMPDEST.
    check("0x600456605b", &[PUSH(vec![4]),JUMP,PUSH(vec![0x5b])]);
}

#[test]
pub fn test_disassemble_split_03() {
    // A minimal split multiblock program
    check("0x6003565b0061", &[PUSH(vec![3]),JUMP,JUMPDEST(3),STOP,DATA(vec![0x61])]);
}

#[test]
pub fn test_disassemble_split_04() {
    // A minimal split multiblock program
    check("0x60055601025b", &[PUSH(vec![5]),JUMP,DATA(vec![1,2]),JUMPDEST(5)]);
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that disassembling a given hex string produces a given
/// sequence of instructions.
fn check(hex: &str, insns: &[Instruction]) {
    // Parse hex string into bytes
    let bytes = hex.from_hex_string().unwrap();
    // Disassemble bytes into instructions
    let disasm : Disassembly<CfaState> = Disassembly::new(&bytes).build();
    // Check against expected instruction sequence
    assert_eq!(insns, disasm.to_vec());
}
