#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::utils::render_high_quality as render;
use self::scenes::legacy_scene::legacy_scene_light as scene;
use std::env;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    info!("generating scene...");
    let (world, camera) = scene();
    render(world, camera, "legacy_scene_light.png", false)?;
    Ok(())
}
