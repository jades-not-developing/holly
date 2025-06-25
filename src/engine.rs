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

    pub fn start(&mut self) -> anyhow::Result<()> {
        while self.engine.is_running() {
            self.engine.pre_render()?;

            self.engine.clear_screen(self.background)?;

            self.engine.post_render()?;
        }

        Ok(())
    }
}