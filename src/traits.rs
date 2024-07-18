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

#[cfg(feature = "bitflags-support")]
mod impl_bitflags {
    use arctan_bitflags::BitFlags8;

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
}
