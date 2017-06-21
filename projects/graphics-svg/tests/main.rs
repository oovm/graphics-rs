use graphics_core::{Graphics, Point, Square};
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
    graphics.push(Square::new(0.0, 1.0, 5.0));

    let mut renderer = SvgRenderer::default();
    let out = graphics.render_with(&mut renderer).unwrap();
    std::fs::write("tests/show.svg", format!("{}", out)).unwrap();
}
