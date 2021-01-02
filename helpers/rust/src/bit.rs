use std::ops;

/// A struct which allows bit-operations on arbitrary sized numbers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Bits {
    size: usize,
    bits: Vec<u8>,
}

impl Bits {
    /// Construct a new instance of `Bits`.
    pub fn new(bits: &[u8]) -> Bits {
        let size = bits.len();
        let mut v = vec![0; 1 + bits.len() / 8];
        for (idx, bit) in bits.iter().rev().enumerate() {
            //
        }
        Bits { size, bits: v }
    }
}

impl ops::Shl<usize> for Bits {
    type Output = Bits;

    /// Shift all bits `offset` positions to the left, dropping any bits that overflow.
    fn shl(self, offset: usize) -> Self::Output {
        unimplemented!()
    }
}
