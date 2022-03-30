use super::*;

/// initial(初始)、inherit(继承)、unset(未设置)
pub enum Config<T>
where
    T: Default,
{
    /// Every settable property has a initial value
    Initial,
    /// shape.style = none
    Unset,
    /// shape.style = my_style
    Normal(T),
}

impl<T> Config<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Initial | Self::Unset => Default::default(),
            Self::Normal(t) => t,
        }
    }
}

impl<T> AddAssign<Self> for Config<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = match (self, rhs) {
            (Self::Initial, Self::Initial) => Self::Initial,
            (Self::Initial, Self::Unset) => Self::Unset,
            (Self::Initial, Self::Normal(rhs)) => Self::Normal(rhs),
            (Self::Unset, Self::Initial) => Self::Unset,
            (Self::Unset, Self::Unset) => Self::Unset,
            (Self::Unset, Self::Normal(rhs)) => Self::Normal(rhs),
            (Self::Normal(lhs), Self::Initial) => Self::Normal(lhs),
            (Self::Normal(_), Self::Unset) => Self::Unset,
            (Self::Normal(_), Self::Normal(rhs)) => Self::Normal(rhs),
        }
    }
}
