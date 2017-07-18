use super::*;

impl Point {
    pub fn get_size(&self, style: &StyleResolver) -> f32 {
        self.size.unwrap_or(style.point_size()).clone()
    }

    pub fn set_size<T>(&mut self, value: T)
    where
        T: Into<f32>,
    {
        self.size = Some(value.into());
    }

    /// Getter of [`PointColor`]
    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.point_color()).clone()
    }
    /// Setter of [`PointColor`]
    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }
    /// Builder of [`PointColor`]
    pub fn with_color<T>(mut self, value: T) -> Self
    where
        T: Into<RGBA>,
    {
        self.set_color(value);
        self
    }
}

macro_rules! impl_point_style {
    ($id:ident: $($getter:ident, $setter:ident, $builder:ident, $outer:ty, $inner:ty, $this:ty, $that:ty),*) => {
        $(
            impl_point_style!(@wrapper, $getter:ident, $setter:ident, $builder:ident, $outer:ty, $inner:ty, $this:ty, $that:ty)
        )*
    };
    (@wrapper, $getter:ident, $setter:ident, $builder:ident, $outer:ty, $inner:ty, $this:ty, $that:ty) => {
        impl_point_style!(@getter, $getter, $outer, $inner, $this, $that);
        impl_point_style!(@setter, $setter, $outer, $inner, $this);
        impl_point_style!(@builder, $builder, $outer, $inner, $setter);
    };
    (@getter, $getter:ident, $outer:ty, $inner:ty, $field:ty, $function:ty) => {
        #[doc = concat!("Getter of [`", stringify!($ty), "`]")]
        pub fn $getter(&self, style: &StyleResolver) -> $inner {
            self.$field.unwrap_or(style.$function()).clone()
        }
    };
    (@setter, $setter:ident, $outer:ty, $inner:ty, $field:ty) => {
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

impl_point_style![
    Point: (get_color, set_color, with_color, PointColor, RGBA, color, point_color),
    (get_color, set_color, with_color, PointColor, RGBA, color, point_color),
];
