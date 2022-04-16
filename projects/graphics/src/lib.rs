mod shapes;

#[cfg(feature = "svg")]
pub use graphics_svg::SvgRenderer;
#[cfg(feature = "wolfram")]
pub use graphics_wolfram::WolframRenderer;
