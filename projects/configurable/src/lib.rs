mod methods;
mod traits;

/// A configurable field follows tri-state logic.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
