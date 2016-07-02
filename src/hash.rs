use std::fmt::{Debug, Formatter, Error};
use std::fmt::Write;

use blake2_rfc::blake2b::blake2b;

pub struct Blake2 {
    pub bytes: [u8; 64]
}

/// 64-byte blake2b hash of a byte string
impl Blake2 {
    pub fn new(obj: &[u8]) -> Blake2 {
        Blake2 {  bytes: into_64bytes(blake2b(64, b"a key", obj).as_bytes()) }
    }
}

impl Copy for Blake2 { }

impl Clone for Blake2 {
    fn clone(&self) -> Blake2 {
        *self
    }
}

impl PartialEq for Blake2 {
    fn eq(&self, other: &Blake2) -> bool {
        self.bytes.iter()
            .zip(other.bytes.iter())
            .all(|(x, y)| x == y)
    }
}

impl Eq for Blake2 { }

impl Debug for Blake2 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut hex = String::new();
        for &byte in self.bytes.iter() {
            write!(&mut hex, "{:X}", byte).unwrap();
        }
        write!(f, "Blake2 {{ bytes: \"{}\" }}", hex)
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

