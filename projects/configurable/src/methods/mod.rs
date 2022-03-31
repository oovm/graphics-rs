use crate::Setting;

impl<T> Setting<T>
where
    T: Default,
{
    /// Set a value to this field.
    pub fn set(&mut self, value: T) {
        *self = Self::Normal(value);
    }
    /// Reset value of this field to default.
    pub fn reset(&mut self) {
        *self = Self::Initial;
    }
    /// Unset the current value
    pub fn unset(&mut self) {
        *self = Self::Unset;
    }
    /// Get value of this field
    pub fn unwrap(self) -> T {
        match self {
            Self::Initial | Self::Unset => Default::default(),
            Self::Normal(t) => t,
        }
    }
    /// Check if this field has not been set
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Initial)
    }
    /// Check if this field has been set to none
    pub fn is_none(&self) -> bool {
        matches!(self, Self::Unset)
    }
    /// Check if this field has been set to some value
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Normal(_))
    }
}
