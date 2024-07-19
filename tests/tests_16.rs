//! Testing for 16-bit string to flags structs.
//! 
//! Note: excess strings are not counted as duplicates, and vice-versa.

use arctan_bitflags::BitFlags16;
use str_to_flags::StrFlags;

#[rustfmt::skip]
pub const DATA1: [&str; 12] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"
];

#[rustfmt::skip]
pub const DATA2: [&str; 16] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", 
    "m", "n", "o", "p"
];

#[rustfmt::skip]
pub const DATA3: [&str; 24] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", 
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x"
];

#[rustfmt::skip]
pub const DATA4: [&str; 24] = [
    "a", "a", "a", "a", "b", "b", "b", "b", "b", "c", "d", "e", 
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "v", "v"
];

#[rustfmt::skip]
pub const DATA5: [&str; 24] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", 
    "m", "n", "o", "o", "o", "p", "q", "r", "s", "t", "u", "v",
];

#[test]
fn u16_general() {
    let f1 = StrFlags::<u16>::from_slice(&DATA1);
    let f2 = StrFlags::<u16>::from_slice(&DATA2);
    let f3 = StrFlags::<u16>::from_slice(&DATA3);
    let f4 = StrFlags::<u16>::from_slice(&DATA4);
    let f5 = StrFlags::<u16>::from_slice(&DATA5);

    // Indexing
    assert_eq!(f1["a"], 1);
    assert_eq!(f1.get("h"), Some(&2u16.pow(7)));
    assert_eq!(f1.get("m"), None);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();
    
    assert_eq!(count1, 12);
    assert_eq!(count2, 16);
    assert_eq!(count3, 16);
    assert_eq!(count4, 15);
    assert_eq!(count5, 16);

    // Duplicates
    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();
    let dupes4 = f4.duplicates();
    let dupes5 = f5.duplicates();
    
    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 0);
    assert_eq!(dupes3, 0);
    assert_eq!(dupes4, 9);
    assert_eq!(dupes5, 2);
    
    // Excess
    let excess1 = f1.excess();
    let excess2 = f2.excess();
    let excess3 = f3.excess();
    let excess4 = f4.excess();
    let excess5 = f5.excess();

    assert_eq!(excess1, 0);
    assert_eq!(excess2, 0);
    assert_eq!(excess3, 8);
    assert_eq!(excess4, 0);
    assert_eq!(excess5, 6);
}

#[test]
fn flags16_general() {
    let f1 = StrFlags::<BitFlags16>::from_slice(&DATA1);
    let f2 = StrFlags::<BitFlags16>::from_slice(&DATA2);
    let f3 = StrFlags::<BitFlags16>::from_slice(&DATA3);
    let f4 = StrFlags::<BitFlags16>::from_slice(&DATA4);
    let f5 = StrFlags::<BitFlags16>::from_slice(&DATA5);

    // Indexing
    assert_eq!(f1["a"], BitFlags16(1));
    assert_eq!(f1.get("h"), Some(&BitFlags16(2u16.pow(7))));
    assert_eq!(f1.get("m"), None);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();
    
    assert_eq!(count1, 12);
    assert_eq!(count2, 16);
    assert_eq!(count3, 16);
    assert_eq!(count4, 15);
    assert_eq!(count5, 16);

    // Duplicates
    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();
    let dupes4 = f4.duplicates();
    let dupes5 = f5.duplicates();
    
    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 0);
    assert_eq!(dupes3, 0);
    assert_eq!(dupes4, 9);
    assert_eq!(dupes5, 2);
    
    // Excess
    let excess1 = f1.excess();
    let excess2 = f2.excess();
    let excess3 = f3.excess();
    let excess4 = f4.excess();
    let excess5 = f5.excess();

    assert_eq!(excess1, 0);
    assert_eq!(excess2, 0);
    assert_eq!(excess3, 8);
    assert_eq!(excess4, 0);
    assert_eq!(excess5, 6);
}
