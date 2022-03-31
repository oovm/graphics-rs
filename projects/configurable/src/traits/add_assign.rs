use super::*;
use crate::Setting::{Initial, Unset};
use Setting::Normal;

impl<T> AddAssign<Self> for Setting<T>
where
    T: Default,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = match (&self, rhs) {
            (Initial, Initial) => Initial,
            (Initial, Unset) => Unset,
            (Initial, Normal(rhs)) => Normal(rhs),
            (Unset, Initial) => Unset,
            (Unset, Unset) => Unset,
            (Unset, Normal(rhs)) => Normal(rhs),
            (Normal(_), Initial) => return,
            (Normal(_), Unset) => Unset,
            (Normal(_), Normal(rhs)) => Normal(rhs),
        }
    }
}

impl<T> AddAssign<Option<T>> for Setting<T>
where
    T: Default,
{
    /// here None = Initial
    fn add_assign(&mut self, rhs: Option<T>) {
        *self = match (&self, rhs) {
            (Initial, Some(rhs)) => Normal(rhs),
            (Initial, None) => Initial,
            (Unset, Some(rhs)) => Normal(rhs),
            (Unset, None) => Unset,
            (Normal(_), Some(rhs)) => Normal(rhs),
            (Normal(_), None) => return,
        }
    }
}

impl<T> AddAssign<T> for Setting<T>
where
    T: Default,
{
    fn add_assign(&mut self, rhs: T) {
        *self = Normal(rhs)
    }
}
