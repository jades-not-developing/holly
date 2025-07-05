use holly::{platform::sdl::SDLError, Holly};

fn main() -> anyhow::Result<()> {
    let mut engine = Holly::new_sdl("SDL Test", 800, 800)?;

    engine.register_font("default", "font/ArmataHairline-J1Vx.otf")?;

    engine.start()
}
