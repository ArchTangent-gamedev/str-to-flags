//! Testing for 8-bit string to flags structs.
//! 
//! Note: excess strings are not counted as duplicates, and vice-versa.

use arctan_bitflags::BitFlags8;
use str_to_flags::StrFlags;

pub const DATA1: [&str; 12] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "j", "j"];
pub const DATA2: [&str; 12] = ["a", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "j"];
pub const DATA3: [&str; 12] = ["a", "a", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
pub const DATA4: [&str; 12] = ["a", "a", "a", "a", "a", "a", "b", "c", "d", "e", "f", "g"];
pub const DATA5: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

#[test]
fn u8_general() {
    let f1 = StrFlags::<u8>::from_slice(&DATA1);
    let f2 = StrFlags::<u8>::from_slice(&DATA2);
    let f3 = StrFlags::<u8>::from_slice(&DATA3);
    let f4 = StrFlags::<u8>::from_slice(&DATA4);
    let f5 = StrFlags::<u8>::from_slice(&DATA5);

    // Indexing
    assert_eq!(f1["a"], 1);
    assert_eq!(f1.get("h"), Some(&2u8.pow(7)));
    assert_eq!(f1.get("i"), None);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();
    
    assert_eq!(count1, 8);
    assert_eq!(count2, 8);
    assert_eq!(count3, 8);
    assert_eq!(count4, 7);
    assert_eq!(count5, 6);

    // Duplicates
    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();
    let dupes4 = f4.duplicates();
    let dupes5 = f5.duplicates();
    
    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 1);
    assert_eq!(dupes3, 2);
    assert_eq!(dupes4, 5);
    assert_eq!(dupes5, 0);
    
    // Excess
    let excess1 = f1.excess();
    let excess2 = f2.excess();
    let excess3 = f3.excess();
    let excess4 = f4.excess();
    let excess5 = f5.excess();

    assert_eq!(excess1, 4);
    assert_eq!(excess2, 3);
    assert_eq!(excess3, 2);
    assert_eq!(excess4, 0);
    assert_eq!(excess5, 0);
}

#[test]
fn flags8_general() {
    let f1 = StrFlags::<BitFlags8>::from_slice(&DATA1);
    let f2 = StrFlags::<BitFlags8>::from_slice(&DATA2);
    let f3 = StrFlags::<BitFlags8>::from_slice(&DATA3);
    let f4 = StrFlags::<BitFlags8>::from_slice(&DATA4);
    let f5 = StrFlags::<BitFlags8>::from_slice(&DATA5);

    // Indexing
    assert_eq!(f1["a"], BitFlags8(1));
    assert_eq!(f1.get("h"), Some(&BitFlags8(2u8.pow(7))));
    assert_eq!(f1.get("i"), None);
    
    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();
    
    assert_eq!(count1, 8);
    assert_eq!(count2, 8);
    assert_eq!(count3, 8);
    assert_eq!(count4, 7);
    assert_eq!(count5, 6);

    // Duplicates
    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();
    let dupes4 = f4.duplicates();
    let dupes5 = f5.duplicates();
    
    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 1);
    assert_eq!(dupes3, 2);
    assert_eq!(dupes4, 5);
    assert_eq!(dupes5, 0);
    
    // Excess
    let excess1 = f1.excess();
    let excess2 = f2.excess();
    let excess3 = f3.excess();
    let excess4 = f4.excess();
    let excess5 = f5.excess();

    assert_eq!(excess1, 4);
    assert_eq!(excess2, 3);
    assert_eq!(excess3, 2);
    assert_eq!(excess4, 0);
    assert_eq!(excess5, 0);
}
