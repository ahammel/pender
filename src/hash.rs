use std::fmt::{Debug, Formatter, Error};
use std::fmt::Write;

use blake2_rfc::blake2b::blake2b;

pub struct Hash {
    pub bytes: [u8; 64]
}

/// 64-byte blake2b hash of a byte string
impl Hash {
    pub fn new(obj: &[u8]) -> Hash {
        Hash {  bytes: into_64bytes(blake2b(64, b"a key", obj).as_bytes()) }
    }
}

impl Copy for Hash { }

impl Clone for Hash {
    fn clone(&self) -> Hash {
        *self
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Hash) -> bool {
        self.bytes.iter()
            .zip(other.bytes.iter())
            .all(|(x, y)| x == y)
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut hex = String::new();
        for &byte in self.bytes.iter() {
            write!(&mut hex, "{:X}", byte).unwrap();
        }
        write!(f, "Hash {{ bytes: \"{}\" }}", hex)
    }
}

fn into_64bytes(slice: &[u8]) -> [u8; 64] {
    if slice.len() != 64 {
        panic!("Slice is not 64 bytes")
    }

    let mut array = [0u8; 64];
    for (x, y) in slice.iter().zip(array.iter_mut()) {
        *y = *x;
    }
    array
}

