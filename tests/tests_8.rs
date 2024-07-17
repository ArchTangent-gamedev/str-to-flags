//! Testing for 8-bit string to flags structs.
//! 
//! Notes: 
//! - `flags`-prefixed tests are for `BitFlags` types
//! - `plain`-prefixed tests are for unsigned integers
//! - excess strings are not counted as duplicates.

use arctan_bitflags::BitFlags8;
use str_to_flags::StrFlags;

pub const DATA1: [&str; 12] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "j", "j"];
pub const DATA2: [&str; 12] = ["a", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "j"];
pub const DATA3: [&str; 12] = ["a", "a", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

#[test]
fn flags8_duplicates() {
    let f1 = StrFlags::<u8>::from_slice(&DATA1);
    let f2 = StrFlags::<u8>::from_slice(&DATA2);
    let f3 = StrFlags::<u8>::from_slice(&DATA3);

    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();

    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 1);
    assert_eq!(dupes3, 2);
}

#[test]
fn plain8_duplicates() {
    let f1 = StrFlags::<BitFlags8>::from_slice(&DATA1);
    let f2 = StrFlags::<BitFlags8>::from_slice(&DATA2);
    let f3 = StrFlags::<BitFlags8>::from_slice(&DATA3);

    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();

    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 1);
    assert_eq!(dupes3, 2);
}
