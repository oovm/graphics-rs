use palette::Srgba;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
};
mod circle;
mod color;
mod disk;
mod lines;
mod points;
mod shape;

pub use self::{
    circle::{CircleColor, CircleStyle, CircleWidth},
    color::RGBA,
    disk::{DiskColor, DiskSize, DiskStyle},
    lines::{LineColor, LineStyle, LineWidth},
    points::{PointColor, PointSize, PointStyle},
    shape::StyledShape,
};
