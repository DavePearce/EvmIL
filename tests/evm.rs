use evmil::util::u256;
use evmil::evm;
use evmil::evm::opcode::*;
use evmil::evm::concrete::{Stack};

/// Define a concrete EVM
type Evm<'a> = evm::Evm<'a,u256,Stack<u256>>;

#[test]
fn test_evm_01() {
    Evm::new(&[PUSH1,0x1,PUSH1,0x2,ADD,STOP]).run();
}
