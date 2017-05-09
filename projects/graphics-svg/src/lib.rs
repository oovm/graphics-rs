use std::collections::BTreeMap;

pub struct SVG {
    element: &'static str,
    keys: BTreeMap<&'static str, String>,
    children: Vec<SVG>,
}

pub trait ToSVG {
    fn to_svg(&self) -> SVG;
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl ToSVG for Point {
    fn to_svg(&self) -> SVG {
        todo!()
    }
}
