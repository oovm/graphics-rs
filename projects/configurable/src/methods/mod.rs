use crate::Setting;

impl<T> Setting<T>
where
    T: Default,
{
    pub fn unset(&mut self) {
        *self = Self::Unset;
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Initial | Self::Unset => Default::default(),
            Self::Normal(t) => t,
        }
    }
}
