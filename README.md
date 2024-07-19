# str-to-flags
Types to convert lists of strings into a bitflag representation.

Designed to work with the `arctan-bitflags` crate [GIT](https://github.com/ArchTangent/arctan-bitflags).

## Usage

```toml
[dependencies]
str-to-flags = { git = "https://github.com/ArchTangent-gamedev/str-to-flags.git", branch = "main" }
```

## Example

```rust
let planets_str = [
    "mercury",
    "venus",
    "earth",
    "mars",
    "jupiter",
    "saturn",
    "uranus",
    "neptune",
];

let planet_flags = StrFlags::<u8>::from_slice(&planets_str);

assert_eq!(planet_flags.get("mars"), Some(&8u8));   // 2^3
```
