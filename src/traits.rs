//! Traits for the `str-to-flags` crate.

pub trait Bits: Clone + Copy {
    fn zero() -> Self;
    fn from_pow(exp: u32) -> Self;
    fn num_bits() -> usize;
}

impl Bits for u8 {
    fn zero() -> Self {
        0u8
    }

    fn from_pow(exp: u32) -> Self {
        2u8.pow(exp)
    }

    fn num_bits() -> usize {
        8
    }
}

impl Bits for u16 {
    fn zero() -> Self {
        0u16
    }

    fn from_pow(exp: u32) -> Self {
        2u16.pow(exp)
    }

    fn num_bits() -> usize {
        16
    }
}

impl Bits for u32 {
    fn zero() -> Self {
        0u32
    }

    fn from_pow(exp: u32) -> Self {
        2u32.pow(exp)
    }

    fn num_bits() -> usize {
        32
    }
}

impl Bits for u64 {
    fn zero() -> Self {
        0u64
    }

    fn from_pow(exp: u32) -> Self {
        2u64.pow(exp)
    }

    fn num_bits() -> usize {
        64
    }
}

#[cfg(feature = "bitflags-support")]
mod impl_bitflags {
    use arctan_bitflags::BitFlags8;
    use arctan_bitflags::BitFlags16;
    use arctan_bitflags::BitFlags32;
    use arctan_bitflags::BitFlags64;

    impl super::Bits for BitFlags8 {
        fn zero() -> Self {
            BitFlags8::empty()
        }

        fn from_pow(exp: u32) -> Self {
            BitFlags8(2u8.pow(exp))
        }

        fn num_bits() -> usize {
            8
        }
    }

    impl super::Bits for BitFlags16 {
        fn zero() -> Self {
            BitFlags16::empty()
        }

        fn from_pow(exp: u32) -> Self {
            BitFlags16(2u16.pow(exp))
        }

        fn num_bits() -> usize {
            16
        }
    }

    impl super::Bits for BitFlags32 {
        fn zero() -> Self {
            BitFlags32::empty()
        }

        fn from_pow(exp: u32) -> Self {
            BitFlags32(2u32.pow(exp))
        }

        fn num_bits() -> usize {
            32
        }
    }

    impl super::Bits for BitFlags64 {
        fn zero() -> Self {
            BitFlags64::empty()
        }

        fn from_pow(exp: u32) -> Self {
            BitFlags64(2u64.pow(exp))
        }

        fn num_bits() -> usize {
            64
        }
    }
}
