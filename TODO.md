# String to Flags - To-Do List

## New Structure types

- `StrToFlags` (Vec): uses a `Vec` instead of a `HashMap`
  - alphabetical sorting and indexing for faster index retrieval
- `StrToEnum` (HashMap): uses a `HashMap` just like `StrFlags`
  - alphabetical sorting and indexing for faster index retrieval
- `StrToEnum` (Vec): uses a `Vec` instead of a `HashMap`
  - alphabetical sorting and indexing for faster index retrieval

## Testing

- `#[should_panic]` on OOB `from_pow()` values
