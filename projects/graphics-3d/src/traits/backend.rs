use crate::{raw::*, Drawable, Graphics, GraphicsShape};
use graphics_style::{resolved, StyleResolver};
use resolved::CircleStyle;

#[allow(unused_variables)]
pub trait GraphicsBackend {
    type Output;
    type Error;

    fn get_output(&mut self, context: &Graphics) -> Result<Self::Output, Self::Error>;

    fn on_start(&mut self, context: &Graphics, state: &mut StyleResolver) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_finish(&mut self, context: &Graphics, state: &mut StyleResolver) -> Result<(), Self::Error> {
        Ok(())
    }

    fn draw(&mut self, context: &Graphics, state: &mut StyleResolver, drawable: &dyn Drawable) -> Result<(), Self::Error> {
        for i in drawable.changed_style() {
            state.set_local_style(i);
        }
        if drawable.skip() {
            return Ok(());
        }
        let shape = match drawable.get_shape() {
            Some(s) => s,
            None => return Ok(()),
        };
        match shape {
            GraphicsShape::Circle(c, s) => {
                let style = state.resolve_circle_style(s);
                self.draw_circle(context, &c, &style)
            }
        }
    }

    fn draw_circle(&mut self, context: &Graphics, shape: &Circle, style: &CircleStyle) -> Result<(), Self::Error>;
}
