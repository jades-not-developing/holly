use crate::color::Color;

pub mod sdl;

pub trait RenderEngine {
    fn new(title: impl Into<String>, width: usize, height: usize) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn is_running(&self) -> bool;

    fn clear_screen(&mut self, color: Color) -> anyhow::Result<()>;

    fn render_rect(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: Color,
    ) -> anyhow::Result<()>;

    fn render_rect_outlined(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: Color,
    ) -> anyhow::Result<()>;

    fn pre_render(&mut self) -> anyhow::Result<()>;
    fn post_render(&mut self) -> anyhow::Result<()>;
}
