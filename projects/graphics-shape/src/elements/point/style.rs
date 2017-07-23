use super::*;

macro_rules! impl_point_style {
    (@wrapper, $getter:ident, $setter:ident, $builder:ident, $outer:ty, $inner:ty, $this:ident, $that:ident) => {
        impl_point_style!(@getter, $getter, $outer, $inner, $this, $that);
        impl_point_style!(@setter, $setter, $outer, $inner, $this);
        impl_point_style!(@builder, $builder, $outer, $inner, $setter);
    };
    (@getter, $getter:ident, $outer:ty, $inner:ty, $field:ident, $function:ident) => {
        #[doc = concat!("Getter of [`", stringify!($ty), "`]")]
        pub fn $getter(&self, style: &StyleResolver) -> $inner {
            self.$field.unwrap_or(style.$function()).clone()
        }
    };
    (@setter, $setter:ident, $outer:ty, $inner:ty, $field:ident) => {
        #[doc = concat!("Setter of [`", stringify!($ty), "`]")]
        pub fn $setter<T>(&mut self, value: T)
        where
            T: Into<$inner>,
        {
            self.$field = Some(value.into())
        }
    };
    (@builder, $builder:ident, $outer:ty, $inner:ty, $setter:ident) => {
        #[doc = concat!("Builder of [`", stringify!($ty), "`]")]
        pub fn $builder<T>(mut self, value: T) -> Self
        where
            T: Into<$inner>,
        {
            self.$setter(value);
            self
        }
    };
}

impl Point {
    impl_point_style!(@wrapper, get_size, set_size, with_size, PointSize, f32, size, point_size);
    impl_point_style!(@wrapper, get_color, set_color, with_color, PointColor, RGBA, color, point_color);
}
