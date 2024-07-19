//! String to flag tracker for 8-bit values.

use crate::traits::Bits;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// Converts string into its bitflag representation.
///
/// The first entry is always "NONE" with a value of `0`.
#[derive(Debug)]
pub struct StrFlags<T: Bits> {
    inner: HashMap<String, T>,
    count: usize,
    excess: usize,
    duplicates: usize,
}

impl<T: Bits> StrFlags<T> {
    /// Creates a new instance.
    pub fn new() -> Self {
        let mut inner = HashMap::with_capacity(T::num_bits() + 1);
        inner.insert("NONE".into(), T::zero());
        Self {
            inner,
            count: 0,
            excess: 0,
            duplicates: 0,
        }
    }
    /// Creates a new instance from a slice of `&str` values.
    pub fn from_slice(strs: &[&str]) -> Self {
        let mut flags = Self::new();
        for s in strs.iter() {
            flags.insert(s);
        }

        flags
    }
    /// Returns the stored flag for a given string, if present.
    pub fn get(&self, string: &str) -> Option<&T> {
        self.inner.get(string)
    }
    /// Inserts a string ID and returns its flag representation if string is not present.
    ///
    /// Increments `duplicate` counter and returns flag if ID already exists.
    /// Increments `excess` counter and returns `None` if flag limit is exceeded.
    pub fn insert(&mut self, s: &str) -> Option<T> {
        if self.inner.contains_key(s) {
            self.duplicates += 1;
            Some(self.inner[s])
        } else if self.count >= T::num_bits() {
            self.excess += 1;
            None
        } else {
            self.inner
                .insert(s.to_owned(), T::from_pow(self.count as u32));
            self.count += 1;
            Some(self.inner[s])
        }
    }
    /// Returns an iterator visiting all key-value pairs in arbitrary order.
    pub fn iter(&self) -> Iter<String, T> {
        self.inner.iter()
    }
    /// Returns `true` if the struct contains the given string.
    pub fn contains(&self, string: &str) -> bool {
        self.inner.contains_key(string)
    }
    /// Returns number of unique string/flag pairs in the structure, _excluding_ `("NONE": 0)`.
    pub fn count(&self) -> usize {
        self.count
    }
    /// Returns number of duplicate strings pairs added to the structure.
    pub fn duplicates(&self) -> usize {
        self.duplicates
    }
    /// Returns number of excess strings added to the structure.
    pub fn excess(&self) -> usize {
        self.excess
    } 
}

impl<T: Bits> std::ops::Index<&String> for StrFlags<T> {
    type Output = T;

    fn index(&self, index: &String) -> &Self::Output {
        &self.inner.index(index)
    }
}
