use sdl3::{event::Event, render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};

use crate::{color::Color, platform::RenderEngine};

#[derive(Debug, PartialEq)]
enum SDLRenderInstruction {
    SetDrawColor(Color),
    Clear,
    FillRect(sdl3::render::FRect),
    DrawRect(sdl3::render::FRect),
}

pub struct SDLPlatform {
    _sdl: Sdl,
    _video: VideoSubsystem,
    window: Window,
    events: EventPump,

    instruction_queue: Vec<SDLRenderInstruction>,

    running: bool,
}

impl SDLPlatform {
    fn get_canvas(&mut self) -> Canvas<Window> {
        self.window.clone().into_canvas()
    }
}

impl RenderEngine for SDLPlatform {
    fn new(title: impl Into<String>, width: usize, height: usize) -> anyhow::Result<Self> {
        let _sdl = sdl3::init()?;
        let _video = _sdl.video()?;

        let window = _video
            .window(title.into().as_str(), width as u32, height as u32)
            .position_centered()
            .build()?;

        let events = _sdl.event_pump()?;

        Ok(Self {
            _sdl,
            _video,
            window,
            events,

            instruction_queue: Vec::new(),

            running: true,
        })
    }
 
    fn pre_render(&mut self) -> anyhow::Result<()> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                _ => {},
            }
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        return self.running;
    }

    fn clear_screen(&mut self, color: Color) -> anyhow::Result<()> {
        self.instruction_queue.push(SDLRenderInstruction::SetDrawColor(color));
        self.instruction_queue.push(SDLRenderInstruction::Clear);

        Ok(())
    }

    fn render_rect(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: Color,
    ) -> anyhow::Result<()> {
        self.instruction_queue.push(SDLRenderInstruction::SetDrawColor(color));
        self.instruction_queue.push(SDLRenderInstruction::FillRect(sdl3::render::FRect::new(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
        )));

        Ok(())
    }

    
    fn render_rect_outlined(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: Color,
    ) -> anyhow::Result<()> {
        self.instruction_queue.push(SDLRenderInstruction::SetDrawColor(color));
        self.instruction_queue.push(SDLRenderInstruction::DrawRect(sdl3::render::FRect::new(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
        )));

        Ok(())
    }

    
    fn post_render(&mut self) -> anyhow::Result<()> {
        let mut canvas = self.get_canvas();

        for inst in &self.instruction_queue {
            match inst {
                SDLRenderInstruction::SetDrawColor(c) => canvas.set_draw_color(*c),
                SDLRenderInstruction::Clear => canvas.clear(),
                SDLRenderInstruction::FillRect(r) => canvas.fill_rect(*r)?,
                SDLRenderInstruction::DrawRect(r) => canvas.draw_rect(*r)?,
            }
        }

        canvas.present();
       Ok(()) 
    }
}
