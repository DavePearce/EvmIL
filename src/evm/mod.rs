pub mod concrete;
pub mod opcode;

use std::marker::PhantomData;
use crate::evm::opcode::*;
use crate::util::u256;

/// Represents the fundamental unit of computation within the EVM,
/// namely a word.
pub trait Word : Sized + Copy + From<u256> + std::ops::Add<Output=Self> {

}

/// Default implementation for `u256`
impl Word for u256 { }

/// Represents the EVM stack.
pub trait Stack<T:Word> : Default {
    /// Peek `nth` item from stack (where `n==0` is top element).
    fn peek(&self, n:usize) -> T;

    /// Determine number of items on stack.
    fn len(&self) -> T;

    /// Push an item onto the stack.
    fn push(&mut self, item: T);

    /// Pop an item from the stack.
    fn pop(&mut self, n: usize);
}

pub struct Evm<'a,W:Word,S:Stack<W>> {
    // This is needed for some reason.
    phantom: PhantomData<W>,
    /// Program Counter
    pc: usize,
    /// Bytecode being executed
    code: &'a [u8],
    // Stack
    stack: S
}

impl<'a,W:Word,S> Evm<'a,W,S>
where S:Stack<W> {
    /// Construct a new EVM.
    pub fn new(code: &'a [u8]) -> Self {
        // Create default stack
        let stack = S::default();
        // Create EVM!
        Evm{phantom:PhantomData,pc:0,code,stack}
    }

    /// Pop `n` items of the stack.
    pub fn pop(mut self, n:usize) -> Self {
        self.stack.pop(n);
        self
    }

    /// Push a word onto the stack.
    pub fn push(mut self, word: W) -> Self {
        self.stack.push(word);
        self
    }

    /// Shift the `pc` by `n` bytes.
    pub fn next(mut self, n: usize) -> Self {
        self.pc = self.pc + n;
        self
    }

    /// Execute the contract to completion.
    pub fn run(mut self) {
        // Eventually, this needs a return type.
        loop {
            match self.step() {
                None => { return; }
                Some(evm) => {
                    self = evm;
                }
            }
        }
    }

    /// Execute instruction at the current `pc`.
    pub fn step(mut self) -> Option<Self> {
        let opcode = self.code[self.pc];
        //
        match opcode {
            STOP => None,
            //
            ADD => {
                let lhs = self.stack.peek(1);
                let rhs = self.stack.peek(0);
                Some(self.pop(2).push(lhs + rhs).next(1))
            }
            PUSH1..=PUSH32 => {
                // Determine push size
                let n = ((opcode - PUSH1) + 1) as usize;
                let pc = self.pc+1;
                // Extract bytes
                let bytes = &self.code[pc .. pc+n];
                // Convert bytes into u256 word
                let w : u256 = bytes.into();
                // Done
                Some(self.push(w.into()).next(n+1))
            }
            //
            _ => {
                panic!("unknown instruction encountered");
            }
        }
    }
}
