# Tristate

A simple Rust library that adds an alternative to `Option<bool>` used to represent a value that can be either true, false, or default.

The Tristate crate adds an enum that can be either true, false, or represent a default value as a three-valued type alternative to `Option<bool>`, along with a series of helper functions for converting back and forth.

To construct a TriState value simply call
```rust
tristate::TriState::from(true);
```
or convert from an `Option<bool>` using
```rust
tristate::TriState::from(Some(true));
```