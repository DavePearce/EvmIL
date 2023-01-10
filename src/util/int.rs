use std::fmt;
use crate::util;

const ZERO : u256 = u256::from_u64(0);

///
#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
#[allow(non_camel_case_types)]
pub struct u256 {
    /// Represented in little endian notation.
    words: [u64;4]
}

impl u256 {
    pub const fn from_u64(val:u64) -> Self {
        u256{words:[val,0,0,0]}
    }
    /// Convert a given byte array into a u256.  The array is expected
    /// to be at most 32bytes long.
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        assert!(n <= 32);
        let lw;
        let hw;
        //
        if n >= 16 {
            let m = n - 16;
            lw = util::from_be_bytes(&bytes[m..]);
            hw = util::from_be_bytes(&bytes[..m]);
        } else {
            lw = util::from_be_bytes(bytes);
            hw = 0;
        }
        let w0 = lw as u64;
        let w1 = (lw >> 64) as u64;
        let w2 = hw as u64;
        let w3 = (hw >> 64) as u64;
        u256{words:[w0,w1,w2,w3]}
    }
}

impl From<u64> for u256 {
    fn from(val: u64) -> u256 {
        u256{words:[val,0,0,0]}
    }
}

impl From<&[u8]> for u256 {
    fn from(bytes: &[u8]) -> u256 {
        assert!(bytes.len() > 0);
        assert!(bytes.len() <= 1);
        // HACK for now
        let w1 = bytes[0] as u64;
        //
        u256{words:[w1,0,0,0]}
    }
}

impl Into<u16> for u256 {
    fn into(self) -> u16 {
        self.words[0] as u16
    }
}

// =====================================================================
// Arithmetic Operators
// =====================================================================

impl std::ops::Add for u256 {
    type Output=Self;

    fn add(self, rhs: Self) -> Self {
        let w0 = self.words[0];
        let (r,c) = w0.overflowing_add(rhs.words[0]);
        if c {
            // overflow detected
            panic!("fix u256 addition!");
        }
        //
        u256{words:[r,0,0,0]}
    }
}

// =====================================================================
// Formatting
// =====================================================================

impl fmt::Display for u256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.words[0])
    }
}

impl fmt::LowerHex for u256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        panic!("Implement me");
    }
}
