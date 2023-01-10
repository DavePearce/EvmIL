use std::{fmt};
use crate::evm;
use crate::util::u256;

// ============================================================================
// Abstract Word
// ============================================================================

/// An abstract value is either a known constant, or an unknown
/// (i.e. arbitrary value).
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum AbstractValue {
    Known(u256),
    Unknown
}

use AbstractValue::*;

impl AbstractValue {

    pub fn merge(self, other: AbstractValue) -> AbstractValue {
        if self == other {
            self
        } else {
            Unknown
        }
    }

    pub fn is_known(&self) -> bool {
        match self {
            Known(_) => true,
            Unknown => false
        }
    }

    pub fn unwrap(&self) -> u256 {
        match self {
            Known(n) => *n,
            Unknown => {
                panic!("unwrapping unknown value");
            }
        }
    }
}

impl fmt::Display for AbstractValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unknown => write!(f,"(??)"),
            Known(n) => write!(f,"({:#08x})",n)
        }
    }
}

impl From<u256> for AbstractValue {
    fn from(v:u256) -> AbstractValue {
        Known(v)
    }
}

impl std::ops::Add for AbstractValue {
    type Output=Self;

    fn add(self, rhs: Self) -> Self {
        match (self,rhs) {
            (Known(v),Known(w)) => Known(v+w),
            _ => Unknown
        }
    }
}


impl evm::Word for AbstractValue {

}
