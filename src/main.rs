use holly::{color, platform::{sdl::SDLPlatform, RenderEngine}};

fn main() -> anyhow::Result<()> {
    let mut engine = SDLPlatform::new("SDL Engine", 1280, 720)?;

    while engine.is_running() {
        engine.pre_render()?;

        engine.clear_screen(color::RED)?;

        engine.render_rect(100, 100, 100, 100, color::BLUE)?;
        engine.render_rect_outlined(100, 100, 100, 100, color::GREEN)?;

        engine.post_render()?;
    }

    Ok(())
}
