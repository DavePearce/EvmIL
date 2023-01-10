use evmil::util::u256;
use evmil::evm;
use evmil::evm::opcode::*;
use evmil::evm::ConcreteEvm;
use evmil::evm::ConcreteResult::*;

const EMPTY_BYTES : Vec<u8> = Vec::new();

#[test]
fn test_evm_01() {
    let output = ConcreteEvm::new(&[PUSH1,0x1,PUSH1,0x2,ADD,STOP]).run();
    assert_eq!(Return{data:EMPTY_BYTES}, output);
}
