//! Testing for 128-bit string to flags structs.
//!
//! Note: excess strings are not counted as duplicates, and vice-versa.

use arctan_bitflags::BitFlags128;
use str_to_flags::StrFlags;

#[rustfmt::skip]
pub const DATA1: [&str; 12] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"
];

#[rustfmt::skip]
pub const DATA2: [&str; 128] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "p3", "q3", "r3", "s3", "t3", "u3", "v3", "w3", "x3", 
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "i4", "j4", "k4", "l4", 
    "m4", "n4", "o4", "p4", "q4", "r4", "s4", "t4", "u4", "v4", "w4", "x4", 
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "i5", "j5", "k5", "l5", 
    "m5", "n5", "o5", "p5", "q5", "r5", "s5", "t5", "u5", "v5", "w5", "x5", 
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", 
];

#[rustfmt::skip]
pub const DATA3: [&str; 132] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "p3", "q3", "r3", "s3", "t3", "u3", "v3", "w3", "x3", 
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "i4", "j4", "k4", "l4", 
    "m4", "n4", "o4", "p4", "q4", "r4", "s4", "t4", "u4", "v4", "w4", "x4", 
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "i5", "j5", "k5", "l5", 
    "m5", "n5", "o5", "p5", "q5", "r5", "s5", "t5", "u5", "v5", "w5", "x5", 
    "a6", "a6", "a6", "a6", "a6", "a6", "b6", "b6", "b6", "b6", "b6", "b6", 
];

#[rustfmt::skip]
pub const DATA4: [&str; 132] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "p3", "q3", "r3", "s3", "t3", "u3", "v3", "w3", "x3", 
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "i4", "j4", "k4", "l4", 
    "m4", "n4", "o4", "p4", "q4", "r4", "s4", "t4", "u4", "v4", "w4", "x4", 
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "i5", "j5", "k5", "l5", 
    "m5", "n5", "o5", "p5", "q5", "r5", "s5", "t5", "u5", "v5", "w5", "x5", 
    "a6", "b6", "c6", "d6", "d6", "e6", "e6", "f6", "g6", "h6", "i6", "j6", 
];

#[rustfmt::skip]
pub const DATA5: [&str; 132] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1", "j1", "k1", "l1", 
    "m1", "n1", "o1", "p1", "q1", "r1", "s1", "t1", "u1", "v1", "w1", "x1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2", "j2", "k2", "l2", 
    "m2", "n2", "o2", "p2", "q2", "r2", "s2", "t2", "u2", "v2", "w2", "x2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3", "j3", "k3", "l3", 
    "m3", "n3", "o3", "p3", "q3", "r3", "s3", "t3", "u3", "v3", "w3", "x3", 
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "i4", "j4", "k4", "l4", 
    "m4", "n4", "o4", "p4", "q4", "r4", "s4", "t4", "u4", "v4", "w4", "x4", 
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "i5", "j5", "k5", "l5", 
    "m5", "n5", "o5", "p5", "q5", "r5", "s5", "t5", "u5", "v5", "w5", "x5", 
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "i6", "j6", "k6", "l6", 
];

#[test]
fn u128_general() {
    let f1 = StrFlags::<u128>::from_slice(&DATA1);
    let f2 = StrFlags::<u128>::from_slice(&DATA2);
    let f3 = StrFlags::<u128>::from_slice(&DATA3);
    let f4 = StrFlags::<u128>::from_slice(&DATA4);
    let f5 = StrFlags::<u128>::from_slice(&DATA5);

    // Indexing
    assert_eq!(f2["a1"], 1);
    assert_eq!(f2.get("h6"), Some(&2u128.pow(127)));
    assert_eq!(f2.get("i6"), None);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();

    assert_eq!(count1, 12);
    assert_eq!(count2, 128);
    assert_eq!(count3, 122);
    assert_eq!(count4, 128);
    assert_eq!(count5, 128);

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
    assert_eq!(excess4, 2);
    assert_eq!(excess5, 4);
}

#[test]
fn flags128_general() {
    let f1 = StrFlags::<BitFlags128>::from_slice(&DATA1);
    let f2 = StrFlags::<BitFlags128>::from_slice(&DATA2);
    let f3 = StrFlags::<BitFlags128>::from_slice(&DATA3);
    let f4 = StrFlags::<BitFlags128>::from_slice(&DATA4);
    let f5 = StrFlags::<BitFlags128>::from_slice(&DATA5);

    // Indexing
    assert_eq!(f2["a1"], BitFlags128(1));
    assert_eq!(f2.get("h6"), Some(&BitFlags128(2u128.pow(127))));
    assert_eq!(f2.get("i6"), None);

    // Count
    let count1 = f1.count();
    let count2 = f2.count();
    let count3 = f3.count();
    let count4 = f4.count();
    let count5 = f5.count();

    assert_eq!(count1, 12);
    assert_eq!(count2, 128);
    assert_eq!(count3, 122);
    assert_eq!(count4, 128);
    assert_eq!(count5, 128);

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
    assert_eq!(excess4, 2);
    assert_eq!(excess5, 4);    
}
