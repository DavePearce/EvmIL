use std::fmt;
use crate::util::ToHexString;

// ============================================================================
// Label Offsets
// ============================================================================

/// Used to simplify calculation of label offsets.
#[derive(PartialEq,Copy,Clone)]
pub struct Offset(pub u16);

impl Offset {
    /// Determine the width of this offset (in bytes).
    pub fn width(&self) -> u16 {
        if self.0 > 255 { 2 } else { 1 }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0 > 255 {
            vec![(self.0/256) as u8,(self.0%256) as u8]
        } else {
            vec![self.0 as u8]
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum Error {
    /// A push instruction cannot push zero bytes and, likewise,
    /// cannot push more than 32 bytes.
    InvalidPush,
    /// A dup `n` instruction requires `n > 0` and `n <= 32`.
    InvalidDup,
    /// A label cannot exceed the 24Kb limit imposed by the EVM.
    InvalidLabelOffset
}

// ============================================================================
// Bytecode Instructions
// ============================================================================

#[derive(Debug,PartialEq)]
pub enum Instruction {
    // 0s: Stop and Arithmetic Operations
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    // 10s: Comparison & Bitwise Logic Operations
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    // 20s: Keccak256
    KECCAK256,
    // 30s: Environmental Information
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    // 40s: Block Information
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    // 50s: Stack, Memory, Storage and Flow Operations
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST(usize),
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    PUSHL(usize), // Push label offset.
    // 80s: Duplicate Operations
    DUP(u8),
    // 90s: Exchange Operations
    SWAP(u8),
    // a0s: Logging Operations
    LOG(u8),
    // f0s: System Operations
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
    // Signals arbitrary data in the contract, rather than bytecode
    // instructions.
    DATA(Vec<u8>)
}

impl Instruction {
    /// Determine whether or not control can continue to the next
    /// instruction.
    pub fn fallthru(&self) -> bool {
        match self {
            Instruction::DATA(_) => false,
            Instruction::INVALID => false,
            Instruction::JUMP => false,
            Instruction::STOP => false,
            Instruction::RETURN => false,
            Instruction::REVERT => false,
            _ => true
        }
    }

    /// Determine whether or not this instruction can branch.  That
    /// is, whether or not it is a `JUMP` or `JUMPI` instruction.
    pub fn can_branch(&self) -> bool {
        match self {
            Instruction::JUMP => true,
            Instruction::JUMPI => true,
            _ => false,
        }
    }

    /// Encode an instruction into a byte sequence, assuming a given
    /// set of label offsets.
    pub fn encode(&self, offsets: &[Offset], bytes: &mut Vec<u8>) -> Result<(),Error> {
        // Push opcode
        bytes.push(self.opcode(&offsets)?);
        // Push operands (if applicable)
        match self {
            Instruction::PUSH(args) => {
                bytes.extend(args);
            }
            Instruction::PUSHL(idx) => {
                bytes.extend(offsets[*idx].to_bytes());
            }
            _ => {
                // All other instructions have no operands.
            }
        }
        //
        Ok(())
    }

    /// Determine the length of this instruction (in bytes) assuming a
    /// given set of label offsets.
    pub fn length(&self, _offsets: &[Offset]) -> usize {
        let operands = match self {
            Instruction::DATA(bytes) => bytes.len()-1,
            // Push instructions
            Instruction::PUSH(bs) => bs.len(),
            Instruction::PUSHL(_) => {
                todo!("implement me");
            }
            // Default case
            _ => 0
        };
        operands + 1
    }

    /// Determine the opcode for a given instruction.  In many cases,
    /// this is a straightforward mapping.  However, in other cases,
    /// its slightly more involved as a calculation involving the
    /// operands is required.
    pub fn opcode(&self, offsets: &[Offset]) -> Result<u8,Error> {
        let op = match self {
            // 0s: Stop and Arithmetic Operations
            Instruction::STOP => 0x00,
            Instruction::ADD => 0x01,
            Instruction::MUL => 0x02,
            Instruction::SUB => 0x03,
            Instruction::DIV => 0x04,
            Instruction::SDIV => 0x05,
            Instruction::MOD => 0x06,
            Instruction::SMOD => 0x07,
            Instruction::ADDMOD => 0x08,
            Instruction::MULMOD => 0x09,
            Instruction::EXP => 0x0a,
            Instruction::SIGNEXTEND => 0x0b,
            // 10s: Comparison & Bitwise Logic Operations
            Instruction::LT => 0x10,
            Instruction::GT => 0x11,
            Instruction::SLT => 0x12,
            Instruction::SGT => 0x13,
            Instruction::EQ => 0x14,
            Instruction::ISZERO => 0x15,
            Instruction::AND => 0x16,
            Instruction::OR => 0x17,
            Instruction::XOR => 0x18,
            Instruction::NOT => 0x19,
            Instruction::BYTE => 0x1a,
            Instruction::SHL => 0x1b,
            Instruction::SHR => 0x1c,
            Instruction::SAR => 0x1d,
            // 20s: Keccak256
            Instruction::KECCAK256 => 0x20,
            // 30s: Environmental Information
            Instruction::ADDRESS => 0x30,
            Instruction::BALANCE => 0x31,
            Instruction::ORIGIN => 0x32,
            Instruction::CALLER => 0x33,
            Instruction::CALLVALUE => 0x34,
            Instruction::CALLDATALOAD => 0x35,
            Instruction::CALLDATASIZE => 0x36,
            Instruction::CALLDATACOPY => 0x37,
            Instruction::CODESIZE => 0x38,
            Instruction::CODECOPY => 0x39,
            Instruction::GASPRICE => 0x3a,
            Instruction::EXTCODESIZE => 0x3b,
            Instruction::EXTCODECOPY => 0x3c,
            Instruction::RETURNDATASIZE => 0x3d,
            Instruction::RETURNDATACOPY => 0x3e,
            Instruction::EXTCODEHASH => 0x3f,
            // 40s: Block Information
            Instruction::BLOCKHASH => 0x40,
            Instruction::COINBASE => 0x41,
            Instruction::TIMESTAMP => 0x42,
            Instruction::NUMBER => 0x43,
            Instruction::DIFFICULTY => 0x44,
            Instruction::GASLIMIT => 0x45,
            Instruction::CHAINID => 0x46,
            Instruction::SELFBALANCE => 0x47,
            // 50s: Stack, Memory, Storage and Flow Operations
            Instruction::POP => 0x50,
            Instruction::MLOAD => 0x51,
            Instruction::MSTORE => 0x52,
            Instruction::MSTORE8 => 0x53,
            Instruction::SLOAD => 0x54,
            Instruction::SSTORE => 0x55,
            Instruction::JUMP => 0x56,
            Instruction::JUMPI => 0x57,
            Instruction::PC => 0x58,
            Instruction::MSIZE => 0x59,
            Instruction::GAS => 0x5a,
            Instruction::JUMPDEST(_) => 0x5b,
            //
            // 60s & 70s: Push Operations
            Instruction::PUSH(bs) => {
                if bs.len() == 0 || bs.len() > 32 {
                    return Err(Error::InvalidPush);
                } else {
                    (0x5f + bs.len()) as u8
                }
            }
            //
            Instruction::PUSHL(lab) => {
                let offset = &offsets[*lab];
                if offset.width() == 2 { 0x61 }
                else { 0x60 }
            }
            // 80s: Duplication Operations
            Instruction::DUP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                0x7f + n
            }
            // 90s: Swap Operations
            Instruction::SWAP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                0x8f + n
            }
            // a0s: Log Operations
            Instruction::LOG(n) => {
                if *n > 4 {
                    return Err(Error::InvalidDup);
                }
                0xa0 + n
            }
            // f0s: System Operations
            Instruction::CREATE => 0xf0,
            Instruction::CALL => 0xf1,
            Instruction::CALLCODE => 0xf2,
            Instruction::RETURN => 0xf3,
            Instruction::DELEGATECALL => 0xf4,
            Instruction::CREATE2 => 0xf5,
            Instruction::STATICCALL => 0xfa,
            Instruction::REVERT => 0xfd,
            Instruction::INVALID => 0xfe,
            Instruction::SELFDESTRUCT => 0xff,
            //
            Instruction::DATA(_) => {
                 panic!("Invalid instruction ({:?})",self);
            }
        };
        //
        Ok(op)
    }

    /// Decode the next instruction in a given sequence of bytes.
    /// Observe that this never returns a `PUSHL` instruction.  This
    /// is because it cannot determine whether a given operand will be
    /// used as a jump destination.  A separate analysis is required
    /// to "lift" `PUSH` instructions to `PUSHL` instructions.
    pub fn decode(pc: usize, bytes: &[u8]) -> Instruction {
        let opcode = bytes[pc];
        //
        let insn = match opcode {
            // 0s: Stop and Arithmetic Operations
            0x00 => Instruction::STOP,
            0x01 => Instruction::ADD,
            0x02 => Instruction::MUL,
            0x03 => Instruction::SUB,
            0x04 => Instruction::DIV,
            0x05 => Instruction::SDIV,
            0x06 => Instruction::MOD,
            0x07 => Instruction::SMOD,
            0x08 => Instruction::ADDMOD,
            0x09 => Instruction::MULMOD,
            0x0a => Instruction::EXP,
            0x0b => Instruction::SIGNEXTEND,
            // 10s: Comparison & Bitwise Logic Operations
            0x10 => Instruction::LT,
            0x11 => Instruction::GT,
            0x12 => Instruction::SLT,
            0x13 => Instruction::SGT,
            0x14 => Instruction::EQ,
            0x15 => Instruction::ISZERO,
            0x16 => Instruction::AND,
            0x17 => Instruction::OR,
            0x18 => Instruction::XOR,
            0x19 => Instruction::NOT,
            0x1a => Instruction::BYTE,
            0x1b => Instruction::SHL,
            0x1c => Instruction::SHR,
            0x1d => Instruction::SAR,
            // 20s: SHA3
            0x20 => Instruction::KECCAK256,
            // 30s: Environmental Information
            0x30 => Instruction::ADDRESS,
            0x31 => Instruction::BALANCE,
            0x32 => Instruction::ORIGIN,
            0x33 => Instruction::CALLER,
            0x34 => Instruction::CALLVALUE,
            0x35 => Instruction::CALLDATALOAD,
            0x36 => Instruction::CALLDATASIZE,
            0x37 => Instruction::CALLDATACOPY,
            0x38 => Instruction::CODESIZE,
            0x39 => Instruction::CODECOPY,
            0x3a => Instruction::GASPRICE,
            0x3b => Instruction::EXTCODESIZE,
            0x3c => Instruction::EXTCODECOPY,
            0x3d => Instruction::RETURNDATASIZE,
            0x3e => Instruction::RETURNDATACOPY,
            0x3f => Instruction::EXTCODEHASH,
            // 40s: Block Information
            0x40 => Instruction::BLOCKHASH,
            0x41 => Instruction::COINBASE,
            0x42 => Instruction::TIMESTAMP,
            0x43 => Instruction::NUMBER,
            0x44 => Instruction::DIFFICULTY,
            0x45 => Instruction::GASLIMIT,
            0x46 => Instruction::CHAINID,
            0x47 => Instruction::SELFBALANCE,
            // 50s: Stack, Memory, Storage and Flow Operations
            0x50 => Instruction::POP,
            0x51 => Instruction::MLOAD,
            0x52 => Instruction::MSTORE,
            0x53 => Instruction::MSTORE8,
            0x54 => Instruction::SLOAD,
            0x55 => Instruction::SSTORE,
            0x56 => Instruction::JUMP,
            0x57 => Instruction::JUMPI,
            0x58 => Instruction::PC,
            0x59 => Instruction::MSIZE,
            0x5a => Instruction::GAS,
            0x5b => Instruction::JUMPDEST(pc),
            // 60s & 70s: Push Operations
            0x60..=0x7f => {
                let m = pc + 1;
                let n = pc + ((opcode - 0x5e) as usize);
                if n <= bytes.len() {
                    // Simple case: does not overflow
                    Instruction::PUSH(bytes[m..n].to_vec())
                } else {
                    // Harder case: does overflow code.
                    let mut bs = bytes[m..].to_vec();
                    // Pad out with zeros
                    for _i in 0..(n-bytes.len()) { bs.push(0); }
                    // Done
                    Instruction::PUSH(bs)
                }
            }
            // 80s: Duplicate Operations
            0x80..=0x8f => {
                Instruction::DUP(opcode-0x7f)
            }
            // 90s: Swap Operations
            0x90..=0x9f => {
                Instruction::SWAP(opcode-0x8f)
            }
            // a0s: Log Operations
            0xa0..=0xa4 => {
                Instruction::LOG(opcode-0xa0)
            }
            // f0s: System Operations
            0xf0 => Instruction::CREATE,
            0xf1 => Instruction::CALL,
            0xf2 => Instruction::CALLCODE,
            0xf3 => Instruction::RETURN,
            0xf4 => Instruction::DELEGATECALL,
            0xf5 => Instruction::CREATE2,
            0xfa => Instruction::STATICCALL,
            0xfd => Instruction::REVERT,
            0xfe => Instruction::INVALID,
            0xff => Instruction::SELFDESTRUCT,
            // Unknown
            _ => {
                Instruction::DATA(vec![opcode])
            }
        };
        //
        insn
    }
}


// ============================================================================
// Display
// ============================================================================

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	// Use the default (debug) formatter.  Its only for certain
	// instructions that we need to do anything different.
	match self {
	    Instruction::PUSH(bytes) => {
		// Convert bytes into hex string
		let hex = bytes.to_hex_string();
		// Print!
		write!(f,"PUSH{} {}",bytes.len(),hex)
	    }
            Instruction::DATA(bytes) => {
                // Print bytes as hex string
		write!(f,"{}",bytes.to_hex_string())
            }
	    _ => write!(f,"{:?}",self)
	}
    }
}
