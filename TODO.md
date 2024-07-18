# String to Flags - To-Do List

## New Structure types

- `StrToFlags` (Vec): uses a `Vec` instead of a `HashMap`
  - alphabetical sorting and indexing for faster index retrieval

## Readme

- `Usage` with features

## Testing

- all types: `u8` to `u128`, with `Bits` trait and `BitFlags` variant for all
- `#[should_panic]` on OOB `from_pow()` values
- object creation
- duplicates
- excess
