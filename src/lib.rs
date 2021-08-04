
//! The Tristate crate adds an enum that can be either true, false, or represent a default value as a three-valued
//! type alternative to `Option<bool>`, along with a series of helper functions for converting back and forth.
//!
//! To construct a TriState value simply call
//! ```rust
//! tristate::TriState::from(true);
//! ```
//! or convert from an `Option<bool>` using
//! ```rust
//! tristate::TriState::from(Some(true));
//! ```

use std::fmt::Display;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};

/// Represents a enum value that can be either true, false, or represent a default value
///
/// An alternative to `Option<bool>`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum TriState {
    /// Represents the boolean value of `false`
    False,
    /// Represents a fallback to a "default" value
    Default,
    /// Represents the boolean value of `true`
    True
}

impl TriState {
    /// Returns the corresponding boolean value for the tri-state, or if the tri-state
    /// is `TriState::Default` returns the fallback value.
    pub fn or_else(&self, fallback: bool) -> bool {
        if self == &Self::Default { fallback } else { self.into() }
    }

    /// Returns the corresponding boolean value for the tri-state, or if the tri-state
    /// is `TriState::Default` returns the fallback value from the given supplier.
    pub fn or_else_get(&self, supplier: fn () -> bool) -> bool {
        if self == &Self::Default { supplier() } else { self.into() }
    }
}

impl From<bool> for TriState {
    /// Returns the corresponding tristate from a boolean value
    ///
    /// ```rust
    /// assert_eq!(tristate::TriState::from(false), tristate::TriState::False);
    /// assert_eq!(tristate::TriState::from(true), tristate::TriState::True);
    /// ```
    fn from(b: bool) -> Self {
        if b { Self::True } else { Self::False }
    }
}

impl From<Option<bool>> for TriState {
    /// Returns the corresponding tri-state from an optional boolean value
    ///
    /// ```rust
    /// assert_eq!(tristate::TriState::from(Some(false)), tristate::TriState::False);
    /// assert_eq!(tristate::TriState::from(Some(true)), tristate::TriState::True);
    /// assert_eq!(tristate::TriState::from(None), tristate::TriState::Default);
    /// ```
    fn from(b: Option<bool>) -> Self {
        if let Some(inner) = b { Self::from(inner) } else { Self::Default }
    }
}

impl Default for TriState {
    fn default() -> Self {
        Self::Default
    }
}

impl Display for TriState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl AsRef<bool> for TriState {
    fn as_ref(&self) -> &bool {
        if self == &Self::True { &true } else { &false }
    }
}

impl AsRef<Option<bool>> for TriState {
    fn as_ref(&self) -> &Option<bool> {
        match self {
            Self::False => &Some(false),
            Self::Default => &None,
            Self::True => &Some(true)
        }
    }
}

impl From<TriState> for bool {
    /// Returns the boolean value for the tri-state
    ///
    /// ```rust
    /// assert_eq!(bool::from(tristate::TriState::False), false);
    /// assert_eq!(bool::from(tristate::TriState::True), true);
    /// assert_eq!(bool::from(tristate::TriState::Default), false);
    /// ```
    fn from(t: TriState) -> Self {
        Self::from(&t)
    }
}

impl From<&TriState> for bool {
    fn from(t: &TriState) -> Self {
        t == &TriState::True
    }
}

impl From<TriState> for Option<bool> {
    /// Returns the optional boolean value for the tri-state
    ///
    /// ```rust
    /// assert_eq!(Option::<bool>::from(tristate::TriState::False), Some(false));
    /// assert_eq!(Option::<bool>::from(tristate::TriState::True), Some(true));
    /// assert_eq!(Option::<bool>::from(tristate::TriState::Default), None);
    /// ```
    fn from(t: TriState) -> Self {
        Self::from(&t)
    }
}

impl From<&TriState> for Option<bool> {
    fn from(t: &TriState) -> Self {
        match t {
            TriState::False => Some(false),
            TriState::Default => None,
            TriState::True => Some(true)
        }
    }
}