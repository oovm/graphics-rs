use graphics_style::{StyleContext, StyleResolver};
use palette::named::BLUE;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_theme() {
    let my_theme = StyleContext { point_size: Some(2.0), ..Default::default() };
    let resolver = StyleResolver::with_theme_style(my_theme);
}
