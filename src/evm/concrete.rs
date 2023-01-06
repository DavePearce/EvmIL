use crate::evm;
use crate::evm::{Word};
use crate::util::u256;


/// A concrete stack implementation backed by a `Vec`.
pub struct Stack<T> {
    items: Vec<T>
}

impl<T:Word> Stack<T> {
    pub fn new(items: &[T]) -> Self {
        Stack{items: items.to_vec()}
    }
}

impl<T:Word> Default for Stack<T> {
    fn default() -> Self {
        Stack{items:Vec::new()}
    }
}

impl<T:Word> evm::Stack<T> for Stack<T> {

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
        for i in 0..n { self.items.pop();}
    }
}
