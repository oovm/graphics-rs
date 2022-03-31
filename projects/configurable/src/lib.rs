#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str ! ("../Readme.md")]

mod methods;
mod traits;

/// A configurable field follows 3-state logic.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// ```rust
    /// this.cfg = None
    /// ```
    Unset,
    /// ```rust
    /// this.cfg = Some(cfg)
    /// ```
    Normal(T),
}
