//! Testing for 64-bit string to flags structs.
//!
//! Note: excess strings are not counted as duplicates, and vice-versa.

use arctan_bitflags::BitFlags64;
use str_to_flags::StrFlags;

#[rustfmt::skip]
pub const DATA1: [&str; 12] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"
];

#[rustfmt::skip]
pub const DATA2: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "p3", 
];

#[rustfmt::skip]
pub const DATA3: [&str; 72] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "m3", "m3", "m3", "m3", "m3", "n3", "n3", "n3", "n3", "n3", "n3",
];

#[rustfmt::skip]
pub const DATA4: [&str; 72] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "o3", "p3", "p3", "q3", "r3", "s3", "t3", "u3", "v3",
];

#[rustfmt::skip]
pub const DATA5: [&str; 72] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "p3", "q3", "r3", "s3", "t3", "u3", "v3", "w3", "x3",
];

#[test]
fn u64_general() {
    let f1 = StrFlags::<u64>::from_slice(&DATA1);
    let f2 = StrFlags::<u64>::from_slice(&DATA2);
    let f3 = StrFlags::<u64>::from_slice(&DATA3);
    let f4 = StrFlags::<u64>::from_slice(&DATA4);
    let f5 = StrFlags::<u64>::from_slice(&DATA5);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();

    assert_eq!(count1, 12);
    assert_eq!(count2, 64);
    assert_eq!(count3, 62);
    assert_eq!(count4, 64);
    assert_eq!(count5, 64);

    // Duplicates
    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();
    let dupes4 = f4.duplicates();
    let dupes5 = f5.duplicates();

    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 0);
    assert_eq!(dupes3, 10);
    assert_eq!(dupes4, 2);
    assert_eq!(dupes5, 0);

    // Excess
    let excess1 = f1.excess();
    let excess2 = f2.excess();
    let excess3 = f3.excess();
    let excess4 = f4.excess();
    let excess5 = f5.excess();

    assert_eq!(excess1, 0);
    assert_eq!(excess2, 0);
    assert_eq!(excess3, 0);
    assert_eq!(excess4, 6);
    assert_eq!(excess5, 8);
}

#[test]
fn flags64_general() {
    let f1 = StrFlags::<BitFlags64>::from_slice(&DATA1);
    let f2 = StrFlags::<BitFlags64>::from_slice(&DATA2);
    let f3 = StrFlags::<BitFlags64>::from_slice(&DATA3);
    let f4 = StrFlags::<BitFlags64>::from_slice(&DATA4);
    let f5 = StrFlags::<BitFlags64>::from_slice(&DATA5);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();

    assert_eq!(count1, 12);
    assert_eq!(count2, 64);
    assert_eq!(count3, 62);
    assert_eq!(count4, 64);
    assert_eq!(count5, 64);

    // Duplicates
    let dupes1 = f1.duplicates();
    let dupes2 = f2.duplicates();
    let dupes3 = f3.duplicates();
    let dupes4 = f4.duplicates();
    let dupes5 = f5.duplicates();

    assert_eq!(dupes1, 0);
    assert_eq!(dupes2, 0);
    assert_eq!(dupes3, 10);
    assert_eq!(dupes4, 2);
    assert_eq!(dupes5, 0);

    // Excess
    let excess1 = f1.excess();
    let excess2 = f2.excess();
    let excess3 = f3.excess();
    let excess4 = f4.excess();
    let excess5 = f5.excess();

    assert_eq!(excess1, 0);
    assert_eq!(excess2, 0);
    assert_eq!(excess3, 0);
    assert_eq!(excess4, 6);
    assert_eq!(excess5, 8);
}
