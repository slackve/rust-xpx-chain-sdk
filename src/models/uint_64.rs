use core::{fmt, mem};

use byteorder::{BigEndian, WriteBytesExt};

use models::utils::u64_to_array_u8;

/// Represents a 64-bit unsigned integer.
///
/// This class uses Dart's native number type `u64` and has a value check for big integers.
/// `u64` will be translated correctly into JavaScript (supported by dart2js).
/// Value range is 0 through 18446744073709551615.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]// we derive Default in order to use the clear() method in Drop
pub struct Uint64(pub(crate) u64);

impl Uint64 {
    /// Creates a `Uint64` from a pair of u32 integers.
    pub fn from_ints(lower: u32, higher: u32) -> Uint64 {
        let mut buf = [0u8; mem::size_of::<u64>()];
        buf[..4].as_mut().write_u32::<BigEndian>(higher).expect("Unable to write");
        buf[4..].as_mut().write_u32::<BigEndian>(lower).expect("Unable to write");
        Uint64::from_bytes(buf)
    }

    /// Creates a `Uint64` from a u8 array.
    pub fn from_bytes(mut b: [u8; 8]) -> Uint64 {
        Uint64(u64::from_be_bytes(b))
    }

    /// Creates a `Uint64` from a hex &str.
    pub fn from_hex(hex_code: &str) -> Result<Uint64, core::num::ParseIntError> {
        let r: u64 = u64::from_str_radix(&hex_code, 16)?;
        Ok(Uint64(r))
    }

    /// Converts to hex String representation.
    pub fn to_hex(&self) -> String {
        format!("{:X}", &self.0)
    }

    /// Converts to 64-bit byte array.
    pub fn to_bytes(&self) -> [u8; 8] {
        u64_to_array_u8(self.0)
    }

    /// Converts to a pair of i32 integers ([lower, higher]).
    pub fn to_int_array(&self) -> [u32; 2] {
        let lower = self.0 as u32;
        let higher = (self.0 >> 32) as u32;
        return [lower, higher];
    }
}

impl fmt::LowerHex for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        write!(f, "{:x}", val) // delegate to u64's implementation
    }
}

impl fmt::UpperHex for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        write!(f, "{:X}", val) // delegate to u64's implementation
    }
}

impl fmt::Binary for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        write!(f, "{:b}", val) // delegate to i32's implementation
    }
}

impl fmt::Display for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
