//! String to BitFlag conversion for use with games.
//!
//! Takes a list of strings and associates a bitflag to each unique string.
//! Tracks duplicate (same string more than once) and excess (more strings than available bits) strings.

mod str_flags;
mod traits;
pub use str_flags::*;
