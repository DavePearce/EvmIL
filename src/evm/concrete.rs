use crate::evm::{Evm,Stack,Stepable,Word};
use crate::evm::opcode::*;
use crate::util::u256;

// ===================================================================
// Concrete Stack
// ===================================================================

/// A concrete stack implementation backed by a `Vec`.
#[derive(Debug,PartialEq)]
pub struct ConcreteStack<T> {
    items: Vec<T>
}

impl<T:Word> ConcreteStack<T> {
    pub fn new(items: &[T]) -> Self {
        ConcreteStack{items: items.to_vec()}
    }
}

impl<T:Word> Default for ConcreteStack<T> {
    fn default() -> Self {
        ConcreteStack{items:Vec::new()}
    }
}

impl<T:Word> Stack<T> for ConcreteStack<T> {

    fn peek(&self, n:usize) -> T {
        let i = self.items.len() - n;
        self.items[i-1]
    }

    fn len(&self) -> T {
        // FIXME: broken for non-64bit architectures!
        let w : u256 = (self.items.len() as u64).into();
        // Convert into word
        w.into()
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self, n: usize) {
        for _i in 0..n { self.items.pop();}
    }
}

// ===================================================================
// Concrete EVM
// ===================================================================

#[derive(Debug,PartialEq)]
pub enum ConcreteResult<'a> {
    Continue(ConcreteEvm<'a>),
    Return{data:Vec<u8>},
    Revert{data:Vec<u8>}
}

pub type ConcreteEvm<'a> = Evm<'a,u256,ConcreteStack<u256>>;

impl<'a> ConcreteEvm<'a> {
    /// Execute the contract to completion.
    pub fn run(mut self) -> ConcreteResult<'a> {
        // Eventually, this needs a return type.
        loop {
            let r = self.step();

            match r {
                ConcreteResult::Continue(evm) => {
                    self = evm;
                }
                _ => {
                    return r;
                }
            }
        }
    }
}

impl<'a> Stepable for ConcreteEvm<'a> {
    type Result = ConcreteResult<'a>;

    /// Execute instruction at the current `pc`.
    fn step(self) -> Self::Result {
        let opcode = self.code[self.pc];
        //
        match opcode {
            STOP => Self::Result::Return{data:Vec::new()},
            //
            ADD => {
                let lhs = self.stack.peek(1);
                let rhs = self.stack.peek(0);
                Self::Result::Continue(self.pop(2).push(lhs + rhs).next(1))
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
                Self::Result::Continue(self.push(w.into()).next(n+1))
            }
            //
            _ => {
                panic!("unknown instruction encountered");
            }
        }
    }
}
