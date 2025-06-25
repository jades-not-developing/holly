use holly::Holly;

fn main() -> anyhow::Result<()> {
    let mut engine = Holly::new_sdl("SDL Test", 800, 800)?;

    engine.start()
}