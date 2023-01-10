mod concrete;
pub mod opcode;

pub use crate::evm::concrete::*;

use std::marker::PhantomData;
use crate::util::u256;


/// Represents the fundamental unit of computation within the EVM,
/// namely a word.  This is intentially left abstract, so that it
/// could be reused across both _concrete_ and _abstract_ semantics.
pub trait Word : Sized +
    Copy +
    From<u256> +
    PartialEq +
    std::ops::Add<Output=Self> {

}

/// Default implementation for `u256`
impl Word for u256 { }

// ===================================================================
// Stack
// ===================================================================

/// Represents an EVM stack of some form.  This could a _concrete_
/// stack (i.e. useful for execution) or an _abstract_ stack
/// (i.e. useful for analysis).
pub trait Stack<T:Word> : Default+PartialEq {
    /// Peek `nth` item from stack (where `n==0` is top element).
    fn peek(&self, n:usize) -> T;

    /// Determine number of items on stack.
    fn len(&self) -> T;

    /// Push an item onto the stack.
    fn push(&mut self, item: T);

    /// Pop an item from the stack.
    fn pop(&mut self, n: usize);
}

// ===================================================================
// EVM
// ===================================================================

/// Represents an EVM of some form.  This could be a _concrete_ EVM
/// (i.e. useful for actually executing bytecodes), or an _abstract_
/// EVM (i.e. useful for some kind of dataflow analysis).
#[derive(Debug,PartialEq)]
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
}

/// A stepper is a trait for describing a single execution step of the
/// EVM.  This is subtle because it can be abstract or concrete.
pub trait Stepable {
    type Result;

    /// Take a single step of the EVM producing a result of some kind
    /// (e.g. an updated EVM state).
    fn step(self) -> Self::Result;
}
