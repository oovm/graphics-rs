use graphics_style::{Color, PointColor, PointSize, StyleContext, StyleResolver};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_theme() {
    let mut my_theme = StyleContext::default();
    my_theme += PointSize { value: 1.0 };
    my_theme += PointColor { value: Color::AZURE };

    // let resolver = StyleResolver::with_theme_style(my_theme);
    let out = serde_json::to_string(&my_theme).unwrap();
    println!("{}", out);
    let resolver: StyleContext = serde_json::from_str(&out).unwrap();
    println!("{:#?}", resolver);
}
