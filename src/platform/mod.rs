use std::path::Path;

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

    fn render_text(
        &mut self,
        text: String,
        x: isize,
        y: isize,
        font_size: usize,
        color: Color,
        font_name: String
    ) -> anyhow::Result<()>;

    fn register_font(&mut self, name: String, path: &Path) -> anyhow::Result<()>;

    fn pre_render(&mut self) -> anyhow::Result<()>;
    fn post_render(&mut self) -> anyhow::Result<()>;
}
