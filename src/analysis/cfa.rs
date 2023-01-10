// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::{fmt};
use crate::ll::{Instruction,Instruction::*};
use crate::analysis::{AbstractState};
use crate::analysis::{AbstractValue,AbstractStack,BOTTOM_STACK,EMPTY_STACK};
use crate::util;
use crate::util::u256;

const MAX_CODE_SIZE : u256 = u256::from_u64(24576);
const UNKNOWN : AbstractValue = AbstractValue::Unknown;

// ============================================================================
// Disassembly Context
// ============================================================================

#[derive(Debug,PartialEq)]
pub struct CfaState {
    stack: AbstractStack
}

impl CfaState {
    pub fn new(stack: AbstractStack) -> Self {
        // Done
        Self{stack}
    }
    pub fn is_bottom(&self) -> bool {
        self.stack.is_bottom()
    }
    /// Access the stack component of this abstract EVM.
    pub fn stack(&self) -> &AbstractStack{
        &self.stack
    }
    pub fn push(self, val: AbstractValue) -> Self {
        CfaState::new(self.stack.push(val))
    }
    pub fn pop(self, n: usize) -> Self {
        assert!(n > 0);
        let mut stack = self.stack;
        for _i in 0..n {
            stack = stack.pop();
        }
        CfaState::new(stack)
    }
    pub fn set(self, n:usize, val: AbstractValue) -> Self {
        CfaState::new(self.stack.set(n,val))
    }
}

impl Clone for CfaState {
    fn clone(&self) -> Self {
        CfaState::new(self.stack.clone())
    }
}

impl fmt::Display for CfaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.stack)
    }
}

impl AbstractState for CfaState {
    fn is_reachable(&self) -> bool { !self.stack.is_bottom() }

    fn branch(&self, _pc: usize, insn: &Instruction) -> Self {
        match insn {
            JUMPI => self.clone().pop(2),
            JUMP => self.clone().pop(1),
            _ => {
                unreachable!()
            }
        }
    }

    fn peek(&self, n: usize) -> AbstractValue {
        self.stack.peek(n)
    }

    fn merge(&mut self, other: Self) -> bool {
        if *self != other {
            if !other.is_bottom() {
                if self.is_bottom() {
                    *self = other;
                    return true;
                } else {
                    return self.stack.merge_into(&other.stack);
                }
            }
        }
        false
    }

    fn bottom() -> Self { CfaState::new(BOTTOM_STACK) }

    fn origin() -> Self {
        CfaState::new(EMPTY_STACK)
    }

    // ============================================================================
    // Abstract Instruction Semantics (stack)
    // ============================================================================

    /// Update an abstract state with the effects of a given instruction.
    fn transfer(self, insn: &Instruction) -> CfaState {
        match insn {
            STOP => CfaState::bottom(),
            // 0s: Stop and Arithmetic Operations
            ADD|MUL|SUB|DIV|SDIV|MOD|SMOD|EXP|SIGNEXTEND => {
                self.pop(2).push(UNKNOWN)
            }
            ADDMOD|MULMOD => {
                self.pop(3).push(UNKNOWN)
            }
            // 0s: Stop and Arithmetic Operations
            ISZERO|NOT => {
                self.pop(1).push(UNKNOWN)
            }
            // Binary Comparators
            LT|GT|SLT|SGT|EQ => {
                self.pop(2).push(UNKNOWN)
            }
            // Binary bitwise operators
            AND|OR|XOR|BYTE|SHL|SHR|SAR => {
                self.pop(2).push(UNKNOWN)
            }
            // 20s: Keccak256
            KECCAK256 => {
                // NOTE: there is some kind of compiler bug which is
                // preventing me from putting this case in the
                // expected position.
                self.pop(2).push(UNKNOWN)
            }
            // 30s: Environmental Information
            ADDRESS => self.push(UNKNOWN),
            BALANCE => self.pop(1).push(UNKNOWN),
            ORIGIN => self.push(UNKNOWN),
            CALLER => self.push(UNKNOWN),
            CALLVALUE => self.push(UNKNOWN),
            CALLDATALOAD => self.pop(1).push(UNKNOWN),
            CALLDATASIZE => self.push(UNKNOWN),
            CALLDATACOPY => self.pop(3),
            CODESIZE => self.push(UNKNOWN),
            CODECOPY => self.pop(3),
            GASPRICE => self.push(UNKNOWN),
            EXTCODESIZE => self.pop(1).push(UNKNOWN),
            EXTCODECOPY => self.pop(4),
            RETURNDATASIZE => self.push(UNKNOWN),
            RETURNDATACOPY => self.pop(3),
            EXTCODEHASH => self.pop(1).push(UNKNOWN),
            // 40s: Block Information
            BLOCKHASH => self.pop(1).push(UNKNOWN),
            COINBASE => self.push(UNKNOWN),
            TIMESTAMP => self.push(UNKNOWN),
            NUMBER   => self.push(UNKNOWN),
            DIFFICULTY => self.push(UNKNOWN),
            GASLIMIT => self.push(UNKNOWN),
            CHAINID => self.push(UNKNOWN),
            SELFBALANCE => self.push(UNKNOWN),
            // 50s: Stack, Memory, Storage and Flow Operations
            POP => self.pop(1),
            MLOAD => self.pop(1).push(UNKNOWN),
            MSTORE|MSTORE8 => self.pop(2),
            SLOAD => self.pop(1).push(UNKNOWN),
            SSTORE => self.pop(2),
            JUMPI => self.pop(2),
            PC|MSIZE|GAS => self.push(UNKNOWN),
            JUMPDEST(_) => self, // nop
            // 60 & 70s: Push Operations
            PUSH(bytes) => {
                let n = u256::from_be_bytes(&bytes);
                if n <= MAX_CODE_SIZE {
                    self.push(AbstractValue::Known(n))
                } else {
                    self.push(UNKNOWN)
                }
            }
            // 80s: Duplicate Operations
            DUP(n) => {
                let m = (*n - 1) as usize;
                let nth = self.peek(m);
                self.push(nth)
            }
            // 90s: Exchange Operations
            SWAP(n) => {
                let m = *n as usize;
                let x = self.peek(m);
                let y = self.peek(0);
                self.set(0,x).set(m,y)
            }
            // a0s: Logging Operations
            LOG(n) => {
                self.pop((n+2) as usize)
            }
            // f0s: System Operations
            CREATE => self.pop(3).push(UNKNOWN),
            CALL|CALLCODE => self.pop(7).push(UNKNOWN),
            DELEGATECALL|STATICCALL => self.pop(6).push(UNKNOWN),
            CREATE2 => self.pop(4).push(UNKNOWN),
            INVALID|JUMP|RETURN|REVERT => {
                CfaState::bottom()
            }
            SELFDESTRUCT => self.pop(1),
            _ => {
                // This is a catch all to ensure no instructions are
                // missed above.
                panic!("unknown instruction ({:?})",insn);
            }
        }
    }
}
