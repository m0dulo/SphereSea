use super::{Renderer, ThreadedRenderer};
use crate::tracer::{BVHNode, Camera, World};
use std::sync::Arc;

pub fn render_to_file(renderer: impl Renderer, path: &'static str) -> Result<(), std::io::Error> {
    let start_time = time::get_time();
    let buf = renderer.render();
    let end_time = time::get_time();
    buf.save("output/".to_owned() + path)?;
    println!(
        "{} rendered in {}ms",
        path,
        (end_time - start_time).num_milliseconds()
    );
    Ok(())
}

pub fn render_high_quality(
    world: World,
    camera: Camera,
    path: &'static str,
    ambient_light: bool,
) -> Result<(), std::io::Error> {
    info!("constructing bvh node...");
    let world = World {
        hitables: vec![BVHNode::new(world)],
    };
    info!("rendering in progress...");
    render_to_file(
        ThreadedRenderer {
            world: Arc::new(world),
            camera: Arc::new(camera),
            size: (1600, 1600),
            anti_aliasing: 256,
            block_count: (16, 16),
            workers: 4,
            ambient_light,
        },
        path,
    )
}

pub fn render_preview(
    world: World,
    camera: Camera,
    path: &'static str,
    ambient_light: bool,
) -> Result<(), std::io::Error> {
    info!("constructing bvh node...");
    let world = World {
        hitables: vec![BVHNode::new(world)],
    };
    info!("rendering in progress...");
    render_to_file(
        ThreadedRenderer {
            world: Arc::new(world),
            camera: Arc::new(camera),
            size: (600, 600),
             anti_aliasing: 64,
            block_count: (3, 3),
            workers: 4,
            ambient_light,
        },
        path,
    )
}
