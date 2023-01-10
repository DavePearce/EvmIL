use std::fmt::Write;
use std::num::ParseIntError;

/// A simple trait allowing something to be converted into a hex
/// string.
pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

/// A simple trait allowing something to be converted from a hex
/// string.
pub trait FromHexString {
    type Error;

    fn from_hex_string(&self) -> Result<Vec<u8>,Self::Error>;
}

/// A default implementation for byte slices.
impl ToHexString for [u8] {
    fn to_hex_string(&self) -> String {
        let size = 2 + (2 * self.len());
        let mut hexstr = String::with_capacity(size);
        // Prepend "0x"
        write!(hexstr,"0x").unwrap();
        // Write each byte
        for b in self { write!(hexstr, "{:02x}", b).unwrap(); }
        // Done
        hexstr
    }
}

/// A default implementation for string slices
impl FromHexString for str {
    type Error = ParseIntError;
    //
    fn from_hex_string(&self) -> Result<Vec<u8>,Self::Error> {
	let mut bytes : Vec<u8> = Vec::new();
        // Remove prepended "0x" (only if present)
        let slice = if self.len() > 2 && &self[0..2] == "0x" {
            &self[2..]
        } else {
            &self
        };
        // Parse contents of string
	for i in (0..slice.len()).step_by(2) {
	    // Pull out the byte
	    let byte = u8::from_str_radix(&slice[i..i+2], 16)?;
	    // Push it!
	    bytes.push(byte);
	}
	//
	Ok(bytes)
    }
}
