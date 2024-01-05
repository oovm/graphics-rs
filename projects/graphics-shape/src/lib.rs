#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/11549616")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/11549616")]

/// A macro to create a new graphics shape.
pub type Point = shape_core::Point<f32>;
/// A circle defined by center and radius.
pub type Circle = shape_core::Circle<f32>;
/// An ellipse defined by center and axes.
pub type Ellipse = shape_core::Ellipse<f32>;
/// A line segment of finite length, determined by a starting point and an ending point.
pub type Line = shape_core::Line<f32>;
/// A square without rotated.
pub type Square = shape_core::Square<f32>;
/// A rectangle without rotated.
pub type Rectangle = shape_core::Rectangle<f32>;
/// A triangle defined by three points.
pub type Triangle = shape_core::Triangle<f32>;
/// A convex polygon defined by a set of points.
pub type Polygon = shape_core::Polygon<f32>;
/// A polyline defined by a set of points.
pub type Polyline = shape_core::Polyline<f32>;
/// A regular polygon defined by center, radius and edges.
pub type RegularPolygon = shape_core::RegularPolygon<f32>;
