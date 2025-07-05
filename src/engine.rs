use std::path::{Path, PathBuf};

use crate::{color::{self, Color}, platform::{sdl::SDLPlatform, RenderEngine}};

pub struct Holly {
    engine: Box<dyn RenderEngine>,    
    background: Color,
}

impl Holly {
    pub fn new_sdl(title: impl Into<String>, width: usize, height: usize) -> anyhow::Result<Self> {
        Ok(Self {
            engine: Box::new(SDLPlatform::new(title, width, height)?),
            background: color::WHITE,
        })
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
    }

    pub fn register_font(&mut self, name: impl Into<String>, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.engine.register_font(name.into(), path.as_ref())
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        while self.engine.is_running() {
            self.engine.pre_render()?;

            self.engine.clear_screen(self.background)?;

            self.engine.render_rect(100, 100, 200, 200, Color::from_hex("#ff00ff")?)?;

            self.engine.render_text("hello, world".into(), 100, 100, 50, Color::from_hex("#000000")?, "default".into())?;
            self.engine.render_text("another line of text".into(), 100, 150, 50, Color::from_hex("#000000")?, "default".into())?;

            self.engine.post_render()?;
        }

        Ok(())
    }
}
