Graphics Style
==============

The definition of all graphics style properties.

## Theme Style

If you want to make a theme, just define a new [`StyleContext`].

```rust
#[test]
fn test_theme() {
    let mut resolver = StyleResolver::default();
    let my_theme = StyleContext { point_size: Some(2.0), ..Default::default() };
    resolver.set_theme_style(my_theme);
}
```

## Custom Style

If you want to extend style directives, you just need to implement [`GraphicsStyle`].

