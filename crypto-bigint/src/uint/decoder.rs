//! Const-capable bigint decoder.
//!
//! Support for loading bigint values a byte-at-a-time.

use super::UInt;
use crate::{Limb, LIMB_BYTES};

#[derive(Clone, Debug)]
pub(crate) struct Decoder<const LIMBS: usize> {
    /// Limbs being decoded.
    ///
    /// Stored from least significant to most significant.
    limbs: [Limb; LIMBS],

    /// Current limb being decoded.
    index: usize,

    /// Total number of bytes consumed.
    bytes: usize,
}

impl<const LIMBS: usize> Decoder<LIMBS> {
    /// Create a new decoder.
    pub const fn new() -> Self {
        Self {
            limbs: [0; LIMBS],
            index: 0,
            bytes: 0,
        }
    }

    /// Add a byte onto the [`UInt`] being decoded.
    pub const fn add_byte(mut self, byte: u8) -> Self {
        if self.bytes == LIMB_BYTES {
            const_assert!(self.index < LIMBS, "too many bytes in UInt");
            self.index += 1;
            self.bytes = 0;
        }

        self.limbs[self.index] |= (byte as Limb) << (self.bytes * 8);
        self.bytes += 1;
        self
    }

    /// Finish decoding a [`UInt`], returning a decoded value only if we've
    /// received the expected number of bytes.
    pub const fn finish(self) -> UInt<LIMBS> {
        const_assert!(self.index == LIMBS - 1, "decoded UInt is missing limbs");
        const_assert!(self.bytes == LIMB_BYTES, "decoded UInt is missing bytes");
        UInt { limbs: self.limbs }
    }
}

impl<const LIMBS: usize> Default for Decoder<LIMBS> {
    fn default() -> Self {
        Self::new()
    }
}
