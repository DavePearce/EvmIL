use std::fmt;
///
#[derive(Clone,Copy)]
pub struct u256 {
    /// Represented in little endian notation.
    words: [u64;4]
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

impl fmt::Display for u256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.words[0])
    }
}
