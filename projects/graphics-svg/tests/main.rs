use graphics_core::{Graphics, Point};
use graphics_svg::SvgRenderer;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut graphics = Graphics::default();
    graphics.push(Point::new(0.0, 0.0));
    graphics.push(Point::new(0.0, 1.0));
    let mut renderer = SvgRenderer::default();
    let out = graphics.render_with(&mut renderer).unwrap();

    println!("{}", out);
}
