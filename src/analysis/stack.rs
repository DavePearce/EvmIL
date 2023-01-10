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
use std::{cmp,fmt,mem};
use crate::evm;
use crate::analysis::{AbstractValue};
use crate::util::{Interval,MAX_INTERVAL};

/// Represents the singleton set of empty abstract stacks (which is
/// distinct from empty set of stacks).
pub const EMPTY_STACK : AbstractStack = AbstractStack{lower: Interval::new_const(0,0), upper: Vec::new()};
/// Bottom represents the empty set of stacks.
pub const BOTTOM_STACK : AbstractStack = AbstractStack{lower: MAX_INTERVAL, upper: Vec::new()};

// ============================================================================
// Disassembly Context
// ============================================================================

#[derive(Debug,PartialEq)]
pub struct AbstractStack {
    // The lower segment of an abstract stack represents a variable
    // number of unknown values.  An interval is used for a compact
    // representation.  So, for example, `0..1` represents two
    // possible lower segments: `[]` and `[??]`.
    lower: Interval,
    // The upper segment represents zero or more concrete values on
    // the stack.
    upper: Vec<AbstractValue>
}

impl AbstractStack {
    pub fn new<T:Into<Interval>>(lower: T, upper: Vec<AbstractValue>) -> Self {
        // Convert input range into an interval
        let lower_iv = lower.into();
        // Sanity check (maximum) stack height
        assert!((lower_iv.end+upper.len()) <= 1024);
        // Done
        Self{lower:lower_iv,upper}
    }
    pub fn is_bottom(&self) -> bool {
        *self == BOTTOM_STACK
    }
    /// Determine possible lengths of the stack as an interval
    pub fn len(&self) -> Interval {
        self.lower.add(self.upper.len())
    }
    /// Determine the minimum length of any stack represented by this
    /// abstract stack.
    pub fn min_len(&self) -> usize {
        self.lower.start + self.upper.len()
    }
    /// Determine the maximum length of any stack represented by this
    /// abstract stack.
    pub fn max_len(&self) -> usize {
        self.lower.end + self.upper.len()
    }
    /// Access the array of concrete values represented by this stack
    /// (i.e. the _upper_ portion of the stack).
    pub fn values<'a>(&'a self) -> &'a [AbstractValue] {
        &self.upper
    }
    /// Push an iterm onto this stack.
    pub fn push(mut self, val: AbstractValue) -> Self {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        //
        if val == AbstractValue::Unknown && self.upper.len() == 0 {
            self.lower = self.lower.add(1);
        } else {
            // Pop target address off the stack.
            self.upper.push(val);
        }
        // Done
        self
    }
    /// Pop an item of this stack, producing an updated state.
    pub fn pop(mut self) -> Self {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        // Pop target address off the stack.
        if self.upper.is_empty() {
            self.lower = self.lower.sub(1);
        } else {
            self.upper.pop();
        }
        // Done
        self
    }
    /// Perk nth item on the stack (where `0` is top).
    pub fn peek(&self, n: usize) -> AbstractValue {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        // Get the nth value!
        if n < self.upper.len() {
            // Determine stack index
            let i = self.upper.len() - (1+n);
            // Extract value
            self.upper[i]
        } else {
            AbstractValue::Unknown
        }
    }

    /// Set `ith` item from the top on this stack.  Thus, `0` is the
    /// top of the stack, etc.
    pub fn set(mut self, n: usize, val: AbstractValue) -> Self {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        // NOTE: inefficient when putting unknown value into lower
        // portion.
        self.ensure_upper(n+1);
        // Determine stack index
        let i = self.upper.len() - (1+n);
        // Set value
        self.upper[i] = val;
        // Rebalance (which can be necessary is val unknown)
        self.rebalance()
    }

    /// Merge two abstract stacks together.
    pub fn merge(self, other: &AbstractStack) -> Self {
        let slen = self.upper.len();
        let olen = other.upper.len();
        // Determine common upper length
        let n = cmp::min(slen,olen);
        // Normalise lower segments
        let lself = self.lower.add(slen - n);
        let lother = other.lower.add(olen - n);
        let mut merger = AbstractStack::new(lself.union(&lother),Vec::new());
        // Push merged items from upper segment
        for i in (0..n).rev() {
            let ithself = self.peek(i);
            let ithother = other.peek(i);
            merger = merger.push(ithself.merge(ithother));
        }
        // Done
        merger
    }

    /// Merge an abstract stack into this stack, whilst reporting
    /// whether this stack changed or not.
    pub fn merge_into(&mut self, other: &AbstractStack) -> bool {
        // NOTE: this could be done more efficiently.
        let old = self.clone();
        let mut tmp = EMPTY_STACK;
        // Install dummy value to keep self alive
        mem::swap(self, &mut tmp);
        // Perform merge
        *self = tmp.merge(other);
        // Check for change
        *self != old
    }

    /// Rebalance the stack if necessary.  This is necessary when the
    /// upper portion contains unknown values which can be shifted
    /// into the lower portion.
    fn rebalance(mut self) -> Self {
        let mut i = 0;
        // Determine whether any rebalancing necessary.
        while i < self.upper.len() {
            if let AbstractValue::Known(_) = self.upper[i] {
                break;
            }
            i = i + 1;
        }
        // Rebalance only if necessary
        if i > 0 {
            // Increase lower portion
            self.lower = self.lower.add(i);
            // Decrease upper portion
            self.upper.drain(0..i);
        }
        //
        self
    }

    /// Ensure the upper portion has space for at least `n` elements.
    fn ensure_upper(&mut self, n: usize) {
        // FIXME: inefficient!!
        while n > self.upper.len() {
            self.upper.insert(0,AbstractValue::Unknown);
            self.lower = self.lower.sub(1);
        }
    }
}

// impl evm::Stack<AbstractValue> for AbstractStack {

// }

impl Clone for AbstractStack {
    fn clone(&self) -> Self {
        AbstractStack{lower:self.lower.clone(),upper:self.upper.clone()}
    }
}

impl fmt::Display for AbstractStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == BOTTOM_STACK {
            write!(f,"_|_")
        } else {
            write!(f,"({})[",self.lower)?;
            for i in 0..self.upper.len() {
                write!(f,"{}",self.upper[i])?;
            }
            write!(f,"]")
        }
    }
}
