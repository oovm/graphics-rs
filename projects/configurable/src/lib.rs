#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod methods;
mod traits;

/// A configurable field follows 3-state logic.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Setting<T>
where
    T: Default,
{
    /// Every settable property has a initial value
    ///
    /// ```rust
    /// this.cfg = Default::default()
    /// ```
    Initial,
    /// Unset the current setting
    ///
    /// ```rust
    /// this.cfg = None
    /// ```
    Unset,
    /// Set a new value
    /// ```rust
    /// this.cfg = Some(cfg)
    /// ```
    Normal(T),
}
